use std::collections::HashMap;

use axum_core::response::{IntoResponseParts, ResponseParts};
use http::{HeaderValue, Uri};
use serde::Serialize;
use serde_json::Value;

use crate::{
    headers,
    responders::{
        HX_SWAP_AFTER_BEGIN, HX_SWAP_AFTER_END, HX_SWAP_BEFORE_BEGIN, HX_SWAP_BEFORE_END,
        HX_SWAP_DELETE, HX_SWAP_INNER_HTML, HX_SWAP_NONE, HX_SWAP_OUTER_HTML,
    },
    HxError, SwapOption,
};

/// The `HX-Location` header.
///
/// This response header can be used to trigger a client side redirection
/// without reloading the whole page. If you only intend to redirect to the
/// `document.body`, as opposed to a specific target, you can use
/// `axum_htmx::HxResponseLocation` instead.
///
/// Will fail if the supplied data contains or produces characters that are not
/// visible ASCII (32-127) when serializing to JSON.
///
/// See <https://htmx.org/headers/hx-location/> for more information.
#[derive(Debug, Clone, Serialize)]
pub struct HxLocation {
    /// Url to load the response from.
    pub path: String,
    /// The source element of the request.
    pub source: Option<String>,
    /// An event that "triggered" the request.
    pub event: Option<String>,
    /// A callback that will handle the response HTML.
    pub handler: Option<String>,
    /// The target to swap the response into.
    pub target: Option<String>,
    /// How the response will be swapped in relative to the target.
    pub swap: Option<SwapOption>,
    /// Values to submit with the request.
    pub values: Option<Value>,
    /// Headers to submit with the request.
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

/// The `HX-Trigger` header.
///
/// Allows you to trigger client-side events. If you only need to send bare
/// events, you can use `axum_htmx::HxResponseTrigger` instead.
///
/// Will fail if the supplied events contain or produce characters that are not
/// visible ASCII (32-127) when serializing to JSON.
///
/// See <https://htmx.org/headers/hx-trigger/> for more information.
#[derive(Debug, Clone)]
pub struct HxResponseTrigger(pub Vec<HxEvent>);

impl IntoResponseParts for HxResponseTrigger {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut()
            .insert(headers::HX_TRIGGER, events_to_header_value(self.0)?);

        Ok(res)
    }
}

/// The `HX-Trigger-After-Settle` header.
///
/// Allows you to trigger client-side events after the settle step. If you only
/// intend to send bare events, you can use
/// `axum_htmx::HxResponseTriggerAfterSettle` instead.
///
/// Will fail if the supplied events contain or produce characters that are not
/// visible ASCII (32-127) when serializing to JSON.
///
/// See <https://htmx.org/headers/hx-trigger/> for more information.
#[derive(Debug, Clone)]
pub struct HxResponseTriggerAfterSettle(pub Vec<HxEvent>);

impl IntoResponseParts for HxResponseTriggerAfterSettle {
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
/// Allows you to trigger client-side events after the swap step. If you only
/// need to send bare events, you can use
/// `axum_htmx::HxResponseTriggerAfterSwao` instead.
///
/// Will fail if the supplied events contain or produce characters that are not
/// visible ASCII (32-127) when serializing to JSON.
///
/// See <https://htmx.org/headers/hx-trigger/> for more information.
#[derive(Debug, Clone)]
pub struct HxResponseTriggerAfterSwap(pub Vec<HxEvent>);

impl IntoResponseParts for HxResponseTriggerAfterSwap {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().insert(
            headers::HX_TRIGGER_AFTER_SWAP,
            events_to_header_value(self.0)?,
        );

        Ok(res)
    }
}

/// Represents a client-side event carrying optional data.
#[derive(Debug, Clone, Serialize)]
pub struct HxEvent {
    pub name: String,
    pub data: Option<Value>,
}

impl HxEvent {
    pub fn new<T: Serialize>(name: String) -> Self {
        Self {
            name: name.to_string(),
            data: None,
        }
    }

    pub fn new_with_data<T: Serialize>(name: &str, data: T) -> Result<Self, serde_json::Error> {
        let data = serde_json::to_value(data)?;

        Ok(Self {
            name: name.to_string(),
            data: Some(data),
        })
    }
}

pub(crate) fn events_to_header_value(events: Vec<HxEvent>) -> Result<HeaderValue, HxError> {
    let with_data = events.iter().any(|e| e.data.is_some());

    let header_value = if with_data {
        // at least one event contains data so the header_value needs to be json
        // encoded.
        let header_value = events
            .into_iter()
            .map(|e| (e.name, e.data.unwrap_or_default()))
            .collect::<HashMap<String, Value>>();

        serde_json::to_string(&header_value)?
    } else {
        // no event contains data, the event names can be put in the header
        // value separated by a comma.
        events
            .into_iter()
            .map(|e| e.name)
            .reduce(|acc, e| acc + ", " + &e)
            .unwrap_or_default()
    };

    HeaderValue::from_maybe_shared(header_value).map_err(HxError::from)
}

// can be removed  and automatically derived when
// https://github.com/serde-rs/serde/issues/2485 is implemented
impl serde::Serialize for SwapOption {
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn valid_event_to_header_encoding() {
        let evt = HxEvent::new_with_data(
            "my-event",
            json!({"level": "info", "message": {
                "body": "This is a test message.",
                "title": "Hello, world!",
            }}),
        )
        .unwrap();

        let header_value = events_to_header_value(vec![evt]).unwrap();

        let expected_value = r#"{"my-event":{"level":"info","message":{"body":"This is a test message.","title":"Hello, world!"}}}"#;

        assert_eq!(header_value, HeaderValue::from_static(expected_value));
    }
}
