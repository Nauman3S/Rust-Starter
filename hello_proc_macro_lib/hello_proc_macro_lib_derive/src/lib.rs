extern crate proc_macro;
extern crate quote;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_proc_macro_lib_derive(input:TokenStream)->TokenStream{
    let ast=syn::parse(input).unwrap();


    impl_hello_macro(&ast)


}

fn impl_hello_macro(ast:&syn::DeriveInput)->TokenStream{
    let name=&ast.ident;
    let gen=quote!{
        impl HelloMacro for #name{
            fn hello_macro(){
                println!("Hello Macro! my name is {}!"), stringify!(#name)
            }
        }
    };
    gen.into()
}