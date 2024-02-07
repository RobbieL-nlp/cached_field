# cached_field
 Rust proc macro to cache associate function result on strut field. An analogy to python's @cached_property class decorator. 

 ### Install
 ```
 cargo install cached_field
 ```

 ### Usage
 Attribute macro decorates on function implementations of struct;

 Function must receive a &self as its first args; Struct must have a corresponding optional field to store the computation result;  

 ### Params

 **field**: must be a string literal, to indicate the struct field for caching result, default to the ident of function it decorates on;

 **borrow**: bool, indicate whether the transformed function return a reference or copy of the stored value;
             if false, the return type should implement the Copy trait; Default true;

 It receive either positional arguments in order of (field, borrow) or named value arguments;

 # Example

 ```
 use cached_field::cached_field;
 struct SomeStruct{
     some_value: Option<u8>
 }

 impl SomeStruct{
     #[cached_field]
     fn some_value(&self) -> u8 {
         100
     }
     #[cached_field("some_value")]
     fn same_value(&self) -> u8 {
         101
     }
 }

 // s has to be mut 
 // since some_value requires a mutable reference to SomeStruct to mutate the some_value field inside;   
 let mut s = SomeStruct{some_value:None};

 assert_eq!(100, s.some_value());
 assert_eq!(100, s.same_value());

 ```
 ### trait implementation
 - the function defination in trait block should be the final form of the transformation. 
   i.e. &mut self for receiver parameter and &T ReturnType if borrow;
 - The impl function signature should take immutable form; i.e. &self and T without & (if trait signature has &T as ReturnType)
 - General Idea is that the impl function is only for computation purpose, 
   so it should not mutate the struct and it should return the original result;    
 ```
 
 use cached_field::cached_field;
 
 struct AnotherStruct{ 
     another_value: Option<Vec<String>>
 }
 
 trait Another {
     fn another(&mut self)->&Vec<String>;
 }
 
 impl Another for AnotherStruct {
     #[cached_field(field = "another_value", borrow = true)]
     fn another(&self)->Vec<String>{
         vec![1.to_string(),3.to_string(),5.to_string()]
     }
 }
 
 assert_eq!(vec![1.to_string(),3.to_string(),5.to_string()], *AnotherStruct{another_value:None}.another());

 ```
 
 # Feature
 By default, if borrow is true, the macro will check the computation function return type, if it found a &, it will panic;
 
 Modify the behavior when borrow is true and existing & reference found using these features:
 
 **carry**: keep the return type as is when transform.
 
 **prepend**: prepend & to original type, e.g. transform &T to &&T;
 
 *features are meant to be **multually exclusive**;  