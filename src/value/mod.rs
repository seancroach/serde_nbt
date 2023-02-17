//! TODO

pub mod ser;
mod map;

use core::fmt;

use ahash::RandomState;
use indexmap::IndexMap;

/// TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Id {
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

impl Id {
    /// TODO
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

    /// TODO
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

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug)]
pub enum Byte {
    Bool(bool),
    I8(i8),
}

#[derive(Debug)]
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
    #[inline]
    pub fn id(&self) -> Id {
        match self {
            Value::Byte(_) => Id::Byte,
            Value::Short(_) => Id::Short,
            Value::Int(_) => Id::Int,
            Value::Long(_) => Id::Long,
            Value::Float(_) => Id::Float,
            Value::Double(_) => Id::Double,
            Value::ByteArray(_) => Id::ByteArray,
            Value::String(_) => Id::String,
            Value::List(_) => Id::List,
            Value::Compound(_) => Id::Compound,
            Value::IntArray(_) => Id::IntArray,
            Value::LongArray(_) => Id::LongArray,
        }
    }
}

#[derive(Debug, Default)]
pub enum List {
    #[default]
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
    #[inline]
    pub fn with_capacity_and_id(capacity: usize, id: Id) -> Self {
        match id {
            Id::End => List::Empty,
            Id::Byte => List::Byte(Vec::with_capacity(capacity)),
            Id::Short => List::Short(Vec::with_capacity(capacity)),
            Id::Int => List::Int(Vec::with_capacity(capacity)),
            Id::Long => List::Long(Vec::with_capacity(capacity)),
            Id::Float => List::Float(Vec::with_capacity(capacity)),
            Id::Double => List::Double(Vec::with_capacity(capacity)),
            Id::ByteArray => List::ByteArray(Vec::with_capacity(capacity)),
            Id::String => List::String(Vec::with_capacity(capacity)),
            Id::List => List::List(Vec::with_capacity(capacity)),
            Id::Compound => List::Compound(Vec::with_capacity(capacity)),
            Id::IntArray => List::IntArray(Vec::with_capacity(capacity)),
            Id::LongArray => List::LongArray(Vec::with_capacity(capacity)),
        }
    }

    #[must_use]
    #[inline]
    pub fn from_value(value: Value) -> Self {
        match value {
            Value::Byte(v) => List::Byte(vec![v]),
            Value::Short(v) => List::Short(vec![v]),
            Value::Int(v) => List::Int(vec![v]),
            Value::Long(v) => List::Long(vec![v]),
            Value::Float(v) => List::Float(vec![v]),
            Value::Double(v) => List::Double(vec![v]),
            Value::ByteArray(v) => List::ByteArray(vec![v]),
            Value::String(v) => List::String(vec![v]),
            Value::List(v) => List::List(vec![v]),
            Value::Compound(v) => List::Compound(vec![v]),
            Value::IntArray(v) => List::IntArray(vec![v]),
            Value::LongArray(v) => List::LongArray(vec![v]),
        }
    }

    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            List::Empty => 0,
            List::Byte(vec) => vec.len(),
            List::Short(vec) => vec.len(),
            List::Int(vec) => vec.len(),
            List::Long(vec) => vec.len(),
            List::Float(vec) => vec.len(),
            List::Double(vec) => vec.len(),
            List::ByteArray(vec) => vec.len(),
            List::String(vec) => vec.len(),
            List::List(vec) => vec.len(),
            List::Compound(vec) => vec.len(),
            List::IntArray(vec) => vec.len(),
            List::LongArray(vec) => vec.len(),
        }
    }

    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    #[inline]
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

    /// TODO
    ///
    /// # Errors
    ///
    /// TODO
    pub fn push(&mut self, value: Value) -> Result<(), Value> {
        if self.is_empty() {
            *self = List::from_value(value);
            return Ok(());
        }

        match (self, value) {
            (List::Empty, _) => unreachable!(),
            (List::Byte(vec), Value::Byte(v)) => vec.push(v),
            (List::Short(vec), Value::Short(v)) => vec.push(v),
            (List::Int(vec), Value::Int(v)) => vec.push(v),
            (List::Long(vec), Value::Long(v)) => vec.push(v),
            (List::Float(vec), Value::Float(v)) => vec.push(v),
            (List::Double(vec), Value::Double(v)) => vec.push(v),
            (List::ByteArray(vec), Value::ByteArray(v)) => vec.push(v),
            (List::String(vec), Value::String(v)) => vec.push(v),
            (List::List(vec), Value::List(v)) => vec.push(v),
            (List::Compound(vec), Value::Compound(v)) => vec.push(v),
            (List::IntArray(vec), Value::IntArray(v)) => vec.push(v),
            (List::LongArray(vec), Value::LongArray(v)) => vec.push(v),
            (_, value) => return Err(value),
        }

        Ok(())
    }
}

pub type ByteArray = Vec<Byte>;
pub type IntArray = Vec<i32>;
pub type LongArray = Vec<i64>;

pub type Compound = IndexMap<String, Value, RandomState>;
