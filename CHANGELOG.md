# Changelog

## v0.4.0

- Added support for all [htmx response
headers](https://htmx.org/reference/#response_headers) via a type implementing
`IntoResponseParts`. These "responders" allow you to simply and safely apply the
HX-* headers to any of your responses. Thanks to
[@pfz4](https://github.com/pfz4) for the implementation work!
([#5](https://github.com/robertwayne/axum-htmx/pull/5))

## v0.3.1

- Rebuild docs with features enabled so `HxRequestGuardLayer` is visible on
  docs.rs.

## v0.3.0

- `HxRequestGuardLayer` now redirects on failures instead of returning a 403. By
  default, it will redirect to "/", but you can specify a different route to
  redirect to with `HxRequestGuardLayer::new("/your-route-here")`.

## v0.2.0

- Added `HxRequestGuardLayer`, allowing you to protect an entire router from
  non-htmx requests.

## v0.1.0

- Initial release.
