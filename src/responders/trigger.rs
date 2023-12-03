use axum_core::response::{IntoResponseParts, ResponseParts};

use crate::{headers, HxError};

/// Represents a client-side event carrying optional data.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct HxEvent {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(feature = "serde")]
    #[cfg_attr(feature = "unstable", doc(cfg(feature = "serde")))]
    pub data: Option<serde_json::Value>,
}

impl HxEvent {
    /// Creates new event with no associated data.
    pub fn new(name: String) -> Self {
        Self {
            name: name.to_string(),
            #[cfg(feature = "serde")]
            data: None,
        }
    }

    /// Creates new event with data.
    #[cfg(feature = "serde")]
    #[cfg_attr(feature = "unstable", doc(cfg(feature = "serde")))]
    pub fn new_with_data<T: ::serde::Serialize>(
        name: impl AsRef<str>,
        data: T,
    ) -> Result<Self, serde_json::Error> {
        let data = serde_json::to_value(data)?;

        Ok(Self {
            name: name.as_ref().to_owned(),
            #[cfg(feature = "serde")]
            data: Some(data),
        })
    }
}

impl<N: AsRef<str>> From<N> for HxEvent {
    fn from(name: N) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            #[cfg(feature = "serde")]
            data: None,
        }
    }
}

#[cfg(not(feature = "serde"))]
fn events_to_header_value(events: Vec<HxEvent>) -> Result<http::HeaderValue, HxError> {
    let header = events
        .into_iter()
        .map(|HxEvent { name }| name)
        .collect::<Vec<_>>()
        .join(", ");

    http::HeaderValue::from_str(&header).map_err(Into::into)
}

#[cfg(feature = "serde")]
fn events_to_header_value(events: Vec<HxEvent>) -> Result<http::HeaderValue, HxError> {
    use std::collections::HashMap;

    use http::HeaderValue;
    use serde_json::Value;

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

/// Describes when should event be triggered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TriggerMode {
    Normal,
    AfterSettle,
    AfterSwap,
}

/// The `HX-Trigger*` header.
///
/// Allows you to trigger client-side events.
/// Corresponds to `HX-Trigger`, `HX-Trigger-After-Settle` and `HX-Trigger-After-Swap` headers.
/// To change when events trigger use appropriate `mode`.
///
/// Will fail if the supplied events contain or produce characters that are not
/// visible ASCII (32-127) when serializing to JSON.
///
/// See <https://htmx.org/headers/hx-trigger/> for more information.
#[derive(Debug, Clone)]
pub struct HxResponseTrigger {
    pub mode: TriggerMode,
    pub events: Vec<HxEvent>,
}

impl HxResponseTrigger {
    /// Creates new [trigger](https://htmx.org/headers/hx-trigger/) with specified mode and events.
    pub fn new<T: Into<HxEvent>>(mode: TriggerMode, events: impl IntoIterator<Item = T>) -> Self {
        Self {
            mode,
            events: events.into_iter().map(Into::into).collect(),
        }
    }

    /// Creates new [normal](https://htmx.org/headers/hx-trigger/) trigger from events.
    pub fn normal<T: Into<HxEvent>>(events: impl IntoIterator<Item = T>) -> Self {
        Self::new(TriggerMode::Normal, events)
    }

    /// Creates new [after settle](https://htmx.org/headers/hx-trigger/) trigger from events.
    pub fn after_settle<T: Into<HxEvent>>(events: impl IntoIterator<Item = T>) -> Self {
        Self::new(TriggerMode::AfterSettle, events)
    }

    /// Creates new [after swap](https://htmx.org/headers/hx-trigger/) trigger from events.
    pub fn after_swap<T: Into<HxEvent>>(events: impl IntoIterator<Item = T>) -> Self {
        Self::new(TriggerMode::AfterSwap, events)
    }
}

impl<T> From<(TriggerMode, T)> for HxResponseTrigger
where
    T: IntoIterator,
    T::Item: Into<HxEvent>,
{
    fn from((mode, events): (TriggerMode, T)) -> Self {
        Self {
            mode,
            events: events.into_iter().map(Into::into).collect(),
        }
    }
}

impl IntoResponseParts for HxResponseTrigger {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        if !self.events.is_empty() {
            let header = match self.mode {
                TriggerMode::Normal => headers::HX_TRIGGER,
                TriggerMode::AfterSettle => headers::HX_TRIGGER_AFTER_SETTLE,
                TriggerMode::AfterSwap => headers::HX_TRIGGER_AFTER_SETTLE,
            };

            res.headers_mut()
                .insert(header, events_to_header_value(self.events)?);
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use http::HeaderValue;
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

        let value =
            events_to_header_value(HxResponseTrigger::normal(["foo", "bar"]).events).unwrap();
        assert_eq!(value, HeaderValue::from_static("foo, bar"));
    }
}
