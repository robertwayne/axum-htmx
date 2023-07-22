/// Represents all of the headers that can be sent in a response to the client.
///
/// See <https://htmx.org/reference/#response_headers> for more information.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HtmxResponseHeader {
    /// Allows you to do a client-side redirect that does not do a full page
    /// reload.
    Location,
    /// Pushes a new URL onto the history stack.
    PushUrl,
    /// Can be used to do a client-side redirect to a new location.
    Redirect,
    /// If set to `true`, the client will do a full refresh on the page.
    Refresh,
    /// Replaces the currelt URL in the location bar.
    ReplaceUrl,
    /// Allows you to specify how the response value will be swapped.
    ///
    /// See <https://htmx.org/attributes/hx-swap/> for more information.
    Reswap,
    /// A CSS selector that update the target of the content update to a
    /// different element on the page.
    Retarget,
    /// A CSS selector that allows you to choose which part of the response is
    /// used to be swapped in. Overrides an existing `hx-select` on the
    /// triggering element
    Reselect,
    /// Allows you to trigger client-side events.
    ///
    /// See <https://htmx.org/headers/hx-trigger/> for more information.
    Trigger,
    /// Allows you to trigger client-side events.
    ///
    /// See <https://htmx.org/headers/hx-trigger/> for more information.
    TriggerAfterSettle,
    /// Allows you to trigger client-side events.
    ///
    /// See <https://htmx.org/headers/hx-trigger/> for more information.
    TriggerAfterSwap,
}

impl HtmxResponseHeader {
    pub fn as_str(&self) -> &'static str {
        match self {
            HtmxResponseHeader::Location => "HX-Location",
            HtmxResponseHeader::PushUrl => "HX-Push-Url",
            HtmxResponseHeader::Redirect => "HX-Redirect",
            HtmxResponseHeader::Refresh => "HX-Refresh",
            HtmxResponseHeader::ReplaceUrl => "HX-Replace-Url",
            HtmxResponseHeader::Reswap => "HX-Reswap",
            HtmxResponseHeader::Retarget => "HX-Retarget",
            HtmxResponseHeader::Reselect => "HX-Reselect",
            HtmxResponseHeader::Trigger => "HX-Trigger",
            HtmxResponseHeader::TriggerAfterSettle => "HX-Trigger-After-Settle",
            HtmxResponseHeader::TriggerAfterSwap => "HX-Trigger-After-Swap",
        }
    }
}
