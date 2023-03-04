use super::{Byte, ByteArray, Compound, IntArray, List, LongArray, Value};

use alloc::{borrow::Cow, boxed::Box, string::String};

////////////////////////////////////////////////////////////////////////////////
// Simple Conversions
////////////////////////////////////////////////////////////////////////////////

impl From<i16> for Value {
    #[inline]
    fn from(value: i16) -> Self {
        Value::Short(value)
    }
}

impl From<i32> for Value {
    #[inline]
    fn from(value: i32) -> Self {
        Value::Int(value)
    }
}

impl From<i64> for Value {
    #[inline]
    fn from(value: i64) -> Self {
        Value::Long(value)
    }
}

impl From<f32> for Value {
    #[inline]
    fn from(value: f32) -> Self {
        Value::Float(value)
    }
}

impl From<f64> for Value {
    #[inline]
    fn from(value: f64) -> Self {
        Value::Double(value)
    }
}

impl From<List> for Value {
    #[inline]
    fn from(value: List) -> Self {
        Value::List(value)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Byte Conversions
////////////////////////////////////////////////////////////////////////////////

impl From<bool> for Value {
    #[inline]
    fn from(value: bool) -> Self {
        Value::Byte(value.into())
    }
}

impl From<i8> for Value {
    #[inline]
    fn from(value: i8) -> Self {
        Value::Byte(value.into())
    }
}

impl From<Byte> for Value {
    #[inline]
    fn from(value: Byte) -> Self {
        Value::Byte(value)
    }
}

////////////////////////////////////////////////////////////////////////////////
// String Conversions
////////////////////////////////////////////////////////////////////////////////

impl From<String> for Value {
    #[inline]
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&String> for Value {
    #[inline]
    fn from(value: &String) -> Self {
        Value::String(value.clone())
    }
}

impl From<char> for Value {
    #[inline]
    fn from(value: char) -> Self {
        Value::String(value.to_string())
    }
}

impl From<&str> for Value {
    #[inline]
    fn from(value: &str) -> Self {
        Value::String(value.to_owned())
    }
}

impl From<&mut str> for Value {
    #[inline]
    fn from(value: &mut str) -> Self {
        Value::String(value.to_owned())
    }
}

impl From<Box<str>> for Value {
    #[inline]
    fn from(value: Box<str>) -> Self {
        Value::String(value.into_string())
    }
}

impl From<Cow<'_, str>> for Value {
    #[inline]
    fn from(value: Cow<'_, str>) -> Self {
        Value::String(value.into_owned())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Array Conversions
////////////////////////////////////////////////////////////////////////////////

impl From<ByteArray> for Value {
    #[inline]
    fn from(value: ByteArray) -> Self {
        Value::ByteArray(value)
    }
}

impl From<&[bool]> for Value {
    #[inline]
    fn from(value: &[bool]) -> Self {
        let array = value.iter().copied().map(Byte::from).collect();
        Value::ByteArray(array)
    }
}

impl From<&[i8]> for Value {
    #[inline]
    fn from(value: &[i8]) -> Self {
        let array = value.iter().copied().map(Byte::from).collect();
        Value::ByteArray(array)
    }
}

impl From<IntArray> for Value {
    #[inline]
    fn from(value: IntArray) -> Self {
        Value::IntArray(value)
    }
}

impl From<&[i32]> for Value {
    #[inline]
    fn from(value: &[i32]) -> Self {
        Value::IntArray(value.to_vec())
    }
}

impl From<LongArray> for Value {
    #[inline]
    fn from(value: LongArray) -> Self {
        Value::LongArray(value)
    }
}

impl From<&[i64]> for Value {
    #[inline]
    fn from(value: &[i64]) -> Self {
        Value::LongArray(value.to_vec())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Compound Conversions
////////////////////////////////////////////////////////////////////////////////

impl From<Compound> for Value {
    #[inline]
    fn from(value: Compound) -> Self {
        Value::Compound(value)
    }
}

impl<K, V> FromIterator<(K, V)> for Value
where
    K: Into<String>,
    V: Into<Value>,
{
    #[inline]
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let compound = iter
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        Value::Compound(compound)
    }
}
