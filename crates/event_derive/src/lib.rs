use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};
//to be finished later

#[proc_macro_derive(Event)]
pub fn event_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input as DeriveInput);
    //the Event trait needs to be in the prelude along with this macro!!
    let ident = ident.to_string();
    let result = quote! {
        impl Event for #ident {
            fn id() -> &'static str {
                #ident
            }
        }
    };
    TokenStream::from(result)
}
