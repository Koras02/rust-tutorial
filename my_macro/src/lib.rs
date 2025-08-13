use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyTrait)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = quote! {
        impl MyTrait for #name {
            fn hello() {
                println!("Hello from MyTrait for {}", stringify!(#name));
            }
        }
    };
    gen.into()
}