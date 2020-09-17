use proc_macro::TokenStream;
use quote::*;

#[proc_macro]
pub fn pkg_compile_date(_input: TokenStream) -> TokenStream {
    TokenStream::from(quote!(&chrono::Local::now().format("%Y-%m-%d").to_string()))
}

#[proc_macro]
pub fn pkg_compile_time(_input: TokenStream) -> TokenStream {
    TokenStream::from(quote!(&chrono::Local::now().format("%H:%M:%S").to_string()))
}
