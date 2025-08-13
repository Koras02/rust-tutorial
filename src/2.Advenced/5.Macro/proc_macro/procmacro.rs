extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(MyTrait)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    let _ast = syn::parse_macro_input!(input);

    let gen = quote! {
        impl MyTrait for MyStruct {
            fn hello() {
                println!("Hello from MyTrait!");
            }
        }
    };
    gen.into()
}
