//! HTTP headers used by htmx.

use http::HeaderName;

/// Indicates that the request is via an element using `hx-boost` attribute.
///
/// See <https://htmx.org/attributes/hx-boost/> for more information.
pub const HX_BOOSTED: HeaderName = HeaderName::from_static("hx-boosted");

/// The current URL of the browser.
pub const HX_CURRENT_URL: HeaderName = HeaderName::from_static("hx-current-url");

/// `true` if the request is for history restoration after a miss in the local
/// history cache.
pub const HX_HISTORY_RESTORE_REQUEST: HeaderName =
    HeaderName::from_static("hx-history-restore-request");

/// The user response to an `hx-prompt`
///
/// See <https://htmx.org/attributes/hx-prompt/> for more information.
pub const HX_PROMPT: HeaderName = HeaderName::from_static("hx-prompt");

/// Always `true`.
pub const HX_REQUEST: HeaderName = HeaderName::from_static("hx-request");

/// The `id` of the target element, if it exists.
pub const HX_TARGET: HeaderName = HeaderName::from_static("hx-target");

/// The `name` of the triggered element, if it exists.
pub const HX_TRIGGER_NAME: HeaderName = HeaderName::from_static("hx-trigger-name");

/// Allows you to do a client-side redirect that does not do a full page reload.
pub const HX_LOCATION: HeaderName = HeaderName::from_static("hx-location");

/// Pushes a new URL onto the history stack.
pub const HX_PUSH_URL: HeaderName = HeaderName::from_static("hx-push-url");

/// Can be used to do a client-side redirect to a new location.
pub const HX_REDIRECT: HeaderName = HeaderName::from_static("hx-redirect");

/// If set to `true`, the client will do a full refresh on the page.
pub const HX_REFRESH: HeaderName = HeaderName::from_static("hx-refresh");

/// Replaces the currelt URL in the location bar.
pub const HX_REPLACE_URL: HeaderName = HeaderName::from_static("hx-replace-url");

/// Allows you to specify how the response value will be swapped.
///
/// See <https://htmx.org/attributes/hx-swap/> for more information.
pub const HX_RESWAP: HeaderName = HeaderName::from_static("hx-reswap");

/// A CSS selector that update the target of the content update to a different
/// element on the page.
pub const HX_RETARGET: HeaderName = HeaderName::from_static("hx-retarget");

/// A CSS selector that allows you to choose which part of the response is used
/// to be swapped in. Overrides an existing `hx-select` on the triggering
/// element
pub const HX_RESELECT: HeaderName = HeaderName::from_static("hx-reselect");

/// Can be set as a request or response header.
///
/// In a request, it contains the `id` of the element that triggered the
/// request.
///
/// In a response, it can be used to trigger client-side events.
///
/// See <https://htmx.org/headers/hx-trigger/> for more information.
pub const HX_TRIGGER: HeaderName = HeaderName::from_static("hx-trigger");

/// Allows you to trigger client-side events.
///
/// See <https://htmx.org/headers/hx-trigger/> for more information.
pub const HX_TRIGGER_AFTER_SETTLE: HeaderName = HeaderName::from_static("hx-trigger-after-settle");

/// Allows you to trigger client-side events.
///
/// See <https://htmx.org/headers/hx-trigger/> for more information.
pub const HX_TRIGGER_AFTER_SWAP: HeaderName = HeaderName::from_static("hx-trigger-after-swap");
