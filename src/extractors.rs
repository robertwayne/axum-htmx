//! Axum extractors for htmx request headers.

use axum_core::extract::FromRequestParts;
use http::request::Parts;

use crate::{
    HX_BOOSTED, HX_CURRENT_URL, HX_HISTORY_RESTORE_REQUEST, HX_PROMPT, HX_REQUEST, HX_SOURCE,
    HX_SOURCE_NAME, HX_TARGET, HX_TRIGGER, HX_TRIGGER_NAME,
};

/// The `HX-Boosted` header.
///
/// This is set when a request is made from an element where its parent has the
/// `hx-boost` attribute set to `true`.
///
/// This extractor will always return a value. If the header is not present, it
/// will return `false`.
///
/// See <https://htmx.org/attributes/hx-boost/> for more information.
#[derive(Debug, Clone, Copy)]
pub struct HxBoosted(pub bool);

impl<S> FromRequestParts<S> for HxBoosted
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if parts.headers.contains_key(HX_BOOSTED) {
            Ok(HxBoosted(true))
        } else {
            Ok(HxBoosted(false))
        }
    }
}

/// The `HX-Current-Url` header.
///
/// This is set on every request made by htmx itself. As its name implies, it
/// just contains the current url.
///
/// This extractor will always return a value. If the header is not present, or
/// extractor fails to parse the url it will return `None`.
#[derive(Debug, Clone)]
pub struct HxCurrentUrl(pub Option<http::Uri>);

impl<S> FromRequestParts<S> for HxCurrentUrl
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(url) = parts.headers.get(HX_CURRENT_URL) {
            let url = url
                .to_str()
                .ok()
                .and_then(|url| url.parse::<http::Uri>().ok());

            return Ok(HxCurrentUrl(url));
        }

        Ok(HxCurrentUrl(None))
    }
}

/// The `HX-History-Restore-Request` header.
///
/// This extractor will always return a value. If the header is not present, it
/// will return `false`.
#[derive(Debug, Clone, Copy)]
pub struct HxHistoryRestoreRequest(pub bool);

impl<S> FromRequestParts<S> for HxHistoryRestoreRequest
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if parts.headers.contains_key(HX_HISTORY_RESTORE_REQUEST) {
            Ok(HxHistoryRestoreRequest(true))
        } else {
            Ok(HxHistoryRestoreRequest(false))
        }
    }
}

/// The `HX-Prompt` header.
///
/// This is set when a request is made from an element that has the `hx-prompt`
/// attribute set. The value will contain the string input by the user.
///
/// This extractor will always return a value. If the header is not present, it
/// will return `None`.
#[derive(Debug, Clone)]
pub struct HxPrompt(pub Option<String>);

impl<S> FromRequestParts<S> for HxPrompt
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(prompt) = parts.headers.get(HX_PROMPT) {
            if let Ok(prompt) = prompt.to_str() {
                return Ok(HxPrompt(Some(prompt.to_string())));
            }
        }

        Ok(HxPrompt(None))
    }
}

/// The `HX-Request` header.
///
/// This is set on every request made by htmx itself. It won't be present on
/// requests made manually, or by other libraries.
///
/// This extractor will always return a value. If the header is not present, it
/// will return `false`.
#[derive(Debug, Clone, Copy)]
pub struct HxRequest(pub bool);

impl<S> FromRequestParts<S> for HxRequest
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        #[cfg(feature = "auto-vary")]
        parts
            .extensions
            .get_mut::<crate::auto_vary::HxRequestExtracted>()
            .map(crate::auto_vary::Notifier::notify);

        if parts.headers.contains_key(HX_REQUEST) {
            Ok(HxRequest(true))
        } else {
            Ok(HxRequest(false))
        }
    }
}

/// The `HX-Target` header.
///
/// This is set when a request is made from an element that has the `hx-target`
/// attribute set. The value will contain the target element's id. If the id
/// does not exist on the page, the value will be None.
///
/// This extractor will always return a value. If the header is not present, it
/// will return `None`.
#[derive(Debug, Clone)]
pub struct HxTarget(pub Option<String>);

impl<S> FromRequestParts<S> for HxTarget
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        #[cfg(feature = "auto-vary")]
        parts
            .extensions
            .get_mut::<crate::auto_vary::HxTargetExtracted>()
            .map(crate::auto_vary::Notifier::notify);

        if let Some(target) = parts.headers.get(HX_TARGET) {
            if let Ok(target) = target.to_str() {
                return Ok(HxTarget(Some(target.to_string())));
            }
        }

        Ok(HxTarget(None))
    }
}

/// The `HX-Trigger-Name` header.
///
/// This is set when a request is made from an element that has the `hx-trigger`
/// attribute set. The value will contain the trigger element's name. If the
/// name does not exist on the page, the value will be None.
///
/// This extractor will always return a value. If the header is not present, it
/// will return `None`.
#[derive(Debug, Clone)]
pub struct HxTriggerName(pub Option<String>);

impl<S> FromRequestParts<S> for HxTriggerName
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        #[cfg(feature = "auto-vary")]
        parts
            .extensions
            .get_mut::<crate::auto_vary::HxTriggerNameExtracted>()
            .map(crate::auto_vary::Notifier::notify);

        if let Some(trigger_name) = parts.headers.get(HX_TRIGGER_NAME) {
            if let Ok(trigger_name) = trigger_name.to_str() {
                return Ok(HxTriggerName(Some(trigger_name.to_string())));
            }
        }

        Ok(HxTriggerName(None))
    }
}

/// The `HX-Trigger` header.
///
/// This is set when a request is made from an element that has the `hx-trigger`
/// attribute set. The value will contain the trigger element's id. If the id
/// does not exist on the page, the value will be None.
///
/// This extractor will always return a value. If the header is not present, it
/// will return `None`.
#[derive(Debug, Clone)]
pub struct HxTrigger(pub Option<String>);

impl<S> FromRequestParts<S> for HxTrigger
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        #[cfg(feature = "auto-vary")]
        parts
            .extensions
            .get_mut::<crate::auto_vary::HxTriggerExtracted>()
            .map(crate::auto_vary::Notifier::notify);

        if let Some(trigger) = parts.headers.get(HX_TRIGGER) {
            if let Ok(trigger) = trigger.to_str() {
                return Ok(HxTrigger(Some(trigger.to_string())));
            }
        }

        Ok(HxTrigger(None))
    }
}

/// The `HX-Source-Name` header.
///
/// This is set when a request is made from an element that has the `hx-source`
/// attribute set. The value will contain the source element's name. If the
/// name does not exist on the page, the value will be None.
///
/// This extractor will always return a value. If the header is not present, it
/// will return `None`.
#[derive(Debug, Clone)]
pub struct HxSourceName(pub Option<String>);

impl<S> FromRequestParts<S> for HxSourceName
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        #[cfg(feature = "auto-vary")]
        parts
            .extensions
            .get_mut::<crate::auto_vary::HxSourceNameExtracted>()
            .map(crate::auto_vary::Notifier::notify);

        if let Some(source_name) = parts.headers.get(HX_SOURCE_NAME) {
            if let Ok(source_name) = source_name.to_str() {
                return Ok(HxSourceName(Some(source_name.to_string())));
            }
        }

        Ok(HxSourceName(None))
    }
}

/// The `HX-Source` header.
///
/// This is set when a request is made from an element that has the `hx-source`
/// attribute set. The value will contain the source element's id. If the id
/// does not exist on the page, the value will be None.
///
/// This extractor will always return a value. If the header is not present, it
/// will return `None`.
#[derive(Debug, Clone)]
pub struct HxSource(pub Option<String>);

impl<S> FromRequestParts<S> for HxSource
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        #[cfg(feature = "auto-vary")]
        parts
            .extensions
            .get_mut::<crate::auto_vary::HxSourceExtracted>()
            .map(crate::auto_vary::Notifier::notify);

        if let Some(source) = parts.headers.get(HX_SOURCE) {
            if let Ok(source) = source.to_str() {
                return Ok(HxSource(Some(source.to_string())));
            }
        }

        Ok(HxSource(None))
    }
}
