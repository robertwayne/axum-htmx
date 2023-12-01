use serde::Serialize;
use serde_json::Value;

use crate::SwapOption;

/// More options for `HX-Location` header.
///
/// - `source` - the source element of the request
/// - `event` - an event that “triggered” the request
/// - `handler` - a callback that will handle the response HTML
/// - `target` - the target to swap the response into
/// - `swap` - how the response will be swapped in relative to the target
/// - `values` - values to submit with the request
/// - `headers` - headers to submit with the request
/// - `select` - allows you to select the content you want swapped from a response
#[derive(Debug, Clone, Serialize, Default)]
#[non_exhaustive]
pub struct LocationOptions {
    /// The source element of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// An event that "triggered" the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// A callback that will handle the response HTML.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    /// The target to swap the response into.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// How the response will be swapped in relative to the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<SwapOption>,
    /// Values to submit with the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Value>,
    /// Headers to submit with the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Value>,
}

impl LocationOptions {
    pub(super) fn is_default(&self) -> bool {
        let Self {
            source: None,
            event: None,
            handler: None,
            target: None,
            swap: None,
            values: None,
            headers: None,
        } = self
        else {
            return false;
        };

        true
    }
}
