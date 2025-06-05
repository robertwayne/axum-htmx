# Changelog

## v0.8.0

- Change `HxLocation`, `HxPushUrl`, `HxRedirect`, and `HxReplaceUrl` to take `String` instead of `Uri` for better flexibility and ease-of-use. _([@skwee357](https://github.com/skwee357))_
    - `HxCurrentUrl` remains unchanged in order to align with axum.
- Changed how `LocationOptions` is handled internally with regard to non-exhaustiveness,
allowing external crates to use the functional record update syntax.

## v0.7.0

- Support axum v0.8. _([@kakalos12](https://github.com/kakalos12))_

## v0.6.0

- Added support for Vary headers in responses via the `VaryHxRequest`, `VaryHxTarget`, `VaryHxTrigger`, and `VaryHxTriggerName` responders. _([@imbolc](https://github.com/imbolc))_
- Header names/values are now typed as `HeaderName` and `HeaderValue` instead of `&str`. _([@imbolc](https://github.com/imbolc))_
- `HxError` now implements source on `error::Error`. _([@imbolc](https://github.com/imbolc))_
- Added `AutoVaryLayer` middleware to automatically manage `Vary` headers when using corresponding extractors. The middleware is behind the `auto-vary` feature. [See this section of the README for more details.](https://github.com/robertwayne/axum-htmx?tab=readme-ov-file#vary-responders). _([@imbolc](https://github.com/imbolc))_

## v0.5.0

There are some several breaking changes in this release. Big thanks to [@ItsEthra](https://github.com/ItsEthra) for their work in several PRs!

- All responders now take an `HxEvent` instead of a `String | HxEvent`. When the `serde` flag is enabled, it will expose additional data fields.
- `HxResponseTrigger` is now a simple struct containing an `TriggerMode` and a `Vec<HxEvent>`. There are several methods to make constructing these easier: `HxResponseTrigger::normal`, `HxResponseTrigger::after_settle`, and `HxResponseTrigger::after_swap`.
- The `HxCurrentUrl` extractor now returns an `Option<axum::http::Uri>` instead of a `String`. If the Uri cannot be parsed, it will return `None`.
- All Uri-related responders now impl `TryFrom<&str>`.
- `HxError::Serialization` has been renamed to `HxError::Json`.
- The `HxResponseTrigger*` header will not be added to the response if the event list is empty.
- Added feature flag badges and made additional updates to the docs.rs pages.
- Reduced dependency count / compile time by swapping `axum` out for the `axum-core`, `async-trait`, and `http` crates.

## v0.4.0

- Added support for all [htmx response headers](https://htmx.org/reference/#response_headers) via a type implementing `IntoResponseParts`. These "responders" allow you to simply and safely apply the HX-* headers to any of your responses. Thanks to [@pfz4](https://github.com/pfz4) for the implementation work! ([#5](https://github.com/robertwayne/axum-htmx/pull/5))

## v0.3.1

- Rebuild docs with features enabled so `HxRequestGuardLayer` is visible on docs.rs.

## v0.3.0

- `HxRequestGuardLayer` now redirects on failures instead of returning a 403\. By default, it will redirect to "/", but you can specify a different route to redirect to with `HxRequestGuardLayer::new("/your-route-here")`.

## v0.2.0

- Added `HxRequestGuardLayer`, allowing you to protect an entire router from non-htmx requests.

## v0.1.0

- Initial release.
