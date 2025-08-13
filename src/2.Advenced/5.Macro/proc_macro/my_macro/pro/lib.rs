extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput}; // <-- DeriveInput도 필요해!

#[proc_macro_derive(MyTrait)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    // 💥 이 부분을 수정해야 해! input 변수가 겹치니까 다른 이름으로 바꿔주거나,
    //    바로 ast로 받는 게 편해.
    let ast = parse_macro_input!(input as DeriveInput); // <-- 이렇게 수정!
    let name = &ast.ident; // <-- ast에서 ident를 가져오게 바꿔야 해

    let pro = quote! {
        impl MyTrait for #name {
            fn hello() {
                println!("Hello from MyTrait for {}", stringify!(#name) );
            }
        }
    };
    pro.into()
}