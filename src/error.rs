use std::{error, fmt};

use axum_core::response::IntoResponse;
use http::{
    StatusCode,
    header::{InvalidHeaderValue, MaxSizeReached},
};

#[derive(Debug)]
pub enum HxError {
    InvalidHeaderValue(InvalidHeaderValue),
    TooManyResponseHeaders(MaxSizeReached),

    #[cfg(feature = "serde")]
    #[cfg_attr(feature = "unstable", doc(cfg(feature = "serde")))]
    Json(serde_json::Error),
}

impl From<InvalidHeaderValue> for HxError {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(value)
    }
}

impl From<MaxSizeReached> for HxError {
    fn from(value: MaxSizeReached) -> Self {
        Self::TooManyResponseHeaders(value)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "unstable", doc(cfg(feature = "serde")))]
impl From<serde_json::Error> for HxError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl fmt::Display for HxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HxError::InvalidHeaderValue(_) => write!(f, "Invalid header value"),
            HxError::TooManyResponseHeaders(_) => write!(f, "Too many response headers"),
            #[cfg(feature = "serde")]
            HxError::Json(_) => write!(f, "Json"),
        }
    }
}

impl error::Error for HxError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            HxError::InvalidHeaderValue(e) => Some(e),
            HxError::TooManyResponseHeaders(e) => Some(e),
            #[cfg(feature = "serde")]
            HxError::Json(e) => Some(e),
        }
    }
}

impl IntoResponse for HxError {
    fn into_response(self) -> axum_core::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
