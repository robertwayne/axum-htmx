use axum_core::response::{IntoResponseParts, ResponseParts};
use http::HeaderValue;

use crate::{HxError, headers};

/// The `HX-Location` header.
///
/// This response header can be used to trigger a client side redirection
/// without reloading the whole page. If you intend to redirect to a specific
/// target on the page, you must enable the `serde` feature flag and specify
/// [`LocationOptions`].
///
/// Will fail if the supplied uri contains characters that are not visible ASCII
/// (32-127).
///
/// See <https://htmx.org/headers/hx-location/> for more information.
#[derive(Debug, Clone)]
pub struct HxLocation {
    /// URI path of the new location.
    pub path: String,
    /// Extra options.
    #[cfg(feature = "serde")]
    #[cfg_attr(feature = "unstable", doc(cfg(feature = "serde")))]
    pub options: Option<LocationOptions>,
}

impl HxLocation {
    /// Sets `path` as the location.
    #[allow(clippy::should_implement_trait)]
    pub fn from_path(path: impl AsRef<str>) -> Self {
        Self {
            path: path.as_ref().to_owned(),
            #[cfg(feature = "serde")]
            options: None,
        }
    }

    /// Sets `path` as the location with additional options.
    #[cfg(feature = "serde")]
    #[cfg_attr(feature = "unstable", doc(cfg(feature = "serde")))]
    pub fn from_path_with_options(path: impl AsRef<str>, options: LocationOptions) -> Self {
        Self {
            path: path.as_ref().to_string(),
            options: Some(options),
        }
    }

    #[cfg(feature = "serde")]
    fn into_header_with_options(self) -> Result<String, HxError> {
        match self.options {
            Some(options) if !options.is_default() => {
                #[derive(serde::Serialize)]
                struct FlattenedHxLocation<'a> {
                    path: &'a str,
                    #[serde(flatten)]
                    options: &'a LocationOptions,
                }
                let loc_with_opts = FlattenedHxLocation {
                    path: &self.path,
                    options: &options,
                };
                Ok(serde_json::to_string(&loc_with_opts)?)
            }
            _ => Ok(self.path),
        }
    }
}

impl<'a> From<&'a str> for HxLocation {
    fn from(path: &'a str) -> Self {
        Self::from_path(path)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "unstable", doc(cfg(feature = "serde")))]
impl<'a> From<(&'a str, LocationOptions)> for HxLocation {
    fn from((path, options): (&'a str, LocationOptions)) -> Self {
        Self::from_path_with_options(path, options)
    }
}

impl IntoResponseParts for HxLocation {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        #[cfg(feature = "serde")]
        let header = self.into_header_with_options()?;
        #[cfg(not(feature = "serde"))]
        let header = self.path.to_string();

        res.headers_mut().insert(
            headers::HX_LOCATION,
            HeaderValue::from_maybe_shared(header)?,
        );

        Ok(res)
    }
}

/// More options for `HX-Location` header.
///
/// - `source` - the source element of the request
/// - `event` - an event that “triggered” the request
/// - `handler` - a callback that will handle the response HTML
/// - `target` - the target to swap the response into
/// - `swap` - how the response will be swapped in relative to the target
/// - `values` - values to submit with the request
/// - `headers` - headers to submit with the request
/// - `select` - allows you to select the content you want swapped from a
///   response
#[cfg(feature = "serde")]
#[cfg_attr(feature = "unstable", doc(cfg(feature = "serde")))]
#[derive(Debug, Clone, serde::Serialize, Default)]
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
    pub swap: Option<crate::SwapOption>,
    /// Values to submit with the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<serde_json::Value>,
    /// Headers to submit with the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    // Hacky way of making this struct non-exhaustive.
    // See <https://rust-lang.github.io/rfcs/2008-non-exhaustive.html> and <https://github.com/robertwayne/axum-htmx/issues/29> for reasoning.
    #[serde(skip)]
    #[doc(hidden)]
    pub _non_exhaustive: (),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "serde")]
    fn test_serialize_location() {
        use crate::SwapOption;

        let loc = HxLocation::from("/foo");
        assert_eq!(loc.into_header_with_options().unwrap(), "/foo");

        let loc = HxLocation::from_path_with_options(
            "/foo",
            LocationOptions {
                event: Some("click".into()),
                swap: Some(SwapOption::InnerHtml),
                ..Default::default()
            },
        );
        assert_eq!(
            loc.into_header_with_options().unwrap(),
            r#"{"path":"/foo","event":"click","swap":"innerHTML"}"#
        );
    }
}
