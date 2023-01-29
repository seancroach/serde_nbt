use alloc::{borrow::Cow, boxed::Box, vec::Vec};
use core::{
    cmp, fmt,
    hash::{Hash, Hasher},
    ops::{Deref, Index, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

use ahash::RandomState;
use indexmap::IndexMap;

////////////////////////////////////////////////////////////////////////////////
// NbtValue
////////////////////////////////////////////////////////////////////////////////

/// TODO
pub enum NbtValue {
    /// TODO
    Byte(NbtByte),
    /// TODO
    Short(i16),
    /// TODO
    Int(i32),
    /// TODO
    Long(i64),
    /// TODO
    Float(f32),
    /// TODO
    Double(f64),
    /// TODO
    ByteArray(NbtByteArray),
    /// TODO
    String(NbtString),
    /// TODO
    List(NbtList),
    /// TODO
    Compound(NbtCompound),
    /// TODO
    IntArray(NbtIntArray),
    /// TODO
    LongArray(NbtLongArray),
}

/// TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NbtId {
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

impl NbtId {
    /// TODO
    #[must_use]
    pub const fn from_u8(n: u8) -> Option<Self> {
        match n {
            0x00 => Some(NbtId::End),
            0x01 => Some(NbtId::Byte),
            0x02 => Some(NbtId::Short),
            0x03 => Some(NbtId::Int),
            0x04 => Some(NbtId::Long),
            0x05 => Some(NbtId::Float),
            0x06 => Some(NbtId::Double),
            0x07 => Some(NbtId::ByteArray),
            0x08 => Some(NbtId::String),
            0x09 => Some(NbtId::List),
            0x0A => Some(NbtId::Compound),
            0x0B => Some(NbtId::IntArray),
            0x0C => Some(NbtId::LongArray),
            _ => None,
        }
    }

    /// TODO
    #[must_use]
    pub const fn to_u8(self) -> u8 {
        match self {
            NbtId::End => 0x00,
            NbtId::Byte => 0x01,
            NbtId::Short => 0x02,
            NbtId::Int => 0x03,
            NbtId::Long => 0x04,
            NbtId::Float => 0x05,
            NbtId::Double => 0x06,
            NbtId::ByteArray => 0x07,
            NbtId::String => 0x08,
            NbtId::List => 0x09,
            NbtId::Compound => 0x0A,
            NbtId::IntArray => 0x0B,
            NbtId::LongArray => 0x0C,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// TAG_Byte
////////////////////////////////////////////////////////////////////////////////

/// TODO
#[derive(Debug, Clone, Copy)]
pub enum NbtByte {
    /// TODO
    Bool(bool),
    /// TODO
    I8(i8),
}

impl NbtByte {
    /// TODO
    #[must_use]
    #[inline]
    pub const fn to_bool(self) -> bool {
        match self {
            NbtByte::Bool(b) => b,
            NbtByte::I8(0) => false,
            NbtByte::I8(_) => true,
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn to_i8(self) -> i8 {
        match self {
            NbtByte::Bool(false) => 0,
            NbtByte::Bool(true) => 1,
            NbtByte::I8(n) => n,
        }
    }
}

impl PartialEq for NbtByte {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.to_i8() == other.to_i8()
    }
}

impl Eq for NbtByte {}

impl PartialOrd for NbtByte {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NbtByte {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.to_i8().cmp(&other.to_i8())
    }
}

impl Hash for NbtByte {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_i8().hash(state);
    }
}

impl Default for NbtByte {
    /// Returns the default value of [`NbtByte::I8(0)`](NbtByte::I8).
    #[inline]
    fn default() -> Self {
        NbtByte::I8(0)
    }
}

impl From<bool> for NbtByte {
    #[inline]
    fn from(value: bool) -> Self {
        NbtByte::Bool(value)
    }
}

impl From<i8> for NbtByte {
    #[inline]
    fn from(value: i8) -> Self {
        NbtByte::I8(value)
    }
}

impl fmt::Display for NbtByte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            NbtByte::Bool(false) => f.write_str("false"),
            NbtByte::Bool(true) => f.write_str("true"),
            NbtByte::I8(n) => {
                let mut buf = itoa::Buffer::new();
                let s = buf.format(n);
                f.write_str(s)?;
                f.write_str("b")?;
                Ok(())
            }
        }
    }
}

impl fmt::Binary for NbtByte {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_i8().fmt(f)
    }
}

impl fmt::Octal for NbtByte {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_i8().fmt(f)
    }
}

impl fmt::LowerHex for NbtByte {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_i8().fmt(f)
    }
}

impl fmt::UpperHex for NbtByte {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_i8().fmt(f)
    }
}

////////////////////////////////////////////////////////////////////////////////
// TAG_Byte_Array
////////////////////////////////////////////////////////////////////////////////

/// TODO
pub type NbtByteArray = Vec<NbtByte>;

////////////////////////////////////////////////////////////////////////////////
// TAG_String
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct NbtString {
    inner: String,
    quotes: QuoteKind,
}

impl NbtString {
    /// TODO
    #[must_use]
    #[inline]
    pub fn quotes(&self) -> QuoteKind {
        self.quotes
    }

    /// TODO
    #[must_use]
    #[inline]
    pub fn into_inner(self) -> String {
        self.inner
    }

    /// TODO
    #[must_use]
    #[inline]
    pub fn as_string(&self) -> &String {
        &self.inner
    }
}

impl Deref for NbtString {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_string()
    }
}

macro_rules! impl_string_index {
    ($index:ty) => {
        impl Index<$index> for NbtString {
            type Output = str;

            #[inline]
            fn index(&self, index: $index) -> &Self::Output {
                self.inner.index(index)
            }
        }
    };
}

impl_string_index!(Range<usize>);
impl_string_index!(RangeFrom<usize>);
impl_string_index!(RangeFull);
impl_string_index!(RangeInclusive<usize>);
impl_string_index!(RangeTo<usize>);
impl_string_index!(RangeToInclusive<usize>);

macro_rules! impl_string_eq {
    ($lhs:ty, $rhs:ty) => {
        #[allow(unused_lifetimes)]
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                PartialEq::eq(&self[..], &other[..])
            }
        }

        #[allow(unused_lifetimes)]
        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                PartialEq::eq(&self[..], &other[..])
            }
        }
    };
}

impl PartialEq for NbtString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl_string_eq!(NbtString, str);
impl_string_eq!(NbtString, &'a str);
impl_string_eq!(NbtString, Box<str>);
impl_string_eq!(NbtString, String);
impl_string_eq!(NbtString, Cow<'a, str>);

impl Eq for NbtString {}

impl PartialOrd for NbtString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NbtString {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl Hash for NbtString {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

/// TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuoteKind {
    /// TODO
    None,
    /// TODO
    Single,
    /// TODO
    Double,
}

impl Default for QuoteKind {
    /// Returns the default value of [`QuoteKind::Double`].
    #[inline]
    fn default() -> Self {
        QuoteKind::Double
    }
}

////////////////////////////////////////////////////////////////////////////////
// TAG_List
////////////////////////////////////////////////////////////////////////////////

/// TODO
pub enum NbtList {
    /// TODO
    Empty,
    /// TODO
    Byte(Vec<NbtByte>),
    /// TODO
    Short(Vec<i16>),
    /// TODO
    Int(Vec<i32>),
    /// TODO
    Long(Vec<i64>),
    /// TODO
    Float(Vec<f32>),
    /// TODO
    Double(Vec<f64>),
    /// TODO
    ByteArray(Vec<NbtByteArray>),
    /// TODO
    String(Vec<String>),
    /// TODO
    List(Vec<NbtList>),
    /// TODO
    Compound(Vec<NbtCompound>),
    /// TODO
    IntArray(Vec<NbtIntArray>),
    /// TODO
    LongArray(Vec<NbtLongArray>),
}

////////////////////////////////////////////////////////////////////////////////
// TAG_Compound
////////////////////////////////////////////////////////////////////////////////

/// TODO
pub type NbtCompound = IndexMap<NbtString, NbtValue, RandomState>;

////////////////////////////////////////////////////////////////////////////////
// TAG_Int_Array
////////////////////////////////////////////////////////////////////////////////

/// TODO
pub type NbtIntArray = Vec<i32>;

////////////////////////////////////////////////////////////////////////////////
// TAG_LongArray
////////////////////////////////////////////////////////////////////////////////

/// TODO
pub type NbtLongArray = Vec<i64>;
