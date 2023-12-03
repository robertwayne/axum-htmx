#![cfg_attr(feature = "unstable", feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod error;
pub use error::*;

pub mod extractors;
#[cfg(feature = "guards")]
#[cfg_attr(feature = "unstable", doc(cfg(feature = "guards")))]
pub mod guard;
pub mod headers;
pub mod responders;

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
