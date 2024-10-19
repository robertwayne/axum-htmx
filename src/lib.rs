#![cfg_attr(feature = "unstable", feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![allow(clippy::too_long_first_doc_paragraph)]

mod error;
pub use error::*;

#[cfg(feature = "auto-vary")]
#[cfg_attr(feature = "unstable", doc(cfg(feature = "auto-vary")))]
pub mod auto_vary;
pub mod extractors;
#[cfg(feature = "guards")]
#[cfg_attr(feature = "unstable", doc(cfg(feature = "guards")))]
pub mod guard;
pub mod headers;
pub mod responders;

#[cfg(feature = "auto-vary")]
#[cfg_attr(feature = "unstable", doc(cfg(feature = "auto-vary")))]
#[doc(inline)]
pub use auto_vary::*;
#[doc(inline)]
pub use extractors::*;
#[cfg(feature = "guards")]
#[cfg_attr(feature = "unstable", doc(cfg(feature = "guards")))]
#[doc(inline)]
pub use guard::*;
#[doc(inline)]
pub use headers::*;
#[doc(inline)]
pub use responders::*;
