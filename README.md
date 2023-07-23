# axum-htmx

<!-- markdownlint-disable -->
<div align="right">
<a href="https://crates.io/crates/axum-htmx">
    <img src="https://img.shields.io/crates/v/axum-htmx?style=flat-square" alt="crates.io badge">
</a>
<a href="https://docs.rs/axum-htmx/latest/">
    <img src="https://img.shields.io/docsrs/axum-htmx?style=flat-square" alt="docs.rs badge">
</a>
</div>
<br>
<!-- markdownlint-enable -->

`axum-htmx` is a small extension library providing extractors for the various
 [htmx](https://htmx.org/) headers within
 [axum](https://github.com/tokio-rs/axum). Additionally, the library exports
 const values for all of the htmx headers, so there's no need to mess with
 strings in your handlers.

## Getting Started

Simply run `cargo add axum-htmx` to add the library to your project.

If you are using the unreleased branch of `axum` from GitHub, you can build
against the `main` version of `axum-htmx` by adding the following to your
`Cargo.toml`:

```toml
[dependencies]
axum-htmx = { git = "https://github.com/robertwayne/axum-htmx" }
```

## Extractors

All of the [htmx request headers](https://htmx.org/reference/#request_headers)
have a supported extractor. Additionally, all extractors are infallible, meaning
they will always succeed and never return an error. If the header is not
present, the extractor will return `None` or `false` in most cases.

| Header | Extractor | Value |
| --- | --- | --- |
| `HX-Boosted` | `HxBoosted` | `bool` |
| `HX-Current-URL` | `HxCurrentUrl` | `Option<String>` |
| `HX-History-Restore-Request` | `HxHistoryRestoreRequest` | `bool` |
| `HX-Prompt` | `HxPrompt` | `Option<String>` |
| `HX-Request` | `HxRequest` | `bool` |
| `HX-Target` | `HxTarget` | `Option<String>` |
| `HX-Trigger-Name` | `HxTriggerName` | `Option<String>` |
| `HX-Trigger` | `HxTrigger` | `Option<String>` |

## Example Usage

In this example, we'll look for the `HX-Boosted` header, which is set when
applying the [hx-boost](https://htmx.org/attributes/hx-boost/) attribute to an
element. In our case, we'll use it to determine what kind of response we send.

When is this useful? When using a templating engine, like
[minijinja](https://github.com/mitsuhiko/minijinja), it is common to extend
different templates from a `_base.html` template. However, htmx works by sending
partial responses, so extending our `_base.html` would result in lots of extra
data being sent over the wire.

If we wanted to swap between pages, we would need to support both full template
responses and partial responses _(as the page can be accessed directly or
through a boosted anchor)_, so we look for the `HX-Boosted` header and extend
from a `_partial.html` template instead.

```rs
async fn get_index(HxBoosted(boosted): HxBoosted) -> impl IntoResponse {
    if boosted {
        // Send a template extending from _partial.html
    } else {
        // Send a template extending from _base.html
    }
}
```

You can also take advantage of const header values:

```rs
let mut headers = HeaderMap::new();
headers.insert(HX_REDIRECT, HeaderValue::from_static("/some/other/page"));
```

## License

`axum-htmx` is dual-licensed under either

- **[MIT License](/docs/LICENSE-MIT)**
- **[Apache License, Version 2.0](/docs/LICENSE-APACHE)**

at your option.
