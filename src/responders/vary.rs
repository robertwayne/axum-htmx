use axum_core::response::{IntoResponseParts, ResponseParts};
use http::header::VARY;

use crate::{extractors, headers, HxError};

/// The `Vary: HX-Request` header.
///
/// You may want to add this header to the response if your handler responds differently based on
/// the `HX-Request` request header.
///
/// For example, if your server renders the full HTML when the `HX-Request` header is missing or
/// `false`, and it renders a fragment of that HTML when `HX-Request: true`.
///
/// You probably need this only for `GET` requests, as other HTTP methods are not cached by default.
///
/// See <https://htmx.org/docs/#caching> for more information.
#[derive(Debug, Clone)]
pub struct VaryHxRequest;

impl IntoResponseParts for VaryHxRequest {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut()
            .insert(VARY, headers::HX_REQUEST.try_into()?);

        Ok(res)
    }
}

impl extractors::HxRequest {
    /// Convenience method to create the corresponding `Vary` response header
    pub fn vary_response() -> VaryHxRequest {
        VaryHxRequest
    }
}

/// The `Vary: HX-Target` header.
///
/// You may want to add this header to the response if your handler responds differently based on
/// the `HX-Target` request header.
///
/// You probably need this only for `GET` requests, as other HTTP methods are not cached by default.
///
/// See <https://htmx.org/docs/#caching> for more information.
#[derive(Debug, Clone)]
pub struct VaryHxTarget;

impl IntoResponseParts for VaryHxTarget {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut()
            .insert(VARY, headers::HX_TARGET.try_into()?);

        Ok(res)
    }
}

impl extractors::HxTarget {
    /// Convenience method to create the corresponding `Vary` response header
    pub fn vary_response() -> VaryHxTarget {
        VaryHxTarget
    }
}

/// The `Vary: HX-Trigger` header.
///
/// You may want to add this header to the response if your handler responds differently based on
/// the `HX-Trigger` request header.
///
/// You probably need this only for `GET` requests, as other HTTP methods are not cached by default.
///
/// See <https://htmx.org/docs/#caching> for more information.
#[derive(Debug, Clone)]
pub struct VaryHxTrigger;

impl IntoResponseParts for VaryHxTrigger {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut()
            .insert(VARY, headers::HX_TRIGGER.try_into()?);

        Ok(res)
    }
}

impl extractors::HxTrigger {
    /// Convenience method to create the corresponding `Vary` response header
    pub fn vary_response() -> VaryHxTrigger {
        VaryHxTrigger
    }
}

/// The `Vary: HX-Trigger-Name` header.
///
/// You may want to add this header to the response if your handler responds differently based on
/// the `HX-Trigger-Name` request header.
///
/// You probably need this only for `GET` requests, as other HTTP methods are not cached by default.
///
/// See <https://htmx.org/docs/#caching> for more information.
#[derive(Debug, Clone)]
pub struct VaryHxTriggerName;

impl IntoResponseParts for VaryHxTriggerName {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut()
            .insert(VARY, headers::HX_TRIGGER_NAME.try_into()?);

        Ok(res)
    }
}

impl extractors::HxTriggerName {
    /// Convenience method to create the corresponding `Vary` response header
    pub fn vary_response() -> VaryHxTriggerName {
        VaryHxTriggerName
    }
}
