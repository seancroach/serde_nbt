mod byte;
mod de;
mod from;
pub mod list;
pub mod map;

pub use self::byte::Byte;
use self::{list::List, map::Map};

use alloc::vec::Vec;

/// TODO
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// TODO
    Byte(Byte),
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
    ByteArray(ByteArray),
    ///TODO
    String(String),
    /// TODO
    List(List),
    /// TODO
    Compound(Compound),
    /// TODO
    IntArray(IntArray),
    /// TODO
    LongArray(LongArray),
}

impl Value {
    /// TODO
    #[must_use]
    #[inline]
    pub const fn ty(&self) -> Type {
        match self {
            Value::Byte(_) => Type::Byte,
            Value::Short(_) => Type::Short,
            Value::Int(_) => Type::Int,
            Value::Long(_) => Type::Long,
            Value::Float(_) => Type::Float,
            Value::Double(_) => Type::Double,
            Value::ByteArray(_) => Type::ByteArray,
            Value::String(_) => Type::String,
            Value::List(_) => Type::List,
            Value::Compound(_) => Type::Compound,
            Value::IntArray(_) => Type::IntArray,
            Value::LongArray(_) => Type::LongArray,
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // `is_*` Methods
    ////////////////////////////////////////////////////////////////////////////

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_byte(&self) -> bool {
        matches!(self, Value::Byte(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_short(&self) -> bool {
        matches!(self, Value::Short(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_int(&self) -> bool {
        matches!(self, Value::Int(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_long(&self) -> bool {
        matches!(self, Value::Long(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_double(&self) -> bool {
        matches!(self, Value::Double(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_byte_array(&self) -> bool {
        matches!(self, Value::ByteArray(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_list(&self) -> bool {
        matches!(self, Value::List(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_compound(&self) -> bool {
        matches!(self, Value::Compound(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_int_array(&self) -> bool {
        matches!(self, Value::IntArray(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_long_array(&self) -> bool {
        matches!(self, Value::LongArray(_))
    }

    ////////////////////////////////////////////////////////////////////////////
    // `as_*` Methods
    ////////////////////////////////////////////////////////////////////////////

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_byte(&self) -> Option<Byte> {
        if let Value::Byte(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_short(&self) -> Option<i16> {
        if let Value::Short(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_int(&self) -> Option<i32> {
        if let Value::Int(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_long(&self) -> Option<i64> {
        if let Value::Long(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_float(&self) -> Option<f32> {
        if let Value::Float(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_double(&self) -> Option<f64> {
        if let Value::Double(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_byte_array(&self) -> Option<&ByteArray> {
        if let Value::ByteArray(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_string(&self) -> Option<&String> {
        if let Value::String(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_list(&self) -> Option<&List> {
        if let Value::List(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_compound(&self) -> Option<&Compound> {
        if let Value::Compound(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_int_array(&self) -> Option<&IntArray> {
        if let Value::IntArray(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_long_array(&self) -> Option<&LongArray> {
        if let Value::LongArray(value) = self {
            Some(value)
        } else {
            None
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // `try_into_*` Methods
    ////////////////////////////////////////////////////////////////////////////

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_byte(self) -> Result<Byte, Self> {
        if let Value::Byte(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_short(self) -> Result<i16, Self> {
        if let Value::Short(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_int(self) -> Result<i32, Self> {
        if let Value::Int(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_long(self) -> Result<i64, Self> {
        if let Value::Long(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_float(self) -> Result<f32, Self> {
        if let Value::Float(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_double(self) -> Result<f64, Self> {
        if let Value::Double(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_byte_array(self) -> Result<ByteArray, Self> {
        if let Value::ByteArray(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_string(self) -> Result<String, Self> {
        if let Value::String(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_list(self) -> Result<List, Self> {
        if let Value::List(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_compound(self) -> Result<Compound, Self> {
        if let Value::Compound(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_int_array(self) -> Result<IntArray, Self> {
        if let Value::IntArray(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_long_array(self) -> Result<LongArray, Self> {
        if let Value::LongArray(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }
}

/// TODO
pub type Compound = Map<String, Value>;

/// TODO
pub type ByteArray = Vec<Byte>;

/// TODO
pub type IntArray = Vec<i32>;

/// TODO
pub type LongArray = Vec<i64>;

/// TODO
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
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
    ///TODO
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
