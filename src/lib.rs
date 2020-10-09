//! Provides macros for fetching the package's compile date and time.
//!
//! # Example
//!
//! ```
//! use pkg_compile_time::*;
//! fn main(){
//!     println!("Compile date is: {}", pkg_compile_date!());
//!     println!("Compile time is: {}", pkg_compile_time!());
//! }
//! ```
//! The Output is:
//! ```
//! Compile date is: 2020-10-01
//! Compile time is: 19:58:49
//! ```
use proc_macro::TokenStream;
use quote::*;

/// Get the package's compile date.
#[proc_macro]
pub fn pkg_compile_date(_input: TokenStream) -> TokenStream {
    let date = time::OffsetDateTime::now_local().format("%Y-%m-%d");
    TokenStream::from(quote!(#date))
}

/// Get the package's compile time.
#[proc_macro]
pub fn pkg_compile_time(_input: TokenStream) -> TokenStream {
    let time = time::OffsetDateTime::now_local().format("%H:%M:%S");
    TokenStream::from(quote!(#time))
}
