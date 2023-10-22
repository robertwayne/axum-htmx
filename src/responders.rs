//pub struct HxLocation()

use std::{collections::HashMap, convert::Infallible};

use axum::{
    http::{header::InvalidHeaderValue, HeaderValue, StatusCode, Uri},
    response::{IntoResponse, IntoResponseParts, ResponseParts},
};
use serde::Serialize;
use serde_json::Value;

use crate::headers;

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
/// This response header can be used to trigger a client side redirection without reloading the whole page. Instead of changing the page’s location it will act like following a hx-boost link, creating a new history entry, issuing an ajax request to the value of the header and pushing the path into history.
///
///
#[derive(Debug, Clone, Serialize)]
pub struct HxLocation {
    /// Url to load the response from
    pub path: String,
    /// The source element of the request
    pub source: Option<String>,
    /// An event that "triggered" the request
    pub event: Option<String>,
    /// A callback that will handle the response HTML
    pub handler: Option<String>,
    /// The target to swap the response into
    pub target: Option<String>,
    /// How the response will be swapped in relative to the target
    pub swap: Option<SwapOption>,
    /// Values to submit with the request
    pub values: Option<Value>,
    /// headers to submit with the request
    pub headers: Option<Value>,
}

impl HxLocation {
    pub fn from_uri(uri: &Uri) -> Self {
        Self {
            path: uri.to_string(),
            source: None,
            event: None,
            handler: None,
            target: None,
            swap: None,
            values: None,
            headers: None,
        }
    }
}

impl IntoResponseParts for HxLocation {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        let header_value = if self.source.is_none()
            && self.event.is_none()
            && self.handler.is_none()
            && self.target.is_none()
            && self.swap.is_none()
            && self.values.is_none()
            && self.headers.is_none()
        {
            HeaderValue::from_str(&self.path)?
        } else {
            HeaderValue::from_maybe_shared(serde_json::to_string(&self)?)?
        };

        res.headers_mut().insert(headers::HX_LOCATION, header_value);
        Ok(res)
    }
}

/// The `HX-Push-Url` header.
///
/// Pushes a new url into the history stack.
///
/// Will fail if the supplied Uri contains characters that are not visible ASCII (32-127).
#[derive(Debug, Clone)]
pub struct HxPushUrl(Uri);

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
/// Will fail if the supplied Uri contains characters that are not visible ASCII (32-127).
#[derive(Debug, Clone)]
pub struct HxRedirect(Uri);

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
pub struct HxRefresh(bool);

impl IntoResponseParts for HxRefresh {
    type Error = Infallible;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().insert(
            headers::HX_REFRESH,
            match self.0 {
                true => HeaderValue::from_static("true"),
                false => HeaderValue::from_static("false"),
            },
        );
        Ok(res)
    }
}

/// The `HX-Replace-Url` header.
///
/// Replaces the currelt URL in the location bar.
///
/// Will fail if the supplied Uri contains characters that are not visible ASCII (32-127).
#[derive(Debug, Clone)]
pub struct HxReplaceUrl(Uri);

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
pub struct HxReswap(SwapOption);

impl IntoResponseParts for HxReswap {
    type Error = Infallible;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().insert(headers::HX_RESWAP, self.0.into());
        Ok(res)
    }
}

/// The `HX-Retarget` header.
///
/// A CSS selector that updates the target of the content update to a different element on the page.
///
/// Will fail if the supplied String contains characters that are not visible ASCII (32-127).
#[derive(Debug, Clone)]
pub struct HxRetarget(String);

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
/// A CSS selector that allows you to choose which part of the response is used to be swapped in. Overrides an existing hx-select on the triggering element.
///
/// Will fail if the supplied String contains characters that are not visible ASCII (32-127).
#[derive(Debug, Clone)]
pub struct HxReselect(String);

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
/// Allows you to trigger client-side events.
///
/// Will fail if the supplied events contain produce characters that are not visible ASCII (32-127) when serializing to json.
#[derive(Debug, Clone)]
pub struct HxTrigger(Vec<HxEvent>);

impl IntoResponseParts for HxTrigger {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut()
            .insert(headers::HX_TRIGGER, events_to_header_value(self.0)?);
        Ok(res)
    }
}

/// The `HX-Trigger-After-Settle` header.
///
/// Allows you to trigger client-side events after the settle step.
///
/// Will fail if the supplied events contain produce characters that are not visible ASCII (32-127) when serializing to json.
#[derive(Debug, Clone)]
pub struct HxTriggerAfterSettle(Vec<HxEvent>);

impl IntoResponseParts for HxTriggerAfterSettle {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().insert(
            headers::HX_TRIGGER_AFTER_SETTLE,
            events_to_header_value(self.0)?,
        );
        Ok(res)
    }
}

/// The `HX-Trigger-After-Swap` header.
///
/// Allows you to trigger client-side events after the swap step.
///
/// Will fail if the supplied events contain produce characters that are not visible ASCII (32-127) when serializing to json.
#[derive(Debug, Clone)]
pub struct HxTriggerAfterSwap(Vec<HxEvent>);

impl IntoResponseParts for HxTriggerAfterSwap {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().insert(
            headers::HX_TRIGGER_AFTER_SWAP,
            events_to_header_value(self.0)?,
        );
        Ok(res)
    }
}

/// Values of the `hx-swap` attribute.
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
    /// Does not append content from response (out of band items will still be processed).
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

// can be removed  and automatically derived when https://github.com/serde-rs/serde/issues/2485
// is implemented
impl Serialize for SwapOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
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

#[derive(Debug, Clone, Serialize)]
pub struct HxEvent {
    pub name: String,
    pub data: Option<Value>,
}

impl HxEvent {
    pub fn new<T: Serialize>(name: String) -> Self {
        Self { name, data: None }
    }
    pub fn new_with_data<T: Serialize>(name: String, data: T) -> Result<Self, serde_json::Error> {
        let data = serde_json::to_value(data)?;

        Ok(Self {
            name,
            data: Some(data),
        })
    }
}

pub(crate) fn events_to_header_value(events: Vec<HxEvent>) -> Result<HeaderValue, HxError> {
    let with_data = events.iter().any(|e| e.data.is_some());

    let header_value = if with_data {
        // at least one event contains data so the header_value needs to be json encoded.
        let header_value = events
            .into_iter()
            .map(|e| (e.name, e.data.map(|d| d.to_string()).unwrap_or_default()))
            .collect::<HashMap<_, _>>();
        serde_json::to_string(&header_value)?
    } else {
        // no event contains data, the event names can be put in the header value separated
        // by a comma.
        events
            .into_iter()
            .map(|e| e.name)
            .reduce(|acc, e| acc + ", " + &e)
            .unwrap_or_default()
    };

    HeaderValue::from_maybe_shared(header_value).map_err(HxError::from)
}

pub enum HxError {
    InvalidHeaderValue(InvalidHeaderValue),
    Serialization(serde_json::Error),
}

impl From<InvalidHeaderValue> for HxError {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(value)
    }
}

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
            Self::Serialization(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to serialize event",
            )
                .into_response(),
        }
    }
}
