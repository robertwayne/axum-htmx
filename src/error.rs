use std::{error, fmt};

use axum_core::response::IntoResponse;
use http::{header::InvalidHeaderValue, StatusCode};

#[derive(Debug)]
pub enum HxError {
    InvalidHeaderValue(InvalidHeaderValue),

    #[cfg(feature = "serde")]
    #[cfg_attr(feature = "unstable", doc(cfg(feature = "serde")))]
    Json(serde_json::Error),
}

impl From<InvalidHeaderValue> for HxError {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(value)
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
            HxError::InvalidHeaderValue(err) => write!(f, "Invalid header value: {err}"),
            #[cfg(feature = "serde")]
            HxError::Json(err) => write!(f, "Json: {err}"),
        }
    }
}

impl error::Error for HxError {}

impl IntoResponse for HxError {
    fn into_response(self) -> axum_core::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
