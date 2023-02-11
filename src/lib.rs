//! TODO

extern crate alloc;

pub mod char;
pub mod de;
pub mod error;
pub mod ser;
pub mod str;
mod value;
pub mod snbt;

pub use error::{Error, Result};
pub use value::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayBrand {
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
    pub(crate) const fn id(self) -> TypeId {
        match self {
            ArrayBrand::Byte => TypeId::ByteArray,
            ArrayBrand::Int => TypeId::IntArray,
            ArrayBrand::Long => TypeId::LongArray,
        }
    }

    #[must_use]
    pub(crate) const fn element_id(self) -> TypeId {
        match self {
            ArrayBrand::Byte => TypeId::Byte,
            ArrayBrand::Int => TypeId::Int,
            ArrayBrand::Long => TypeId::Long,
        }
    }
}
