//! TODO

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![deny(missing_docs, clippy::pedantic)]

extern crate alloc;

pub mod de;
pub mod error;
mod util;
mod value;

#[doc(inline)]
pub use self::value::{
    list::{self, List},
    map::{self, Map},
    Byte, ByteArray, Compound, IntArray, LongArray, Type, Value,
};
