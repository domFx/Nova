use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};


#[proc_macro_derive(Component)]
pub fn component_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let gen = quote! {
        impl Component for #name { 
            fn get_size(&self) -> usize {
                72
            }
        }
    };
    gen.into()
}