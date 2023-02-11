mod error;
pub mod ser;

use crate::ArrayBrand;

/// TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeqKind {
    /// TODO
    List,
    /// TODO
    ByteArray,
    /// TODO
    IntArray,
    /// TODO
    LongArray,
}

impl ArrayBrand {
    #[must_use]
    #[inline]
    pub(crate) fn to_seq_kind(&self) -> SeqKind {
        match self {
            ArrayBrand::Byte => SeqKind::ByteArray,
            ArrayBrand::Int => SeqKind::IntArray,
            ArrayBrand::Long => SeqKind::LongArray,
        }
    }
}
