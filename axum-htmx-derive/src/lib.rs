#![doc = include_str!("../README.md")]
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod boosted_by;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn hx_boosted_by(args: TokenStream, input: TokenStream) -> TokenStream {
    boosted_by::macros(args.into(), input.into()).into()
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn hx_boosted_by_async(args: TokenStream, input: TokenStream) -> TokenStream {
    boosted_by::macros_async(args.into(), input.into()).into()
}
