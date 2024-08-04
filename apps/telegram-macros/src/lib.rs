extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(TelegramRequest)]
pub fn derive_telegram_request(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_telegram_request(&ast)
}

fn impl_telegram_request(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl TelegramRequest for #name {
            fn extra(&self) -> String {
                self.extra.clone()
            }
        }
    };

    gen.into()
}
