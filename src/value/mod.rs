#![allow(clippy::module_name_repetitions)]

pub(crate) mod de;
pub(crate) mod ser;

use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Byte(Byte),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(ByteArray),
    String(String),
    List(List),
    Compound(Compound),
    IntArray(IntArray),
    LongArray(LongArray),
}

impl Value {
    #[must_use]
    pub const fn kind(&self) -> Kind {
        match self {
            Value::Byte(_) => Kind::Byte,
            Value::Short(_) => Kind::Short,
            Value::Int(_) => Kind::Int,
            Value::Long(_) => Kind::Long,
            Value::Float(_) => Kind::Float,
            Value::Double(_) => Kind::Double,
            Value::ByteArray(_) => Kind::ByteArray,
            Value::String(_) => Kind::String,
            Value::List(_) => Kind::List,
            Value::Compound(_) => Kind::Compound,
            Value::IntArray(_) => Kind::IntArray,
            Value::LongArray(_) => Kind::LongArray,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum List {
    Empty,
    Byte(Vec<Byte>),
    Short(Vec<i16>),
    Int(Vec<i32>),
    Long(Vec<i64>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    ByteArray(Vec<ByteArray>),
    String(Vec<String>),
    List(Vec<List>),
    Compound(Vec<Compound>),
    IntArray(Vec<IntArray>),
    LongArray(Vec<LongArray>),
}

impl List {
    #[must_use]
    pub fn with_capacity_and_kind(capacity: usize, kind: Kind) -> Self {
        match kind {
            Kind::Byte => List::Byte(Vec::with_capacity(capacity)),
            Kind::Short => List::Short(Vec::with_capacity(capacity)),
            Kind::Int => List::Int(Vec::with_capacity(capacity)),
            Kind::Long => List::Long(Vec::with_capacity(capacity)),
            Kind::Float => List::Float(Vec::with_capacity(capacity)),
            Kind::Double => List::Double(Vec::with_capacity(capacity)),
            Kind::ByteArray => List::ByteArray(Vec::with_capacity(capacity)),
            Kind::String => List::String(Vec::with_capacity(capacity)),
            Kind::List => List::List(Vec::with_capacity(capacity)),
            Kind::Compound => List::Compound(Vec::with_capacity(capacity)),
            Kind::IntArray => List::IntArray(Vec::with_capacity(capacity)),
            Kind::LongArray => List::LongArray(Vec::with_capacity(capacity)),
        }
    }

    #[must_use]
    #[inline]
    pub fn from_kind(kind: Kind) -> Self {
        Self::with_capacity_and_kind(0, kind)
    }

    #[must_use]
    pub fn from_capacity_and_value(value: Value) -> Self {
        let mut list = Self::with_capacity_and_kind(1, value.kind());
        unsafe { list.push_unchecked(value) };
        list
    }

    #[must_use]
    pub fn id(&self) -> Id {
        match self {
            List::Empty => Id::End,
            List::Byte(_) => Id::Byte,
            List::Short(_) => Id::Short,
            List::Int(_) => Id::Int,
            List::Long(_) => Id::Long,
            List::Float(_) => Id::Float,
            List::Double(_) => Id::Double,
            List::ByteArray(_) => Id::ByteArray,
            List::String(_) => Id::String,
            List::List(_) => Id::List,
            List::Compound(_) => Id::Compound,
            List::IntArray(_) => Id::IntArray,
            List::LongArray(_) => Id::LongArray,
        }
    }

    #[track_caller]
    pub unsafe fn push_unchecked(&mut self, value: Value) {
        debug_assert_eq!(self.id(), value.kind().to_id());
        self.push_checked(value).unwrap_unchecked();
    }

    /// TODO
    ///
    /// # Panics
    #[track_caller]
    pub fn push(&mut self, value: Value) {
        self.push_checked(value)
            .expect("the provided `Value` should have been the same datatype as the `List`");
    }

    /// TODO
    ///
    /// # Panics
    ///
    /// TODO
    ///
    /// # Errors
    ///
    /// TODO
    #[track_caller]
    pub fn push_checked(&mut self, value: Value) -> Result<(), Value> {
        match (self, value) {
            (List::Byte(vec), Value::Byte(value)) => vec.push(value),
            (List::Short(vec), Value::Short(value)) => vec.push(value),
            (List::Int(vec), Value::Int(value)) => vec.push(value),
            (List::Long(vec), Value::Long(value)) => vec.push(value),
            (List::Float(vec), Value::Float(value)) => vec.push(value),
            (List::Double(vec), Value::Double(value)) => vec.push(value),
            (List::ByteArray(vec), Value::ByteArray(value)) => vec.push(value),
            (List::String(vec), Value::String(value)) => vec.push(value),
            (List::List(vec), Value::List(value)) => vec.push(value),
            (List::Compound(vec), Value::Compound(value)) => vec.push(value),
            (List::IntArray(vec), Value::IntArray(value)) => vec.push(value),
            (List::LongArray(vec), Value::LongArray(value)) => vec.push(value),
            (_, value) => return Err(value),
        }

        Ok(())
    }
}

impl Default for List {
    /// Returns the default value [`List::Empty`].
    #[inline]
    fn default() -> Self {
        List::Empty
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Byte {
    Boolean(bool),
    Integer(i8),
}

impl Default for Byte {
    /// Returns the default value [`Byte::Integer(0)`].
    #[inline]
    fn default() -> Self {
        Byte::Integer(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    String,
    List,
    Compound,
    IntArray,
    LongArray,
}

impl Kind {
    #[must_use]
    #[inline]
    pub fn to_id(self) -> Id {
        match self {
            Kind::Byte => Id::Byte,
            Kind::Short => Id::Short,
            Kind::Int => Id::Int,
            Kind::Long => Id::Long,
            Kind::Float => Id::Float,
            Kind::Double => Id::Double,
            Kind::ByteArray => Id::ByteArray,
            Kind::String => Id::String,
            Kind::List => Id::List,
            Kind::Compound => Id::Compound,
            Kind::IntArray => Id::IntArray,
            Kind::LongArray => Id::LongArray,
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_id().fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Id {
    End,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    String,
    List,
    Compound,
    IntArray,
    LongArray,
}

impl Id {
    #[must_use]
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Id::End => "TAG_End",
            Id::Byte => "TAG_Byte",
            Id::Short => "TAG_Short",
            Id::Int => "TAG_Int",
            Id::Long => "TAG_Long",
            Id::Float => "TAG_Float",
            Id::Double => "TAG_Double",
            Id::ByteArray => "TAG_Byte_Array",
            Id::String => "TAG_String",
            Id::List => "TAG_List",
            Id::Compound => "TAG_Compound",
            Id::IntArray => "TAG_Int_Array",
            Id::LongArray => "TAG_Long_Array",
        }
    }

    #[must_use]
    #[inline]
    pub const fn from_u8(value: u8) -> Option<Self> {
        let id = match value {
            0x00 => Id::End,
            0x01 => Id::Byte,
            0x02 => Id::Short,
            0x03 => Id::Int,
            0x04 => Id::Long,
            0x05 => Id::Float,
            0x06 => Id::Double,
            0x07 => Id::ByteArray,
            0x08 => Id::String,
            0x09 => Id::List,
            0x0A => Id::Compound,
            0x0B => Id::IntArray,
            0x0C => Id::LongArray,
            _ => return None,
        };
        Some(id)
    }

    #[must_use]
    #[inline]
    pub const fn to_kind(self) -> Option<Kind> {
        match self {
            Id::End => None,
            Id::Byte => Some(Kind::Byte),
            Id::Short => Some(Kind::Short),
            Id::Int => Some(Kind::Int),
            Id::Long => Some(Kind::Long),
            Id::Float => Some(Kind::Float),
            Id::Double => Some(Kind::Double),
            Id::ByteArray => Some(Kind::ByteArray),
            Id::String => Some(Kind::String),
            Id::List => Some(Kind::List),
            Id::Compound => Some(Kind::Compound),
            Id::IntArray => Some(Kind::IntArray),
            Id::LongArray => Some(Kind::LongArray),
        }
    }

    #[must_use]
    #[inline]
    pub const fn to_u8(self) -> u8 {
        match self {
            Id::End => 0x00,
            Id::Byte => 0x01,
            Id::Short => 0x02,
            Id::Int => 0x03,
            Id::Long => 0x04,
            Id::Float => 0x05,
            Id::Double => 0x06,
            Id::ByteArray => 0x07,
            Id::String => 0x08,
            Id::List => 0x09,
            Id::Compound => 0x0A,
            Id::IntArray => 0x0B,
            Id::LongArray => 0x0C,
        }
    }
}

impl TryFrom<u8> for Id {
    type Error = u8;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Id::from_u8(value).ok_or(value)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Default for Id {
    /// Returns the default value [`Id::End`].
    #[inline]
    fn default() -> Self {
        Id::End
    }
}

pub type Compound = crate::map::Map<String, Value>;

pub type ByteArray = Vec<Byte>;
pub type IntArray = Vec<i32>;
pub type LongArray = Vec<i64>;
