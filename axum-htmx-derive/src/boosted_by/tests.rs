#![cfg(test)]

use proc_macro2::TokenStream;
use quote::quote;

use super::macros;

#[test]
fn boosted_by() {
    let before = quote! {
        async fn index(Path(user_id): Path<u32>) -> Html<String> {
            let ctx = HomeTemplate {
                locale: "en".to_string(),
            };

            Html(ctx.render_once().unwrap_or(String::new()))
        }
    };
    let expected = quote! {
        async fn index(axum_htmx::HxBoosted(boosted): axum_htmx::HxBoosted, Path(user_id): Path<u32>) -> Html<String> {
            let ctx = HomeTemplate {
                locale: "en".to_string(),
            };

            if boosted {
                Html(ctx.render_once().unwrap_or(String::new()))
            } else {
                with_layout(Html(ctx.render_once().unwrap_or(String::new())), state1, state2)
            }
        }
    };

    let after = macros(quote! {with_layout, state1, state2}, before);

    assert_tokens_eq(&expected, &after);
}

fn assert_tokens_eq(expected: &TokenStream, actual: &TokenStream) {
    let expected = expected.to_string();
    let actual = actual.to_string();

    if expected != actual {
        println!(
            "{}",
            colored_diff::PrettyDifference {
                expected: &expected,
                actual: &actual,
            }
        );

        panic!("expected != actual");
    }
}
