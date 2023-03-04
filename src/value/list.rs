//! TODO

use super::{Byte, ByteArray, Compound, IntArray, LongArray, Type, Value};

use alloc::{
    collections::TryReserveError,
    vec::{self, Vec},
};
use core::{fmt, iter::FusedIterator, mem};

/// TODO
#[derive(Debug, Clone, PartialEq)]
pub enum List {
    /// TODO
    Empty,
    /// TODO
    Byte(Vec<Byte>),
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
    ByteArray(Vec<ByteArray>),
    /// TODO
    String(Vec<String>),
    /// TODO
    List(Vec<List>),
    /// TODO
    Compound(Vec<Compound>),
    /// TODO
    IntArray(Vec<IntArray>),
    /// TODO
    LongArray(Vec<LongArray>),
}

impl List {
    /// TODO
    ///
    /// # Panics
    ///
    /// TODO
    #[must_use]
    #[track_caller]
    pub fn from_vec(vec: Vec<Value>) -> Self {
        match vec.first().map(Value::ty) {
            Some(Type::Byte) => {
                let result: Result<Vec<_>, _> = vec.into_iter().map(Value::try_into_byte).collect();
                let vec = result.expect("heterogeneous `Vec`");
                List::Byte(vec)
            }
            Some(Type::Short) => {
                let result: Result<Vec<_>, _> =
                    vec.into_iter().map(Value::try_into_short).collect();
                let vec = result.expect("heterogeneous `Vec`");
                List::Short(vec)
            }
            Some(Type::Int) => {
                let result: Result<Vec<_>, _> = vec.into_iter().map(Value::try_into_int).collect();
                let vec = result.expect("heterogeneous `Vec`");
                List::Int(vec)
            }
            Some(Type::Long) => {
                let result: Result<Vec<_>, _> = vec.into_iter().map(Value::try_into_long).collect();
                let vec = result.expect("heterogeneous `Vec`");
                List::Long(vec)
            }
            Some(Type::Float) => {
                let result: Result<Vec<_>, _> =
                    vec.into_iter().map(Value::try_into_float).collect();
                let vec = result.expect("heterogeneous `Vec`");
                List::Float(vec)
            }
            Some(Type::Double) => {
                let result: Result<Vec<_>, _> =
                    vec.into_iter().map(Value::try_into_double).collect();
                let vec = result.expect("heterogeneous `Vec`");
                List::Double(vec)
            }
            Some(Type::ByteArray) => {
                let result: Result<Vec<_>, _> =
                    vec.into_iter().map(Value::try_into_byte_array).collect();
                let vec = result.expect("heterogeneous `Vec`");
                List::ByteArray(vec)
            }
            Some(Type::String) => {
                let result: Result<Vec<_>, _> =
                    vec.into_iter().map(Value::try_into_string).collect();
                let vec = result.expect("heterogeneous `Vec`");
                List::String(vec)
            }
            Some(Type::List) => {
                let result: Result<Vec<_>, _> = vec.into_iter().map(Value::try_into_list).collect();
                let vec = result.expect("heterogeneous `Vec`");
                List::List(vec)
            }
            Some(Type::Compound) => {
                let result: Result<Vec<_>, _> =
                    vec.into_iter().map(Value::try_into_compound).collect();
                let vec = result.expect("heterogeneous `Vec`");
                List::Compound(vec)
            }
            Some(Type::IntArray) => {
                let result: Result<Vec<_>, _> =
                    vec.into_iter().map(Value::try_into_int_array).collect();
                let vec = result.expect("heterogeneous `Vec`");
                List::IntArray(vec)
            }
            Some(Type::LongArray) => {
                let result: Result<Vec<_>, _> =
                    vec.into_iter().map(Value::try_into_long_array).collect();
                let vec = result.expect("heterogeneous `Vec`");
                List::LongArray(vec)
            }
            None => List::Empty,
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub fn with_capacity_and_type(capacity: usize, ty: Type) -> Self {
        match ty {
            Type::Byte => List::Byte(Vec::with_capacity(capacity)),
            Type::Short => List::Short(Vec::with_capacity(capacity)),
            Type::Int => List::Int(Vec::with_capacity(capacity)),
            Type::Long => List::Long(Vec::with_capacity(capacity)),
            Type::Float => List::Float(Vec::with_capacity(capacity)),
            Type::Double => List::Double(Vec::with_capacity(capacity)),
            Type::ByteArray => List::ByteArray(Vec::with_capacity(capacity)),
            Type::String => List::String(Vec::with_capacity(capacity)),
            Type::List => List::List(Vec::with_capacity(capacity)),
            Type::Compound => List::Compound(Vec::with_capacity(capacity)),
            Type::IntArray => List::IntArray(Vec::with_capacity(capacity)),
            Type::LongArray => List::LongArray(Vec::with_capacity(capacity)),
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub fn ty(&self) -> Option<Type> {
        match self {
            List::Empty => None,
            List::Byte(_) => Some(Type::Byte),
            List::Short(_) => Some(Type::Short),
            List::Int(_) => Some(Type::Int),
            List::Long(_) => Some(Type::Long),
            List::Float(_) => Some(Type::Float),
            List::Double(_) => Some(Type::Double),
            List::ByteArray(_) => Some(Type::ByteArray),
            List::String(_) => Some(Type::String),
            List::List(_) => Some(Type::List),
            List::Compound(_) => Some(Type::Compound),
            List::IntArray(_) => Some(Type::IntArray),
            List::LongArray(_) => Some(Type::LongArray),
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // `is_*` Methods
    ////////////////////////////////////////////////////////////////////////////

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_byte(&self) -> bool {
        matches!(self, List::Byte(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_short(&self) -> bool {
        matches!(self, List::Short(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_int(&self) -> bool {
        matches!(self, List::Int(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_long(&self) -> bool {
        matches!(self, List::Long(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_float(&self) -> bool {
        matches!(self, List::Float(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_double(&self) -> bool {
        matches!(self, List::Double(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_byte_array(&self) -> bool {
        matches!(self, List::ByteArray(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_string(&self) -> bool {
        matches!(self, List::String(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_list(&self) -> bool {
        matches!(self, List::List(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_compound(&self) -> bool {
        matches!(self, List::Compound(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_int_array(&self) -> bool {
        matches!(self, List::IntArray(_))
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn is_long_array(&self) -> bool {
        matches!(self, List::LongArray(_))
    }

    ////////////////////////////////////////////////////////////////////////////
    // `as_*` Methods
    ////////////////////////////////////////////////////////////////////////////

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_byte(&self) -> Option<&Vec<Byte>> {
        if let List::Byte(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_short(&self) -> Option<&Vec<i16>> {
        if let List::Short(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_int(&self) -> Option<&Vec<i32>> {
        if let List::Int(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_long(&self) -> Option<&Vec<i64>> {
        if let List::Long(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_float(&self) -> Option<&Vec<f32>> {
        if let List::Float(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_double(&self) -> Option<&Vec<f64>> {
        if let List::Double(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_byte_array(&self) -> Option<&Vec<ByteArray>> {
        if let List::ByteArray(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_string(&self) -> Option<&Vec<String>> {
        if let List::String(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_list(&self) -> Option<&Vec<List>> {
        if let List::List(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_compound(&self) -> Option<&Vec<Compound>> {
        if let List::Compound(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_int_array(&self) -> Option<&Vec<IntArray>> {
        if let List::IntArray(value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn as_long_array(&self) -> Option<&Vec<LongArray>> {
        if let List::LongArray(value) = self {
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
    pub fn try_into_byte(self) -> Result<Vec<Byte>, Self> {
        if let List::Byte(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_short(self) -> Result<Vec<i16>, Self> {
        if let List::Short(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_int(self) -> Result<Vec<i32>, Self> {
        if let List::Int(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_long(self) -> Result<Vec<i64>, Self> {
        if let List::Long(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_float(self) -> Result<Vec<f32>, Self> {
        if let List::Float(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_double(self) -> Result<Vec<f64>, Self> {
        if let List::Double(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_byte_array(self) -> Result<Vec<ByteArray>, Self> {
        if let List::ByteArray(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_string(self) -> Result<Vec<String>, Self> {
        if let List::String(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_list(self) -> Result<Vec<List>, Self> {
        if let List::List(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_compound(self) -> Result<Vec<Compound>, Self> {
        if let List::Compound(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_int_array(self) -> Result<Vec<IntArray>, Self> {
        if let List::IntArray(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// TODO
    #[allow(clippy::missing_errors_doc)]
    #[inline]
    pub fn try_into_long_array(self) -> Result<Vec<LongArray>, Self> {
        if let List::LongArray(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // `Vec`-ish methods
    ////////////////////////////////////////////////////////////////////////////

    /// TODO
    #[must_use]
    #[inline]
    pub fn capacity(&self) -> usize {
        match self {
            List::Empty => 0,
            List::Byte(vec) => vec.capacity(),
            List::Short(vec) => vec.capacity(),
            List::Int(vec) => vec.capacity(),
            List::Long(vec) => vec.capacity(),
            List::Float(vec) => vec.capacity(),
            List::Double(vec) => vec.capacity(),
            List::ByteArray(vec) => vec.capacity(),
            List::String(vec) => vec.capacity(),
            List::List(vec) => vec.capacity(),
            List::Compound(vec) => vec.capacity(),
            List::IntArray(vec) => vec.capacity(),
            List::LongArray(vec) => vec.capacity(),
        }
    }

    /// TODO
    ///
    /// # Panics
    ///
    /// TODO
    #[track_caller]
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        match self {
            List::Empty => {}
            List::Byte(vec) => vec.reserve(additional),
            List::Short(vec) => vec.reserve(additional),
            List::Int(vec) => vec.reserve(additional),
            List::Long(vec) => vec.reserve(additional),
            List::Float(vec) => vec.reserve(additional),
            List::Double(vec) => vec.reserve(additional),
            List::ByteArray(vec) => vec.reserve(additional),
            List::String(vec) => vec.reserve(additional),
            List::List(vec) => vec.reserve(additional),
            List::Compound(vec) => vec.reserve(additional),
            List::IntArray(vec) => vec.reserve(additional),
            List::LongArray(vec) => vec.reserve(additional),
        }
    }

    /// TODO
    ///
    /// # Panics
    ///
    /// TODO
    #[track_caller]
    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        match self {
            List::Empty => {}
            List::Byte(vec) => vec.reserve_exact(additional),
            List::Short(vec) => vec.reserve_exact(additional),
            List::Int(vec) => vec.reserve_exact(additional),
            List::Long(vec) => vec.reserve_exact(additional),
            List::Float(vec) => vec.reserve_exact(additional),
            List::Double(vec) => vec.reserve_exact(additional),
            List::ByteArray(vec) => vec.reserve_exact(additional),
            List::String(vec) => vec.reserve_exact(additional),
            List::List(vec) => vec.reserve_exact(additional),
            List::Compound(vec) => vec.reserve_exact(additional),
            List::IntArray(vec) => vec.reserve_exact(additional),
            List::LongArray(vec) => vec.reserve_exact(additional),
        }
    }

    /// TODO
    ///
    /// # Errors
    ///
    /// TODO
    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        match self {
            List::Empty => Ok(()),
            List::Byte(vec) => vec.try_reserve(additional),
            List::Short(vec) => vec.try_reserve(additional),
            List::Int(vec) => vec.try_reserve(additional),
            List::Long(vec) => vec.try_reserve(additional),
            List::Float(vec) => vec.try_reserve(additional),
            List::Double(vec) => vec.try_reserve(additional),
            List::ByteArray(vec) => vec.try_reserve(additional),
            List::String(vec) => vec.try_reserve(additional),
            List::List(vec) => vec.try_reserve(additional),
            List::Compound(vec) => vec.try_reserve(additional),
            List::IntArray(vec) => vec.try_reserve(additional),
            List::LongArray(vec) => vec.try_reserve(additional),
        }
    }

    /// TODO
    ///
    /// # Errors
    ///
    /// TODO
    #[inline]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        match self {
            List::Empty => Ok(()),
            List::Byte(vec) => vec.try_reserve_exact(additional),
            List::Short(vec) => vec.try_reserve_exact(additional),
            List::Int(vec) => vec.try_reserve_exact(additional),
            List::Long(vec) => vec.try_reserve_exact(additional),
            List::Float(vec) => vec.try_reserve_exact(additional),
            List::Double(vec) => vec.try_reserve_exact(additional),
            List::ByteArray(vec) => vec.try_reserve_exact(additional),
            List::String(vec) => vec.try_reserve_exact(additional),
            List::List(vec) => vec.try_reserve_exact(additional),
            List::Compound(vec) => vec.try_reserve_exact(additional),
            List::IntArray(vec) => vec.try_reserve_exact(additional),
            List::LongArray(vec) => vec.try_reserve_exact(additional),
        }
    }

    /// TODO
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        match self {
            List::Empty => {}
            List::Byte(vec) => vec.shrink_to_fit(),
            List::Short(vec) => vec.shrink_to_fit(),
            List::Int(vec) => vec.shrink_to_fit(),
            List::Long(vec) => vec.shrink_to_fit(),
            List::Float(vec) => vec.shrink_to_fit(),
            List::Double(vec) => vec.shrink_to_fit(),
            List::ByteArray(vec) => vec.shrink_to_fit(),
            List::String(vec) => vec.shrink_to_fit(),
            List::List(vec) => vec.shrink_to_fit(),
            List::Compound(vec) => vec.shrink_to_fit(),
            List::IntArray(vec) => vec.shrink_to_fit(),
            List::LongArray(vec) => vec.shrink_to_fit(),
        }
    }

    /// TODO
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        match self {
            List::Empty => {}
            List::Byte(vec) => vec.shrink_to(min_capacity),
            List::Short(vec) => vec.shrink_to(min_capacity),
            List::Int(vec) => vec.shrink_to(min_capacity),
            List::Long(vec) => vec.shrink_to(min_capacity),
            List::Float(vec) => vec.shrink_to(min_capacity),
            List::Double(vec) => vec.shrink_to(min_capacity),
            List::ByteArray(vec) => vec.shrink_to(min_capacity),
            List::String(vec) => vec.shrink_to(min_capacity),
            List::List(vec) => vec.shrink_to(min_capacity),
            List::Compound(vec) => vec.shrink_to(min_capacity),
            List::IntArray(vec) => vec.shrink_to(min_capacity),
            List::LongArray(vec) => vec.shrink_to(min_capacity),
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub fn into_vec(self) -> Vec<Value> {
        match self {
            List::Empty => Vec::new(),
            List::Byte(vec) => vec.into_iter().map(Value::Byte).collect(),
            List::Short(vec) => vec.into_iter().map(Value::Short).collect(),
            List::Int(vec) => vec.into_iter().map(Value::Int).collect(),
            List::Long(vec) => vec.into_iter().map(Value::Long).collect(),
            List::Float(vec) => vec.into_iter().map(Value::Float).collect(),
            List::Double(vec) => vec.into_iter().map(Value::Double).collect(),
            List::ByteArray(vec) => vec.into_iter().map(Value::ByteArray).collect(),
            List::String(vec) => vec.into_iter().map(Value::String).collect(),
            List::List(vec) => vec.into_iter().map(Value::List).collect(),
            List::Compound(vec) => vec.into_iter().map(Value::Compound).collect(),
            List::IntArray(vec) => vec.into_iter().map(Value::IntArray).collect(),
            List::LongArray(vec) => vec.into_iter().map(Value::LongArray).collect(),
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub fn into_boxed_slice(self) -> Box<[Value]> {
        self.into_vec().into_boxed_slice()
    }

    /// TODO
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        match self {
            List::Empty => {}
            List::Byte(vec) => vec.truncate(len),
            List::Short(vec) => vec.truncate(len),
            List::Int(vec) => vec.truncate(len),
            List::Long(vec) => vec.truncate(len),
            List::Float(vec) => vec.truncate(len),
            List::Double(vec) => vec.truncate(len),
            List::ByteArray(vec) => vec.truncate(len),
            List::String(vec) => vec.truncate(len),
            List::List(vec) => vec.truncate(len),
            List::Compound(vec) => vec.truncate(len),
            List::IntArray(vec) => vec.truncate(len),
            List::LongArray(vec) => vec.truncate(len),
        }
    }

    /// TODO
    ///
    /// # Safety
    ///
    /// TODO
    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.capacity());

        match self {
            List::Empty => {}
            List::Byte(vec) => vec.set_len(new_len),
            List::Short(vec) => vec.set_len(new_len),
            List::Int(vec) => vec.set_len(new_len),
            List::Long(vec) => vec.set_len(new_len),
            List::Float(vec) => vec.set_len(new_len),
            List::Double(vec) => vec.set_len(new_len),
            List::ByteArray(vec) => vec.set_len(new_len),
            List::String(vec) => vec.set_len(new_len),
            List::List(vec) => vec.set_len(new_len),
            List::Compound(vec) => vec.set_len(new_len),
            List::IntArray(vec) => vec.set_len(new_len),
            List::LongArray(vec) => vec.set_len(new_len),
        }
    }

    /// TODO
    ///
    /// # Panics
    ///
    /// TODO
    #[track_caller]
    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> Value {
        match self {
            List::Empty => panic!("cannot call `swap_remove` on an empty list"),
            List::Byte(vec) => vec.swap_remove(index).into(),
            List::Short(vec) => vec.swap_remove(index).into(),
            List::Int(vec) => vec.swap_remove(index).into(),
            List::Long(vec) => vec.swap_remove(index).into(),
            List::Float(vec) => vec.swap_remove(index).into(),
            List::Double(vec) => vec.swap_remove(index).into(),
            List::ByteArray(vec) => vec.swap_remove(index).into(),
            List::String(vec) => vec.swap_remove(index).into(),
            List::List(vec) => vec.swap_remove(index).into(),
            List::Compound(vec) => vec.swap_remove(index).into(),
            List::IntArray(vec) => vec.swap_remove(index).into(),
            List::LongArray(vec) => vec.swap_remove(index).into(),
        }
    }

    /// TODO
    ///
    /// # Panics
    ///
    /// TODO
    #[track_caller]
    #[inline]
    pub fn insert(&mut self, index: usize, element: Value) {
        if let List::Empty = self {
            *self = List::from(element);
            return;
        }

        match (self, element) {
            (List::Empty, _) => unreachable!(),
            (List::Byte(vec), Value::Byte(element)) => vec.insert(index, element),
            (List::Short(vec), Value::Short(element)) => vec.insert(index, element),
            (List::Int(vec), Value::Int(element)) => vec.insert(index, element),
            (List::Long(vec), Value::Long(element)) => vec.insert(index, element),
            (List::Float(vec), Value::Float(element)) => vec.insert(index, element),
            (List::Double(vec), Value::Double(element)) => vec.insert(index, element),
            (List::ByteArray(vec), Value::ByteArray(element)) => vec.insert(index, element),
            (List::String(vec), Value::String(element)) => vec.insert(index, element),
            (List::List(vec), Value::List(element)) => vec.insert(index, element),
            (List::Compound(vec), Value::Compound(element)) => vec.insert(index, element),
            (List::IntArray(vec), Value::IntArray(element)) => vec.insert(index, element),
            (List::LongArray(vec), Value::LongArray(element)) => vec.insert(index, element),
            _ => panic!("the `List` and `Value` variants do not match"),
        }
    }

    /// TODO
    ///
    /// # Errors
    ///
    /// TODO
    ///
    /// # Panics
    ///
    /// TODO
    #[inline]
    pub fn insert_checked(&mut self, index: usize, element: Value) -> Result<(), Value> {
        if let Some(expected) = self.ty() {
            if element.ty() != expected {
                return Err(element);
            }
        }

        self.insert(index, element);
        Ok(())
    }

    /// TODO
    ///
    /// # Panics
    ///
    /// TODO
    #[track_caller]
    #[inline]
    pub fn remove(&mut self, index: usize) -> Value {
        match self {
            List::Empty => panic!("cannot call `remove` on an empty list"),
            List::Byte(vec) => vec.remove(index).into(),
            List::Short(vec) => vec.remove(index).into(),
            List::Int(vec) => vec.remove(index).into(),
            List::Long(vec) => vec.remove(index).into(),
            List::Float(vec) => vec.remove(index).into(),
            List::Double(vec) => vec.remove(index).into(),
            List::ByteArray(vec) => vec.remove(index).into(),
            List::String(vec) => vec.remove(index).into(),
            List::List(vec) => vec.remove(index).into(),
            List::Compound(vec) => vec.remove(index).into(),
            List::IntArray(vec) => vec.remove(index).into(),
            List::LongArray(vec) => vec.remove(index).into(),
        }
    }

    /// TODO
    ///
    /// # Panics
    ///
    /// TODO
    #[track_caller]
    #[inline]
    pub fn push(&mut self, value: Value) {
        if let List::Empty = self {
            *self = List::from(value);
            return;
        }

        match (self, value) {
            (List::Empty, _) => unreachable!(),
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
            _ => panic!("the `List` and `Value` variants do not match"),
        }
    }

    /// TODO
    ///
    /// # Errors
    ///
    /// TODO
    ///
    /// # Panics
    ///
    /// TODO
    pub fn push_checked(&mut self, value: Value) -> Result<(), Value> {
        if let Some(expected) = self.ty() {
            if value.ty() != expected {
                return Err(value);
            }
        }

        self.push(value);
        Ok(())
    }

    /// TODO
    pub fn pop(&mut self) -> Option<Value> {
        match self {
            List::Empty => None,
            List::Byte(vec) => vec.pop().map(Value::from),
            List::Short(vec) => vec.pop().map(Value::from),
            List::Int(vec) => vec.pop().map(Value::from),
            List::Long(vec) => vec.pop().map(Value::from),
            List::Float(vec) => vec.pop().map(Value::from),
            List::Double(vec) => vec.pop().map(Value::from),
            List::ByteArray(vec) => vec.pop().map(Value::from),
            List::String(vec) => vec.pop().map(Value::from),
            List::List(vec) => vec.pop().map(Value::from),
            List::Compound(vec) => vec.pop().map(Value::from),
            List::IntArray(vec) => vec.pop().map(Value::from),
            List::LongArray(vec) => vec.pop().map(Value::from),
        }
    }

    /// TODO
    ///
    /// # Panics
    ///
    /// TODO
    pub fn append(&mut self, other: &mut List) {
        let other = mem::take(other);

        if let List::Empty = self {
            *self = other;
            return;
        }

        match (self, other) {
            (_, List::Empty) => {}
            (List::Byte(vec), List::Byte(ref mut other)) => vec.append(other),
            (List::Short(vec), List::Short(ref mut other)) => vec.append(other),
            (List::Int(vec), List::Int(ref mut other)) => vec.append(other),
            (List::Long(vec), List::Long(ref mut other)) => vec.append(other),
            (List::Float(vec), List::Float(ref mut other)) => vec.append(other),
            (List::Double(vec), List::Double(ref mut other)) => vec.append(other),
            (List::ByteArray(vec), List::ByteArray(ref mut other)) => vec.append(other),
            (List::String(vec), List::String(ref mut other)) => vec.append(other),
            (List::List(vec), List::List(ref mut other)) => vec.append(other),
            (List::Compound(vec), List::Compound(ref mut other)) => vec.append(other),
            (List::IntArray(vec), List::IntArray(ref mut other)) => vec.append(other),
            (List::LongArray(vec), List::LongArray(ref mut other)) => vec.append(other),
            _ => panic!("cannot append different tys of lists"),
        }
    }

    /// TODO
    ///
    /// # Errors
    ///
    /// TODO
    ///
    /// # Panics
    ///
    /// TODO
    pub fn append_checked(&mut self, other: &mut List) -> Result<(), MismatchError> {
        if self.ty() != other.ty() && self.ty().is_some() {
            return Err(MismatchError);
        }

        self.append(other);
        Ok(())
    }

    /// TODO
    #[inline]
    pub fn clear(&mut self) {
        match self {
            List::Empty => {}
            List::Byte(vec) => vec.clear(),
            List::Short(vec) => vec.clear(),
            List::Int(vec) => vec.clear(),
            List::Long(vec) => vec.clear(),
            List::Float(vec) => vec.clear(),
            List::Double(vec) => vec.clear(),
            List::ByteArray(vec) => vec.clear(),
            List::String(vec) => vec.clear(),
            List::List(vec) => vec.clear(),
            List::Compound(vec) => vec.clear(),
            List::IntArray(vec) => vec.clear(),
            List::LongArray(vec) => vec.clear(),
        }
    }

    /// TODO
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

    /// TODO
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for List {
    /// TODO
    #[inline]
    fn default() -> Self {
        List::Empty
    }
}

impl FromIterator<Byte> for List {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Byte>>(iter: I) -> Self {
        List::Byte(iter.into_iter().collect())
    }
}

impl FromIterator<i16> for List {
    #[inline]
    fn from_iter<I: IntoIterator<Item = i16>>(iter: I) -> Self {
        List::Short(iter.into_iter().collect())
    }
}

impl FromIterator<i32> for List {
    #[inline]
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        List::Int(iter.into_iter().collect())
    }
}

impl FromIterator<i64> for List {
    #[inline]
    fn from_iter<I: IntoIterator<Item = i64>>(iter: I) -> Self {
        List::Long(iter.into_iter().collect())
    }
}

impl FromIterator<f32> for List {
    #[inline]
    fn from_iter<I: IntoIterator<Item = f32>>(iter: I) -> Self {
        List::Float(iter.into_iter().collect())
    }
}

impl FromIterator<f64> for List {
    #[inline]
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        List::Double(iter.into_iter().collect())
    }
}

impl FromIterator<ByteArray> for List {
    #[inline]
    fn from_iter<I: IntoIterator<Item = ByteArray>>(iter: I) -> Self {
        List::ByteArray(iter.into_iter().collect())
    }
}

impl FromIterator<String> for List {
    #[inline]
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        List::String(iter.into_iter().collect())
    }
}

impl FromIterator<List> for List {
    #[inline]
    fn from_iter<I: IntoIterator<Item = List>>(iter: I) -> Self {
        List::List(iter.into_iter().collect())
    }
}

impl FromIterator<Compound> for List {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Compound>>(iter: I) -> Self {
        List::Compound(iter.into_iter().collect())
    }
}

impl FromIterator<IntArray> for List {
    #[inline]
    fn from_iter<I: IntoIterator<Item = IntArray>>(iter: I) -> Self {
        List::IntArray(iter.into_iter().collect())
    }
}

impl FromIterator<LongArray> for List {
    #[inline]
    fn from_iter<I: IntoIterator<Item = LongArray>>(iter: I) -> Self {
        List::LongArray(iter.into_iter().collect())
    }
}

impl IntoIterator for List {
    type Item = Value;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let inner = self.into_vec().into_iter();
        IntoIter { inner }
    }
}

impl TryFrom<Vec<Value>> for List {
    type Error = Vec<Value>;

    fn try_from(vec: Vec<Value>) -> Result<Self, Self::Error> {
        let Some(expected) = vec.first().map(Value::ty) else { return Ok(List::Empty) };
        let is_valid = vec.iter().all(|value| value.ty() == expected);

        if !is_valid {
            return Err(vec);
        }

        todo!()
    }
}

impl From<Value> for List {
    #[inline]
    fn from(value: Value) -> Self {
        match value {
            Value::Byte(value) => List::Byte(vec![value]),
            Value::Short(value) => List::Short(vec![value]),
            Value::Int(value) => List::Int(vec![value]),
            Value::Long(value) => List::Long(vec![value]),
            Value::Float(value) => List::Float(vec![value]),
            Value::Double(value) => List::Double(vec![value]),
            Value::ByteArray(value) => List::ByteArray(vec![value]),
            Value::String(value) => List::String(vec![value]),
            Value::List(value) => List::List(vec![value]),
            Value::Compound(value) => List::Compound(vec![value]),
            Value::IntArray(value) => List::IntArray(vec![value]),
            Value::LongArray(value) => List::LongArray(vec![value]),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

/// TODO
#[derive(Clone)]
pub struct IntoIter {
    inner: vec::IntoIter<Value>,
}

impl IntoIter {
    /// TODO
    #[must_use]
    #[inline]
    pub fn as_slice(&self) -> &[Value] {
        self.inner.as_slice()
    }

    /// TODO
    #[must_use]
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [Value] {
        self.inner.as_mut_slice()
    }
}

impl fmt::Debug for IntoIter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl Iterator for IntoIter {
    type Item = Value;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.inner.count()
    }
}

impl DoubleEndedIterator for IntoIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl ExactSizeIterator for IntoIter {}

impl FusedIterator for IntoIter {}

////////////////////////////////////////////////////////////////////////////////

/// TODO
pub struct MismatchError;

impl fmt::Display for MismatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("the given datatypes did not match")
    }
}
