/// Indicates that the request is via an element using `hx-boost` attribute.
///
/// See <https://htmx.org/attributes/hx-boost/> for more information.
pub const HX_BOOSTED: &str = "HX-Boosted";

/// The current URL of the browser.
pub const HX_CURRENT_URL: &str = "HX-Current-Url";

/// `true` if the request is for history restoration after a miss in the local
/// history cache.
pub const HX_HISTORY_RESTORE_REQUEST: &str = "HX-History-Restore-Request";

/// The user response to an `hx-prompt`
///
/// See <https://htmx.org/attributes/hx-prompt/> for more information.
pub const HX_PROMPT: &str = "HX-Prompt";

/// Always `true`.
pub const HX_REQUEST: &str = "HX-Request";

/// The `id` of the target element, if it exists.
pub const HX_TARGET: &str = "HX-Target";

/// The `name` of the triggered element, if it exists.
pub const HX_TRIGGER_NAME: &str = "HX-Trigger-Name";

/// Allows you to do a client-side redirect that does not do a full page reload.
pub const HX_LOCATION: &str = "HX-Location";

/// Pushes a new URL onto the history stack.
pub const HX_PUSH_URL: &str = "HX-Push-Url";

/// Can be used to do a client-side redirect to a new location.
pub const HX_REDIRECT: &str = "HX-Redirect";

/// If set to `true`, the client will do a full refresh on the page.
pub const HX_REFRESH: &str = "HX-Refresh";

/// Replaces the currelt URL in the location bar.
pub const HX_REPLACE_URL: &str = "HX-Replace-Url";

/// Allows you to specify how the response value will be swapped.
///
/// See <https://htmx.org/attributes/hx-swap/> for more information.
pub const HX_RESWAP: &str = "HX-Reswap";

/// A CSS selector that update the target of the content update to a different
/// element on the page.
pub const HX_RETARGET: &str = "HX-Retarget";

/// A CSS selector that allows you to choose which part of the response is used
/// to be swapped in. Overrides an existing `hx-select` on the triggering
/// element
pub const HX_RESELECT: &str = "HX-Reselect";

/// Can be set as a request or response header.
///
/// In a request, it contains the `id of the element that triggered the request.
///
/// In a response, it can be used to trigger client-side events.
///
/// See <https://htmx.org/headers/hx-trigger/> for more information.
pub const HX_TRIGGER: &str = "HX-Trigger";

/// Allows you to trigger client-side events.
///
/// See <https://htmx.org/headers/hx-trigger/> for more information.
pub const HX_TRIGGER_AFTER_SETTLE: &str = "HX-Trigger-After-Settle";

/// Allows you to trigger client-side events.
///
/// See <https://htmx.org/headers/hx-trigger/> for more information.
pub const HX_TRIGGER_AFTER_SWAP: &str = "HX-Trigger-After-Swap";
