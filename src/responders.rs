//! Axum responses for htmx response headers.

use std::convert::Infallible;

use axum_core::response::{IntoResponseParts, ResponseParts};
use http::HeaderValue;

use crate::{headers, HxError};

mod location;
pub use location::*;
mod trigger;
pub use trigger::*;
mod vary;
pub use vary::*;

const HX_SWAP_INNER_HTML: &str = "innerHTML";
const HX_SWAP_OUTER_HTML: &str = "outerHTML";
const HX_SWAP_BEFORE_BEGIN: &str = "beforebegin";
const HX_SWAP_AFTER_BEGIN: &str = "afterbegin";
const HX_SWAP_BEFORE_END: &str = "beforeend";
const HX_SWAP_AFTER_END: &str = "afterend";
const HX_SWAP_DELETE: &str = "delete";
const HX_SWAP_NONE: &str = "none";

/// The `HX-Push-Url` header.
///
/// Pushes a new url into the history stack.
///
/// Will fail if the supplied Uri contains characters that are not visible ASCII
/// (32-127).
///
/// See <https://htmx.org/headers/hx-push-url/> for more information.
#[derive(Debug, Clone)]
pub struct HxPushUrl(pub String);

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

impl<'a> From<&'a str> for HxPushUrl {
    fn from(value: &'a str) -> Self {
        Self(value.to_string())
    }
}

/// The `HX-Redirect` header.
///
/// Can be used to do a client-side redirect to a new location.
///
/// Will fail if the supplied Uri contains characters that are not visible ASCII
/// (32-127).
#[derive(Debug, Clone)]
pub struct HxRedirect(pub String);

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

impl<'a> From<&'a str> for HxRedirect {
    fn from(value: &'a str) -> Self {
        Self(value.to_string())
    }
}

/// The `HX-Refresh`header.
///
/// If set to `true` the client-side will do a full refresh of the page.
///
/// This responder will never fail.
#[derive(Debug, Copy, Clone)]
pub struct HxRefresh(pub bool);

impl From<bool> for HxRefresh {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

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
pub struct HxReplaceUrl(pub String);

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

impl<'a> From<&'a str> for HxReplaceUrl {
    fn from(value: &'a str) -> Self {
        Self(value.to_string())
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

impl From<SwapOption> for HxReswap {
    fn from(value: SwapOption) -> Self {
        Self(value)
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

impl<T: Into<String>> From<T> for HxRetarget {
    fn from(value: T) -> Self {
        Self(value.into())
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

impl<T: Into<String>> From<T> for HxReselect {
    fn from(value: T) -> Self {
        Self(value.into())
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

// can be removed  and automatically derived when
// https://github.com/serde-rs/serde/issues/2485 is implemented
#[cfg(feature = "serde")]
impl ::serde::Serialize for SwapOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        const UNIT_NAME: &str = "SwapOption";
        match self {
            Self::InnerHtml => serializer.serialize_unit_variant(UNIT_NAME, 0, HX_SWAP_INNER_HTML),
            Self::OuterHtml => serializer.serialize_unit_variant(UNIT_NAME, 1, HX_SWAP_OUTER_HTML),
            Self::BeforeBegin => {
                serializer.serialize_unit_variant(UNIT_NAME, 2, HX_SWAP_BEFORE_BEGIN)
            }
            Self::AfterBegin => {
                serializer.serialize_unit_variant(UNIT_NAME, 3, HX_SWAP_AFTER_BEGIN)
            }
            Self::BeforeEnd => serializer.serialize_unit_variant(UNIT_NAME, 4, HX_SWAP_BEFORE_END),
            Self::AfterEnd => serializer.serialize_unit_variant(UNIT_NAME, 5, HX_SWAP_AFTER_END),
            Self::Delete => serializer.serialize_unit_variant(UNIT_NAME, 6, HX_SWAP_DELETE),
            Self::None => serializer.serialize_unit_variant(UNIT_NAME, 7, HX_SWAP_NONE),
        }
    }
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
