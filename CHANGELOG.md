# Changelog

## v0.3.0

- `HxRequestGuardLayer` now redirects on failures instead of returning a 403. By
  default, it will redirect to "/", but you can specify a different route to
  redirect to with `HxRequestGuardLayer::new("/your-route-here")`.

## v0.2.0

- Added `HxRequestGuardLayer`, allowing you to protect an entire router from
  non-htmx requests.

## v0.1.0

- Initial release.
