//! TODO

/// TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeId {
    /// TODO
    End,
    /// TODO
    Byte,
    /// TODO
    Short,
    /// TODO
    Int,
    /// TODO
    Long,
    /// TODO
    Float,
    /// TODO
    Double,
    /// TODO
    ByteArray,
    /// TODO
    String,
    /// TODO
    List,
    /// TODO
    Compound,
    /// TODO
    IntArray,
    /// TODO
    LongArray,
}

impl TypeId {
    /// TODO
    #[must_use]
    #[inline]
    pub const fn to_u8(self) -> u8 {
        match self {
            TypeId::End => 0x00,
            TypeId::Byte => 0x01,
            TypeId::Short => 0x02,
            TypeId::Int => 0x03,
            TypeId::Long => 0x04,
            TypeId::Float => 0x05,
            TypeId::Double => 0x06,
            TypeId::ByteArray => 0x07,
            TypeId::String => 0x08,
            TypeId::List => 0x09,
            TypeId::Compound => 0x0A,
            TypeId::IntArray => 0x0B,
            TypeId::LongArray => 0x0C,
        }
    }
}
