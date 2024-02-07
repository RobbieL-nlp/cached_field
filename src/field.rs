use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::Parse, parse_macro_input, parse_quote, Expr, ExprClosure, Ident, ItemFn, Lit,
    MetaNameValue, Signature,
};

use crate::utils::{decor_output_amper, mut_receiver, split_comma_parse_stream, DUPL_ACTION};

struct CacheArgs {
    field: Option<String>,
    borrow: bool,
    thread_safe: bool,
    //state
}

impl CacheArgs {
    fn field_count() -> u8 {
        3
    }

    fn new_default() -> Self {
        Self {
            field: None,
            borrow: false,
            thread_safe: false,
        }
    }
}

impl Parse for CacheArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(attrs) = Self::parse_list(input) {
            return Ok(attrs);
        }

        if let Ok(attrs) = Self::parse_namevalue(input) {
            return Ok(attrs);
        }

        Ok(Self::new_default())
    }
}
impl CacheArgs {
    fn parse_list(input: syn::parse::ParseStream) -> Result<CacheArgs, syn::Error> {
        let mut args = CacheArgs::new_default();

        split_comma_parse_stream::<Lit>(input)?
            .iter()
            .take(CacheArgs::field_count() as usize)
            .enumerate()
            .for_each(|(i, arg)| match i {
                0 => {
                    if let Lit::Str(s) = arg {
                        args.field = Some(s.value())
                    }
                }
                1 => {
                    if let Lit::Bool(s) = arg {
                        args.borrow = s.value()
                    }
                }
                2 => {
                    if let Lit::Bool(s) = arg {
                        args.thread_safe = s.value()
                    }
                }
                _ => (),
            });

        Ok(args)
    }

    fn parse_namevalue(input: syn::parse::ParseStream) -> Result<CacheArgs, syn::Error> {
        let mut args = CacheArgs::new_default();

        split_comma_parse_stream::<MetaNameValue>(input)?
            .iter()
            .for_each(|kv| {
                if let Expr::Lit(value) = &kv.value {
                    if kv.path.is_ident("field") {
                        if let Lit::Str(lit) = &value.lit {
                            args.field = Some(lit.value());
                        }
                    } else if kv.path.is_ident("thread_safe") {
                        if let Lit::Bool(lit) = &value.lit {
                            args.thread_safe = lit.value();
                        }
                    } else if kv.path.is_ident("borrow") {
                        if let Lit::Bool(lit) = &value.lit {
                            args.borrow = lit.value();
                        }
                    }
                }
            });

        Ok(args)
    }
}

fn gen_compute_ident(func: &ItemFn) -> Ident {
    let prefix = "_compute_fn_for_";
    format_ident!("{}{}", prefix, func.sig.ident)
}

fn gen_compute_closure(func: ItemFn) -> ExprClosure {
    let block = func.block;
    parse_quote!(
        || #block
    )
}

fn gen_compute_fn(func: ItemFn) -> ItemFn {
    // remember to filter out attribute
    todo!()
}

fn gen_signature(mut sig: Signature, borrow: bool) -> Signature {
    sig.inputs = mut_receiver(sig.inputs, true);

    if borrow {
        sig.output = decor_output_amper(sig.output, DUPL_ACTION);
    }

    sig
}

pub fn cached_field_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    let cache_args = parse_macro_input!(args as CacheArgs);
    let function = parse_macro_input!(item as ItemFn);
    let cache_field = format_ident!(
        "{}",
        cache_args.field.unwrap_or(function.sig.ident.to_string())
    );

    let vis = function.vis.clone();
    let signature = gen_signature(function.sig.clone(), cache_args.borrow);
    let closure_ident = gen_compute_ident(&function);
    let closure = gen_compute_closure(function);

    let ret_suffix;
    if cache_args.borrow {
        ret_suffix = quote!(.as_ref().unwrap());
    } else {
        ret_suffix = quote!(.unwrap());
    }

    let func = quote!(
        #vis #signature {
            let #closure_ident = #closure;
            match self.#cache_field {
                Some(_)=>self.#cache_field #ret_suffix,
                None => {
                    self.#cache_field = Some(#closure_ident());
                    self.#cache_field #ret_suffix
                }
            }
        }
    );

    func.into()
}
