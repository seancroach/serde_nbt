use crate::value::{Id, Kind};

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
    pub(crate) const fn display(self) -> &'static str {
        match self {
            ArrayBrand::Byte => "byte array",
            ArrayBrand::Int => "int array",
            ArrayBrand::Long => "long array",
        }
    }

    #[must_use]
    pub(crate) const fn kind(self) -> Kind {
        match self {
            ArrayBrand::Byte => Kind::ByteArray,
            ArrayBrand::Int => Kind::IntArray,
            ArrayBrand::Long => Kind::LongArray,
        }
    }

    #[must_use]
    pub(crate) const fn element_kind(self) -> Kind {
        match self {
            ArrayBrand::Byte => Kind::Byte,
            ArrayBrand::Int => Kind::Int,
            ArrayBrand::Long => Kind::Long,
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

pub(crate) fn needs_escaped(s: &str) -> bool {
    !s.bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'+' | b'-' | b'.' | b'_'))
}