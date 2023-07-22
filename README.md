# axum-htmx

`axum-htmx` is a small extension library providing extractors for the various
 [htmx](https://htmx.org/) headers within [axum](https://github.com/tokio-rs/axum).

 __This crate is current a work-in-progress. There are many missing header implementations, and it builds against the upcoming, unreleased branch for `axum`.__

## Usage

```toml
axum-htmx = { git = "https://github.com/robertwayne/axum-htmx", branch = "main" }
```

## Examples

In this example, we'll look for the `HX-Boosted` header, which is set when applying the [hx-boost](https://htmx.org/attributes/hx-boost/) attribute to an element. In our case, we'll use it to determine what kind of response we send.

When is this useful? When using a templating engine, like [minijinja](https://github.com/mitsuhiko/minijinja), it is common to extend different templates from a `_base.html` template. However, HTMX works by sending partial responses, so extending our `_base.html` would result in lots of extra data being sent over the wire.

If we wanted to swap between pages, we would need to support both full template responses and partial responses _(as the page can be accessed directly or through a boosted anchor)_, so we look for the `HX-Boosted` header and extend from a `_partial.html` template instead.

```rs
async fn get_index(HxBoosted(boosted): HxBoosted) -> impl IntoResponse {
    if boosted {
        // Send a template extending from _partial.html
    } else {
        // Send a template extending from _base.html
    }
}
```

## License

`axum-htmx` is dual-licensed under either

- **[MIT License](/docs/LICENSE-MIT)**
- **[Apache License, Version 2.0](/docs/LICENSE-APACHE)**

at your option.
