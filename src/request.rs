use axum::{extract::FromRequestParts, http::request::Parts};

/// Represents all of the headers that can be sent in a request to the server.
///
/// See <https://htmx.org/reference/#request_headers> for more information.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HtmxRequestHeader {
    /// Indicates that the request is via an element using `hx-boost` attribute.
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

pub struct HxHistoryRestoreRequest(pub bool);

#[axum::async_trait]
impl<S> FromRequestParts<S> for HxHistoryRestoreRequest
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if parts
            .headers
            .contains_key(HtmxRequestHeader::HistoryRestoreRequest.as_str())
        {
            return Ok(HxHistoryRestoreRequest(true));
        } else {
            return Ok(HxHistoryRestoreRequest(false));
        }
    }
}

#[derive(Debug, Clone)]
pub struct HxPrompt(pub Option<String>);

#[axum::async_trait]
impl<S> FromRequestParts<S> for HxPrompt
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(prompt) = parts.headers.get(HtmxRequestHeader::Prompt.as_str()) {
            if let Ok(prompt) = prompt.to_str() {
                return Ok(HxPrompt(Some(prompt.to_string())));
            }
        }

        return Ok(HxPrompt(None));
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

#[derive(Debug, Clone)]
pub struct HxTriggerName(pub Option<String>);

#[axum::async_trait]
impl<S> FromRequestParts<S> for HxTriggerName
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(trigger_name) = parts.headers.get(HtmxRequestHeader::TriggerName.as_str()) {
            if let Ok(trigger_name) = trigger_name.to_str() {
                return Ok(HxTriggerName(Some(trigger_name.to_string())));
            }
        }

        return Ok(HxTriggerName(None));
    }
}

#[derive(Debug, Clone)]
pub struct HxTrigger(pub Option<String>);

#[axum::async_trait]
impl<S> FromRequestParts<S> for HxTrigger
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(trigger) = parts.headers.get(HtmxRequestHeader::Trigger.as_str()) {
            if let Ok(trigger) = trigger.to_str() {
                return Ok(HxTrigger(Some(trigger.to_string())));
            }
        }

        return Ok(HxTrigger(None));
    }
}
