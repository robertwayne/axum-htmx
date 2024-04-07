use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::{parse2, parse_quote, parse_str, ItemFn};

pub struct MacroInput {
    pub source_fn: ItemFn,
    pub layout_fn: String,
    pub fn_args: Vec<String>,
}

pub fn parse_macros_input(
    args: TokenStream,
    input: TokenStream,
) -> Result<MacroInput, TokenStream> {
    let mut args_iter = args.clone().into_iter().map(|arg| arg.to_string());

    // get layout_fn from args
    let layout_fn = match args_iter.next() {
        Some(arg) => arg,
        None => abort!(
            args,
            "boosted_by requires layout function (to produce non-boosted response) as an argument."
        ),
    };

    // arguments for callable function
    let fn_args = args_iter.collect::<Vec<String>>();

    // parse input as ItemFn
    let source_fn = match parse2::<ItemFn>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => return Err(error.to_compile_error()),
    };

    Ok(MacroInput {
        source_fn,
        layout_fn,
        fn_args,
    })
}

pub fn transform(input: MacroInput) -> ItemFn {
    let template_fn: ItemFn = parse_quote!(
        fn index(axum_htmx::HxBoosted(boosted): axum_htmx::HxBoosted) {
            if boosted {
                result_boosted
            } else {
                layout_fn(result_with_layout, fn_args)
            }
        }
    );

    transform_using_template(input, template_fn)
}

pub fn transform_async(input: MacroInput) -> ItemFn {
    let template_fn: ItemFn = parse_quote!(
        fn index(axum_htmx::HxBoosted(boosted): axum_htmx::HxBoosted) {
            if boosted {
                result_boosted
            } else {
                layout_fn(result_with_layout, fn_args).await
            }
        }
    );

    transform_using_template(input, template_fn)
}

pub fn transform_using_template(input: MacroInput, template_fn: ItemFn) -> ItemFn {
    let mut source_fn = input.source_fn.clone();

    // add HxBoosted input to source_fn
    let hx_boosted_input = template_fn.sig.inputs.first().unwrap().clone();
    source_fn.sig.inputs.push(hx_boosted_input);

    // pop the last statement and wrap it with if-else
    let source_stmt = source_fn.block.stmts.pop().unwrap();
    let source_stmt = quote!(#source_stmt).to_string();

    let new_fn_str = quote!(#template_fn)
        .to_string()
        .replace("layout_fn", input.layout_fn.as_str())
        .replace("result_boosted", source_stmt.as_str())
        .replace("result_with_layout", source_stmt.as_str());

    // add layout_args
    let layout_args = input.fn_args.join("");
    let new_fn_str = new_fn_str.replace(", fn_args", layout_args.as_str());

    // parse new_fn_str as ItemFn
    let new_fn: ItemFn = parse_str(new_fn_str.as_str()).unwrap();

    // push the new statement to source_fn
    let new_fn_stmt = new_fn.block.stmts.first().unwrap().clone();
    source_fn.block.stmts.push(new_fn_stmt);

    source_fn.to_owned()
}
