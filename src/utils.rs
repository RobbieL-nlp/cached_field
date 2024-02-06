use proc_macro::TokenStream;
use syn::{
    parse::{ ParseStream, Parser}, parse_quote, punctuated::Punctuated, Error, FnArg, ReturnType, Token,
    Type, TypeReference,
};

pub fn split_comma<T>(args: TokenStream) -> Result<Punctuated<T, Token![,]>, Error>
where
    T: syn::parse::Parse,
{
    Punctuated::<T, Token![,]>::parse_terminated.parse(args)
}

pub fn split_comma_parse_stream<T>(args: ParseStream) -> Result<Punctuated<T, Token![,]>, Error>
where
    T: syn::parse::Parse,
{
    Punctuated::<T, Token![,]>::parse_terminated(args)
}

pub fn mut_receiver(
    mut inputs: Punctuated<FnArg, Token![,]>,
    check: bool,
) -> Punctuated<FnArg, Token![,]> {
    for input in inputs.iter_mut() {
        if let FnArg::Receiver(arg) = input {
            if check && arg.mutability.is_some() {
                panic!("compute function should take &self, not &mut self");
            }
            arg.mutability = parse_quote!(mut);
            if let Type::Reference(ty) = arg.ty.as_mut() {
                ty.mutability = parse_quote!(mut);
            }
        }
    }
    inputs
}

pub enum DuplAction {
    Throw,
    Prepend,
    Carry,
}

pub fn decor_output_amper(output: ReturnType, on_dupl: DuplAction) -> ReturnType {
    match &output {
        ReturnType::Type(tk, ty) => {
            if let Type::Reference(_) = **ty {
                match on_dupl {
                    DuplAction::Throw => panic!("function signature output already a reference!"),
                    DuplAction::Carry => return output,
                    _ => (),
                }
            }
            ReturnType::Type(
                *tk,
                Box::new(Type::Reference(TypeReference {
                    and_token: parse_quote!(&),
                    lifetime: None,
                    mutability: None,
                    elem: ty.clone(),
                })),
            )
        }

        _ => output,
    }
}
