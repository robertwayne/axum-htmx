use std::str::FromStr;

use axum_core::response::{IntoResponseParts, ResponseParts};
use http::{HeaderValue, Uri};

use crate::{headers, HxError};

/// The `HX-Location` header.
///
/// This response header can be used to trigger a client side redirection
/// without reloading the whole page. If you intend to redirect to a specific
/// target on the page, you must enable the `serde` feature flag and specify
/// [`LocationOptions`].
///
/// Will fail if the supplied Uri contains characters that are not visible ASCII
/// (32-127).
///
/// See <https://htmx.org/headers/hx-location/> for more information.
#[derive(Debug, Clone)]
pub struct HxLocation {
    /// Uri of the new location.
    pub uri: Uri,
    /// Extra options.
    #[cfg(feature = "serde")]
    #[cfg_attr(feature = "unstable", doc(cfg(feature = "serde")))]
    pub options: LocationOptions,
}

impl HxLocation {
    /// Creates location from [`Uri`] without any options.
    pub fn from_uri(uri: Uri) -> Self {
        Self {
            #[cfg(feature = "serde")]
            options: LocationOptions::default(),
            uri,
        }
    }

    /// Creates location from [`Uri`] and options.
    #[cfg(feature = "serde")]
    #[cfg_attr(feature = "unstable", doc(cfg(feature = "serde")))]
    pub fn from_uri_with_options(uri: Uri, options: LocationOptions) -> Self {
        Self { uri, options }
    }

    /// Parses `uri` and sets it as location.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(uri: impl AsRef<str>) -> Result<Self, http::uri::InvalidUri> {
        Ok(Self {
            #[cfg(feature = "serde")]
            options: LocationOptions::default(),
            uri: uri.as_ref().parse::<Uri>()?,
        })
    }

    /// Parses `uri` and sets it as location with additional options.
    #[cfg(feature = "serde")]
    #[cfg_attr(feature = "unstable", doc(cfg(feature = "serde")))]
    pub fn from_str_with_options(
        uri: impl AsRef<str>,
        options: LocationOptions,
    ) -> Result<Self, http::uri::InvalidUri> {
        Ok(Self {
            options,
            uri: uri.as_ref().parse::<Uri>()?,
        })
    }

    #[cfg(feature = "serde")]
    fn into_header_with_options(self) -> Result<String, HxError> {
        if self.options.is_default() {
            return Ok(self.uri.to_string());
        }

        #[derive(::serde::Serialize)]
        struct LocWithOpts {
            path: String,
            #[serde(flatten)]
            opts: LocationOptions,
        }

        let loc_with_opts = LocWithOpts {
            path: self.uri.to_string(),
            opts: self.options,
        };
        Ok(serde_json::to_string(&loc_with_opts)?)
    }
}

impl From<Uri> for HxLocation {
    fn from(uri: Uri) -> Self {
        Self::from_uri(uri)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "unstable", doc(cfg(feature = "serde")))]
impl From<(Uri, LocationOptions)> for HxLocation {
    fn from((uri, options): (Uri, LocationOptions)) -> Self {
        Self::from_uri_with_options(uri, options)
    }
}

impl<'a> TryFrom<&'a str> for HxLocation {
    type Error = <Uri as FromStr>::Err;

    fn try_from(uri: &'a str) -> Result<Self, Self::Error> {
        Self::from_str(uri)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "unstable", doc(cfg(feature = "serde")))]
impl<'a> TryFrom<(&'a str, LocationOptions)> for HxLocation {
    type Error = <Uri as FromStr>::Err;

    fn try_from((uri, options): (&'a str, LocationOptions)) -> Result<Self, Self::Error> {
        Self::from_str_with_options(uri, options)
    }
}

impl IntoResponseParts for HxLocation {
    type Error = HxError;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        #[cfg(feature = "serde")]
        let header = self.into_header_with_options()?;
        #[cfg(not(feature = "serde"))]
        let header = self.uri.to_string();

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
/// - `select` - allows you to select the content you want swapped from a response
#[cfg(feature = "serde")]
#[cfg_attr(feature = "unstable", doc(cfg(feature = "serde")))]
#[derive(Debug, Clone, serde::Serialize, Default)]
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
    pub swap: Option<crate::SwapOption>,
    /// Values to submit with the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<serde_json::Value>,
    /// Headers to submit with the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "unstable", doc(cfg(feature = "serde")))]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "serde")]
    fn test_serialize_location() {
        use crate::SwapOption;

        let loc = HxLocation::try_from("/foo").unwrap();
        assert_eq!(loc.into_header_with_options().unwrap(), "/foo");

        let loc = HxLocation::from_uri_with_options(
            "/foo".parse().unwrap(),
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
