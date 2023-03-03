//! TODO

#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![deny(clippy::pedantic)]

extern crate alloc;

#[cfg(feature = "binary")]
mod binary;
#[cfg(feature = "snbt")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "snbt")))]
pub mod char;
pub mod de;
pub mod error;
pub mod map;
pub mod ser;
#[cfg(feature = "snbt")]
mod snbt;
#[cfg(feature = "snbt")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "snbt")))]
pub mod str;
pub mod util;
pub mod value;

#[doc(inline)]
pub use crate::{
    error::{Error, Result},
    value::Value,
};

#[cfg(feature = "be")]
pub use crate::binary::ser::{to_be_vec, to_be_writer};
#[cfg(feature = "le")]
pub use crate::binary::ser::{to_le_vec, to_le_writer};
#[cfg(feature = "varint")]
pub use crate::binary::ser::{to_varint_vec, to_varint_writer};
pub use crate::value::ser::to_value;
