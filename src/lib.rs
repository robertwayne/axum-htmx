#![forbid(unsafe_code)]

use axum::{extract::FromRequestParts, http::request::Parts};

/// Represents all of the headers that can be sent in a request to the server.
///
/// See <https://htmx.org/reference/#request_headers> for more information.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HtmxRequestHeader {
    ///	Indicates that the request is via an element using `hx-boost` attribute.
    ///
    /// See <https://htmx.org/attributes/hx-boost/> for more information.
    Boosted,
    /// The current URL of the browser.
    CurrentUrl,
    /// `true` if the request is for history restoration after a miss in the
    /// local history cache.
    HistoryRestoreRequest,
    /// The user response to an `hx-prompt`
    ///
    /// See <https://htmx.org/attributes/hx-prompt/> for more information.
    Prompt,
    /// Always `true`.
    Request,
    /// The `id` of the target element, if it exists.
    Target,
    /// The `name` of the triggered element, if it exists.
    TriggerName,
    /// The `id` of the triggered element, if it exists.
    Trigger,
}

impl HtmxRequestHeader {
    pub fn as_str(&self) -> &'static str {
        match self {
            HtmxRequestHeader::Boosted => "HX-Boosted",
            HtmxRequestHeader::CurrentUrl => "HX-Current-Url",
            HtmxRequestHeader::HistoryRestoreRequest => "HX-History-Restore-Request",
            HtmxRequestHeader::Prompt => "HX-Prompt",
            HtmxRequestHeader::Request => "HX-Request",
            HtmxRequestHeader::Target => "HX-Target",
            HtmxRequestHeader::TriggerName => "HX-Trigger-Name",
            HtmxRequestHeader::Trigger => "HX-Trigger",
        }
    }
}

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

/// The `HX-Boosted` header. This header is set when a request is made with the
/// "hx-boost" attribute is set on an element.
///
/// This extractor does not fail if no header is present, instead returning a
/// `false` value.
///
/// See <https://htmx.org/attributes/hx-boost/> for more information.
#[derive(Debug, Clone, Copy)]
pub struct HxBoosted(pub bool);

#[axum::async_trait]
impl<S> FromRequestParts<S> for HxBoosted
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if parts
            .headers
            .contains_key(HtmxRequestHeader::Boosted.as_str())
        {
            return Ok(HxBoosted(true));
        } else {
            return Ok(HxBoosted(false));
        }
    }
}

#[derive(Debug, Clone)]
pub struct HxCurrentUrl(pub String);

#[axum::async_trait]
impl<S> FromRequestParts<S> for HxCurrentUrl
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(url) = parts.headers.get(HtmxRequestHeader::CurrentUrl.as_str()) {
            if let Ok(url) = url.to_str() {
                return Ok(HxCurrentUrl(url.to_string()));
            }
        }

        return Ok(HxCurrentUrl("".to_string()));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HxRequest(pub bool);

#[axum::async_trait]
impl<S> FromRequestParts<S> for HxRequest
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(_: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        return Ok(HxRequest(true));
    }
}

#[derive(Debug, Clone)]
pub struct HxTarget(pub Option<String>);

#[axum::async_trait]
impl<S> FromRequestParts<S> for HxTarget
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(target) = parts.headers.get(HtmxRequestHeader::Target.as_str()) {
            if let Ok(target) = target.to_str() {
                return Ok(HxTarget(Some(target.to_string())));
            }
        }

        return Ok(HxTarget(None));
    }
}
