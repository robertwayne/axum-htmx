//! Axum responses for htmx response headers.

use std::convert::Infallible;

use axum::{
    http::{header::InvalidHeaderValue, HeaderValue, StatusCode, Uri},
    response::{IntoResponse, IntoResponseParts, ResponseParts},
};

use crate::headers;

#[cfg(feature = "serde")]
pub mod serde;

const HX_SWAP_INNER_HTML: &str = "innerHTML";
const HX_SWAP_OUTER_HTML: &str = "outerHTML";
const HX_SWAP_BEFORE_BEGIN: &str = "beforebegin";
const HX_SWAP_AFTER_BEGIN: &str = "afterbegin";
const HX_SWAP_BEFORE_END: &str = "beforeend";
const HX_SWAP_AFTER_END: &str = "afterend";
const HX_SWAP_DELETE: &str = "delete";
const HX_SWAP_NONE: &str = "none";

/// The `HX-Location` header.
///
/// This response header can be used to trigger a client side redirection
/// without reloading the whole page. If you intend to redirect to a specific
/// target on the page, you must enable the `serde` feature flag and use
/// `axum_htmx::responders::serde::HxLocation` instead.
///
/// Will fail if the supplied Uri contains characters that are not visible ASCII
/// (32-127).
///
/// See <https://htmx.org/headers/hx-location/> for more information.
#[derive(Debug, Clone)]
pub struct HxLocation(pub Uri);

impl IntoResponseParts for HxLocation {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().insert(
            headers::HX_LOCATION,
            HeaderValue::from_maybe_shared(self.0.to_string())?,
        );

        Ok(res)
    }
}

/// The `HX-Push-Url` header.
///
/// Pushes a new url into the history stack.
///
/// Will fail if the supplied Uri contains characters that are not visible ASCII
/// (32-127).
///
/// See <https://htmx.org/headers/hx-push-url/> for more information.
#[derive(Debug, Clone)]
pub struct HxPushUrl(pub Uri);

impl IntoResponseParts for HxPushUrl {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().insert(
            headers::HX_PUSH_URL,
            HeaderValue::from_maybe_shared(self.0.to_string())?,
        );

        Ok(res)
    }
}

/// The `HX-Redirect` header.
///
/// Can be used to do a client-side redirect to a new location.
///
/// Will fail if the supplied Uri contains characters that are not visible ASCII
/// (32-127).
#[derive(Debug, Clone)]
pub struct HxRedirect(pub Uri);

impl IntoResponseParts for HxRedirect {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().insert(
            headers::HX_REDIRECT,
            HeaderValue::from_maybe_shared(self.0.to_string())?,
        );

        Ok(res)
    }
}

/// The `HX-Refresh`header.
///
/// If set to `true` the client-side will do a full refresh of the page.
///
/// This responder will never fail.
#[derive(Debug, Copy, Clone)]
pub struct HxRefresh(pub bool);

impl IntoResponseParts for HxRefresh {
    type Error = Infallible;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().insert(
            headers::HX_REFRESH,
            if self.0 {
                HeaderValue::from_static("true")
            } else {
                HeaderValue::from_static("false")
            },
        );

        Ok(res)
    }
}

/// The `HX-Replace-Url` header.
///
/// Replaces the currelt URL in the location bar.
///
/// Will fail if the supplied Uri contains characters that are not visible ASCII
/// (32-127).
///
/// See <https://htmx.org/headers/hx-replace-url/> for more information.
#[derive(Debug, Clone)]
pub struct HxReplaceUrl(pub Uri);

impl IntoResponseParts for HxReplaceUrl {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().insert(
            headers::HX_REPLACE_URL,
            HeaderValue::from_maybe_shared(self.0.to_string())?,
        );

        Ok(res)
    }
}

/// The `HX-Reswap` header.
///
/// Allows you to specidy how the response will be swapped.
///
/// This responder will never fail.
#[derive(Debug, Copy, Clone)]
pub struct HxReswap(pub SwapOption);

impl IntoResponseParts for HxReswap {
    type Error = Infallible;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().insert(headers::HX_RESWAP, self.0.into());

        Ok(res)
    }
}

/// The `HX-Retarget` header.
///
/// A CSS selector that updates the target of the content update to a different
/// element on the page.
///
/// Will fail if the supplied String contains characters that are not visible
/// ASCII (32-127).
#[derive(Debug, Clone)]
pub struct HxRetarget(pub String);

impl IntoResponseParts for HxRetarget {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().insert(
            headers::HX_RETARGET,
            HeaderValue::from_maybe_shared(self.0)?,
        );

        Ok(res)
    }
}

/// The `HX-Reselect` header.
///
/// A CSS selector that allows you to choose which part of the response is used
/// to be swapped in. Overrides an existing hx-select on the triggering element.
///
/// Will fail if the supplied String contains characters that are not visible
/// ASCII (32-127).
#[derive(Debug, Clone)]
pub struct HxReselect(pub String);

impl IntoResponseParts for HxReselect {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().insert(
            headers::HX_RESELECT,
            HeaderValue::from_maybe_shared(self.0)?,
        );

        Ok(res)
    }
}

/// The `HX-Trigger` header.
///
/// Allows you to trigger client-side events. If you intend to add data to your
/// events, you must enable the `serde` feature flag and use
/// `axum_htmx::responders::serde::HxResponseTrigger` instead.
///
/// Will fail if the supplied events contain or produce characters that are not
/// visible ASCII (32-127) when serializing to JSON.
///
/// See <https://htmx.org/headers/hx-trigger/> for more information.
#[derive(Debug, Clone)]
pub struct HxResponseTrigger(pub Vec<String>);

impl HxResponseTrigger {
    pub fn new(events: &[&str]) -> Self {
        Self(events.iter().map(|e| e.to_string()).collect())
    }
}

impl IntoResponseParts for HxResponseTrigger {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().insert(
            headers::HX_TRIGGER,
            HeaderValue::from_maybe_shared(
                self.0
                    .into_iter()
                    .reduce(|acc, e| acc + ", " + &e)
                    .unwrap_or_default(),
            )?,
        );

        Ok(res)
    }
}

/// The `HX-Trigger-After-Settle` header.
///
/// Allows you to trigger client-side events after the settle step. If you
/// intend to add data to your events, you must enable the `serde` feature flag
/// and use `axum_htmx::responders::serde::HxResponseTriggerAfterSettle`
/// instead.
///
/// Will fail if the supplied events contain or produce characters that are not
/// visible ASCII (32-127) when serializing to JSON.
///
/// See <https://htmx.org/headers/hx-trigger/> for more information.
#[derive(Debug, Clone)]
pub struct HxResponseTriggerAfterSettle(pub Vec<String>);

impl IntoResponseParts for HxResponseTriggerAfterSettle {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().insert(
            headers::HX_TRIGGER_AFTER_SETTLE,
            HeaderValue::from_maybe_shared(
                self.0
                    .into_iter()
                    .reduce(|acc, e| acc + ", " + &e)
                    .unwrap_or_default(),
            )?,
        );

        Ok(res)
    }
}

/// The `HX-Trigger-After-Swap` header.
///
/// Allows you to trigger client-side events after the swap step. If you intend
/// to add data to your events, you must enable the `serde` feature flag and use
/// `axum_htmx::responders::serde::HxResponseTriggerAfterSwap` instead.
///
/// Will fail if the supplied events contain or produce characters that are not
/// visible ASCII (32-127) when serializing to JSON.
///
/// See <https://htmx.org/headers/hx-trigger/> for more information.
#[derive(Debug, Clone)]
pub struct HxResponseTriggerAfterSwap(pub Vec<String>);

impl IntoResponseParts for HxResponseTriggerAfterSwap {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().insert(
            headers::HX_TRIGGER_AFTER_SWAP,
            HeaderValue::from_maybe_shared(
                self.0
                    .into_iter()
                    .reduce(|acc, e| acc + ", " + &e)
                    .unwrap_or_default(),
            )?,
        );

        Ok(res)
    }
}

/// Values of the `hx-swap` attribute.
// serde::Serialize is implemented in responders/serde.rs
#[derive(Debug, Copy, Clone)]
pub enum SwapOption {
    /// Replace the inner html of the target element.
    InnerHtml,
    /// Replace the entire target element with the response.
    OuterHtml,
    /// Insert the response before the target element.
    BeforeBegin,
    /// Insert the response before the first child of the target element.
    AfterBegin,
    /// Insert the response after the last child of the target element
    BeforeEnd,
    /// Insert the response after the target element
    AfterEnd,
    /// Deletes the target element regardless of the response
    Delete,
    /// Does not append content from response (out of band items will still be
    /// processed).
    None,
}

impl From<SwapOption> for HeaderValue {
    fn from(value: SwapOption) -> Self {
        match value {
            SwapOption::InnerHtml => HeaderValue::from_static(HX_SWAP_INNER_HTML),
            SwapOption::OuterHtml => HeaderValue::from_static(HX_SWAP_OUTER_HTML),
            SwapOption::BeforeBegin => HeaderValue::from_static(HX_SWAP_BEFORE_BEGIN),
            SwapOption::AfterBegin => HeaderValue::from_static(HX_SWAP_AFTER_BEGIN),
            SwapOption::BeforeEnd => HeaderValue::from_static(HX_SWAP_BEFORE_END),
            SwapOption::AfterEnd => HeaderValue::from_static(HX_SWAP_AFTER_END),
            SwapOption::Delete => HeaderValue::from_static(HX_SWAP_DELETE),
            SwapOption::None => HeaderValue::from_static(HX_SWAP_NONE),
        }
    }
}

#[derive(Debug)]
pub enum HxError {
    InvalidHeaderValue(InvalidHeaderValue),

    #[cfg(feature = "serde")]
    Serialization(serde_json::Error),
}

impl From<InvalidHeaderValue> for HxError {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(value)
    }
}

#[cfg(feature = "serde")]
impl From<serde_json::Error> for HxError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialization(value)
    }
}

impl IntoResponse for HxError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::InvalidHeaderValue(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "invalid header value").into_response()
            }

            #[cfg(feature = "serde")]
            Self::Serialization(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to serialize event",
            )
                .into_response(),
        }
    }
}
