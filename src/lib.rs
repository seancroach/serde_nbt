//! TODO

#![deny(clippy::pedantic)]

extern crate alloc;
extern crate core;

pub mod binary;
pub mod char;
pub mod de;
mod emit;
pub mod error;
pub mod ser;
pub mod str;
pub mod value;

pub use value::{map, ByteArray, Compound, Id, List, LongArray, ValueKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ArrayBrand {
    Byte,
    Int,
    Long,
}

impl ArrayBrand {
    #[must_use]
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            ArrayBrand::Byte => "__serde_nbt_byte_array",
            ArrayBrand::Int => "__serde_nbt_int_array",
            ArrayBrand::Long => "__serde_nbt_long_array",
        }
    }

    #[must_use]
    pub(crate) const fn from_str(s: &str) -> Option<Self> {
        match s.as_bytes() {
            b"__serde_nbt_byte_array" => Some(ArrayBrand::Byte),
            b"__serde_nbt_int_array" => Some(ArrayBrand::Int),
            b"__serde_nbt_long_array" => Some(ArrayBrand::Long),
            _ => None,
        }
    }

    #[must_use]
    pub(crate) const fn kind(self) -> ValueKind {
        match self {
            ArrayBrand::Byte => ValueKind::ByteArray,
            ArrayBrand::Int => ValueKind::IntArray,
            ArrayBrand::Long => ValueKind::LongArray,
        }
    }

    #[must_use]
    pub(crate) const fn element_kind(self) -> ValueKind {
        match self {
            ArrayBrand::Byte => ValueKind::Byte,
            ArrayBrand::Int => ValueKind::Int,
            ArrayBrand::Long => ValueKind::Long,
        }
    }

    #[must_use]
    pub(crate) fn to_seq_kind(self) -> SeqKind {
        match self {
            ArrayBrand::Byte => SeqKind::ByteArray,
            ArrayBrand::Int => SeqKind::IntArray,
            ArrayBrand::Long => SeqKind::LongArray,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeqKind {
    ByteArray,
    IntArray,
    LongArray,
    List(Id),
}

impl SeqKind {
    #[must_use]
    pub const fn element_id(self) -> Id {
        match self {
            SeqKind::ByteArray => Id::Byte,
            SeqKind::IntArray => Id::Int,
            SeqKind::LongArray => Id::Long,
            SeqKind::List(id) => id,
        }
    }
}

impl Default for SeqKind {
    fn default() -> Self {
        SeqKind::List(Id::End)
    }
}
