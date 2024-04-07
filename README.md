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

`axum-htmx` is a small extension library providing extractors, responders, and
 request guards for [htmx](https://htmx.org/) headers within
 [axum](https://github.com/tokio-rs/axum).

## Table of Contents

- [axum-htmx](#axum-htmx)
  - [Table of Contents](#table-of-contents)
  - [Getting Started](#getting-started)
  - [Extractors](#extractors)
  - [Responders](#responders)
  - [Request Guards](#request-guards)
  - [Macroses](#macroses)
  - [Examples](#examples)
    - [Example: Extractors](#example-extractors)
    - [Example: Responders](#example-responders)
    - [Example: Router Guard](#example-router-guard)
  - [Feature Flags](#feature-flags)
  - [Contributing](#contributing)
  - [License](#license)

## Getting Started

Run `cargo add axum-htmx` to add the library to your project.

## Extractors

All of the [htmx request headers](https://htmx.org/reference/#request_headers)
have a supported extractor. Extractors are infallible, meaning they will always
succeed and never return an error. In the case where a header is not present,
the extractor will return `None` or `false` dependant on the expected return
type.

| Header                       | Extractor                 | Value                     |
|------------------------------|---------------------------|---------------------------|
| `HX-Boosted`                 | `HxBoosted`               | `bool`                    |
| `HX-Current-URL`             | `HxCurrentUrl`            | `Option<axum::http::Uri>` |
| `HX-History-Restore-Request` | `HxHistoryRestoreRequest` | `bool`                    |
| `HX-Prompt`                  | `HxPrompt`                | `Option<String>`          |
| `HX-Request`                 | `HxRequest`               | `bool`                    |
| `HX-Target`                  | `HxTarget`                | `Option<String>`          |
| `HX-Trigger-Name`            | `HxTriggerName`           | `Option<String>`          |
| `HX-Trigger`                 | `HxTrigger`               | `Option<String>`          |

## Responders

All of the [htmx response headers](https://htmx.org/reference/#response_headers)
have a supported responder. A responder is a basic type that implements
`IntoResponseParts`, allowing you to simply and safely apply the HX-* headers to
any of your responses.

| Header                    | Responder           | Value                               |
|---------------------------|---------------------|-------------------------------------|
| `HX-Location`             | `HxLocation`        | `axum::http::Uri`                   |
| `HX-Push-Url`             | `HxPushUrl`         | `axum::http::Uri`                   |
| `HX-Redirect`             | `HxRedirect`        | `axum::http::Uri`                   |
| `HX-Refresh`              | `HxRefresh`         | `bool`                              |
| `HX-Replace-Url`          | `HxReplaceUrl`      | `axum::http::Uri`                   |
| `HX-Reswap`               | `HxReswap`          | `axum_htmx::responders::SwapOption` |
| `HX-Retarget`             | `HxRetarget`        | `String`                            |
| `HX-Reselect`             | `HxReselect`        | `String`                            |
| `HX-Trigger`              | `HxResponseTrigger` | `axum_htmx::serde::HxEvent`         |
| `HX-Trigger-After-Settle` | `HxResponseTrigger` | `axum_htmx::serde::HxEvent`         |
| `HX-Trigger-After-Swap`   | `HxResponseTrigger` | `axum_htmx::serde::HxEvent`         |

## Request Guards

__Requires features `guards`.__

In addition to the extractors, there is also a route-wide layer request guard
for the `HX-Request` header. This will redirect any requests without the header
to "/" by default.

_It should be noted that this is NOT a replacement for an auth guard. A user can
trivially set the `HX-Request` header themselves. This is merely a convenience
for preventing users from receiving partial responses without context. If you
need to secure an endpoint you should be using a proper auth system._

## Macroses

__Requires features `derive`.__

In addition to the HxBoosted extractor, the library provides macroses `hx_boosted_by` and it's async version `hx_boosted_by_async` for managing the response based on the presence of the `HX-Boosted` header.

The macro input should have a `layout_fn`, and can have arguments passed from annotated function into `layout_fn`. The macro will call the `layout_fn` if the `HX-Boosted` header is not present, otherwise it will return the response directly.

`#[hx_boosted_by(layout_fn [, arg1, agr2, ...])]`

If `layout_fn` is an async function, use `hx_boosted_by_async` instead.

## Examples

### Example: Extractors

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

```rust
use axum::response::IntoResponse;
use axum_htmx::HxBoosted;

async fn get_index(HxBoosted(boosted): HxBoosted) -> impl IntoResponse {
    if boosted {
        // Send a template extending from _partial.html
    } else {
        // Send a template extending from _base.html
    }
}
```

### Example: Responders

We can trigger any event being listened to by the DOM using an [htmx
trigger](https://htmx.org/attributes/hx-trigger/) header.

```rust
use axum_htmx::HxResponseTrigger;

// When we load our page, we will trigger any event listeners for "my-event.
async fn index() -> (HxResponseTrigger, &'static str) {
    // Note: As HxResponseTrigger only implements `IntoResponseParts`, we must
    // return our trigger first here.
    (
        HxResponseTrigger::normal(["my-event", "second-event"]),
        "Hello, world!",
    )
}
```

`htmx` also allows arbitrary data to be sent along with the event, which we can
use via the `serde` feature flag and the `HxEvent` type.

```rust
use serde_json::json;

// Note that we are using `HxResponseTrigger` from the `axum_htmx::serde` module
// instead of the root module.
use axum_htmx::{HxEvent, HxResponseTrigger};

async fn index() -> (HxResponseTrigger, &'static str) {
    let event = HxEvent::new_with_data(
        "my-event",
        // May be any object that implements `serde::Serialize`
        json!({"level": "info", "message": {
            "title": "Hello, world!",
            "body": "This is a test message.",
        }}),
    )
    .unwrap();

    // Note: As HxResponseTrigger only implements `IntoResponseParts`, we must
    // return our trigger first here.
    (HxResponseTrigger::normal([event]), "Hello, world!")
}
```

### Example: Router Guard

```rust
use axum::Router;
use axum_htmx::HxRequestGuardLayer;

fn router_one() -> Router {
    Router::new()
        // Redirects to "/" if the HX-Request header is not present
        .layer(HxRequestGuardLayer::default())
}

fn router_two() -> Router {
    Router::new()
        .layer(HxRequestGuardLayer::new("/redirect-to-this-route"))
}
```

### Example: Macros

```rust
use axum::extract::Path;
use axum::response::Html;
use axum_htmx::hx_boosted_by;

#[hx_boosted_by(with_layout, page_title)]
async fn get_hello(Path(name): Path<String>) -> Html<String> {
    let page_title = "Hello Page";
    Html(format!("Hello, {}!", name))
}

#[hx_boosted_by(with_layout, page_title)]
async fn get_bye(Path(name): Path<String>) -> Html<String> {
    let page_title = "Bye Page";
    Html(format!("Bye, {}!", name))
}

fn with_layout(Html(partial): Html<String>, page_title: &str) -> Html<String> {
    Html(format!("<html><head><title>{}</title></head><body>{}</body></html>", page_title, partial))
}
```

## Feature Flags

<!-- markdownlint-disable -->
| Flag     | Default  | Description                                                | Dependencies                                |
|----------|----------|------------------------------------------------------------|---------------------------------------------|
| `guards` | Disabled | Adds request guard layers.                                 | `tower`, `futures-core`, `pin-project-lite` |
| `serde`  | Disabled | Adds serde support for the `HxEvent` and `LocationOptions` | `serde`, `serde_json`                       |
| `derive` | Disabled | Adds the `hx_boosted_by` and `hx_boosted_by_async` macros. | `proc-macro-error`, `proc-macro2`, `quote`, `syn`     |
<!-- markdownlint-enable -->

## Contributing

Contributions are always welcome! If you have an idea for a feature or find a
bug, let me know. PR's are appreciated, but if it's not a small change, please
open an issue first so we're all on the same page!

## License

`axum-htmx` is dual-licensed under either

- **[MIT License](/LICENSE-MIT)**
- **[Apache License, Version 2.0](/LICENSE-APACHE)**

at your option.
