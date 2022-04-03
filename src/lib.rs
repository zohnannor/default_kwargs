#![doc = include_str!("../README.md")]

extern crate proc_macro;
mod default_args;
mod keyword_args;
mod utils;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn default_args(item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as default_args::DefaultFn);
    quote! { #item }.into()
}

#[proc_macro]
pub fn keyword_args(item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as keyword_args::KeywordFn);
    quote! { #item }.into()
}
