pub mod map;
pub mod ser;

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
    pub const fn kind(&self) -> ValueKind {
        match self {
            Value::Byte(_) => ValueKind::Byte,
            Value::Short(_) => ValueKind::Short,
            Value::Int(_) => ValueKind::Int,
            Value::Long(_) => ValueKind::Long,
            Value::Float(_) => ValueKind::Float,
            Value::Double(_) => ValueKind::Double,
            Value::ByteArray(_) => ValueKind::ByteArray,
            Value::String(_) => ValueKind::String,
            Value::List(_) => ValueKind::List,
            Value::Compound(_) => ValueKind::Compound,
            Value::IntArray(_) => ValueKind::IntArray,
            Value::LongArray(_) => ValueKind::LongArray,
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
    pub fn with_capacity_and_kind(capacity: usize, kind: ValueKind) -> Self {
        match kind {
            ValueKind::Byte => List::Byte(Vec::with_capacity(capacity)),
            ValueKind::Short => List::Short(Vec::with_capacity(capacity)),
            ValueKind::Int => List::Int(Vec::with_capacity(capacity)),
            ValueKind::Long => List::Long(Vec::with_capacity(capacity)),
            ValueKind::Float => List::Float(Vec::with_capacity(capacity)),
            ValueKind::Double => List::Double(Vec::with_capacity(capacity)),
            ValueKind::ByteArray => List::ByteArray(Vec::with_capacity(capacity)),
            ValueKind::String => List::String(Vec::with_capacity(capacity)),
            ValueKind::List => List::List(Vec::with_capacity(capacity)),
            ValueKind::Compound => List::Compound(Vec::with_capacity(capacity)),
            ValueKind::IntArray => List::IntArray(Vec::with_capacity(capacity)),
            ValueKind::LongArray => List::LongArray(Vec::with_capacity(capacity)),
        }
    }

    #[must_use]
    #[inline]
    pub fn from_kind(kind: ValueKind) -> Self {
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
        self.push_checked(value).unwrap_unchecked()
    }

    /// TODO
    ///
    /// # Panics
    #[track_caller]
    pub fn push(&mut self, value: Value) {
        self.push_checked(value)
            .expect("the provided `Value` should have been the same datatype as the `List`")
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
pub enum ValueKind {
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

impl ValueKind {
    #[must_use]
    #[inline]
    pub fn to_id(self) -> Id {
        match self {
            ValueKind::Byte => Id::Byte,
            ValueKind::Short => Id::Short,
            ValueKind::Int => Id::Int,
            ValueKind::Long => Id::Long,
            ValueKind::Float => Id::Float,
            ValueKind::Double => Id::Double,
            ValueKind::ByteArray => Id::ByteArray,
            ValueKind::String => Id::String,
            ValueKind::List => Id::List,
            ValueKind::Compound => Id::Compound,
            ValueKind::IntArray => Id::IntArray,
            ValueKind::LongArray => Id::LongArray,
        }
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
    pub fn as_str(self) -> &'static str {
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
    pub fn to_kind(self) -> Option<ValueKind> {
        match self {
            Id::End => None,
            Id::Byte => Some(ValueKind::Byte),
            Id::Short => Some(ValueKind::Short),
            Id::Int => Some(ValueKind::Int),
            Id::Long => Some(ValueKind::Long),
            Id::Float => Some(ValueKind::Float),
            Id::Double => Some(ValueKind::Double),
            Id::ByteArray => Some(ValueKind::ByteArray),
            Id::String => Some(ValueKind::String),
            Id::List => Some(ValueKind::List),
            Id::Compound => Some(ValueKind::Compound),
            Id::IntArray => Some(ValueKind::IntArray),
            Id::LongArray => Some(ValueKind::LongArray),
        }
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

pub type Compound = map::Map<String, Value>;

pub type ByteArray = Vec<Byte>;
pub type IntArray = Vec<i32>;
pub type LongArray = Vec<i64>;
