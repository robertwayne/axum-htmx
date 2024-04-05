use proc_macro2::TokenStream;
use quote::quote;

#[cfg(test)]
mod tests;

mod boosted_by;

pub fn macros(args: TokenStream, input: TokenStream) -> TokenStream {
    match boosted_by::parse_macros_input(args, input) {
        Ok(macros_input) => {
            let new_item_fn = boosted_by::transform(macros_input);
            quote!(#new_item_fn)
        }
        Err(error) => error,
    }
}

pub fn macros_async(args: TokenStream, input: TokenStream) -> TokenStream {
    match boosted_by::parse_macros_input(args, input) {
        Ok(macros_input) => {
            let new_item_fn = boosted_by::transform_async(macros_input);
            quote!(#new_item_fn)
        }
        Err(error) => error,
    }
}
