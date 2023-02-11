use crate::{
    error::{Error, Path, Result},
    value::TypeId,
    ArrayBrand,
};

use alloc::borrow::Cow;
use core::{fmt::Display, result, str};

use serde::{ser, Serialize};

////////////////////////////////////////////////////////////////////////////////
// `serialize_with` methods
////////////////////////////////////////////////////////////////////////////////

/// TODO
///
/// # Errors
///
/// TODO
#[inline]
pub fn byte_array<T, S>(value: T, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        T: IntoIterator,
        T::IntoIter: ExactSizeIterator,
        <T as IntoIterator>::Item: Serialize,
        S: ser::Serializer,
{
    let mut iter = value.into_iter();
    let mut ser = serializer.serialize_tuple_struct(ArrayBrand::Byte.as_str(), iter.len())?;
    iter.try_for_each(|value| ser::SerializeTupleStruct::serialize_field(&mut ser, &value))?;
    ser::SerializeTupleStruct::end(ser)
}

/// TODO
///
/// # Errors
///
/// TODO
#[inline]
pub fn int_array<T, S>(value: T, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        T: IntoIterator,
        T::IntoIter: ExactSizeIterator,
        <T as IntoIterator>::Item: Serialize,
        S: ser::Serializer,
{
    let mut iter = value.into_iter();
    let mut ser = serializer.serialize_tuple_struct(ArrayBrand::Int.as_str(), iter.len())?;
    iter.try_for_each(|value| ser::SerializeTupleStruct::serialize_field(&mut ser, &value))?;
    ser::SerializeTupleStruct::end(ser)
}

/// TODO
///
/// # Errors
///
/// TODO
#[inline]
pub fn long_array<T, S>(value: T, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        T: IntoIterator,
        T::IntoIter: ExactSizeIterator,
        <T as IntoIterator>::Item: Serialize,
        S: ser::Serializer,
{
    let mut iter = value.into_iter();
    let mut ser = serializer.serialize_tuple_struct(ArrayBrand::Long.as_str(), iter.len())?;
    iter.try_for_each(|value| ser::SerializeTupleStruct::serialize_field(&mut ser, &value))?;
    ser::SerializeTupleStruct::end(ser)
}

////////////////////////////////////////////////////////////////////////////////
// TypeId Queries
////////////////////////////////////////////////////////////////////////////////

pub(crate) struct IdQuery {
    is_human_readable: bool,
}

impl IdQuery {
    pub(crate) fn new(is_human_readable: bool) -> Self {
        IdQuery { is_human_readable }
    }
}

impl ser::Serializer for IdQuery {
    type Ok = TypeId;
    type Error = Error;

    type SerializeSeq = NoOp<Self::Ok>;
    type SerializeTuple = NoOp<Self::Ok>;
    type SerializeTupleStruct = NoOp<Self::Ok>;
    type SerializeTupleVariant = NoOp<Self::Ok>;

    type SerializeMap = NoOp<Self::Ok>;
    type SerializeStruct = NoOp<Self::Ok>;
    type SerializeStructVariant = NoOp<Self::Ok>;

    fn serialize_bool(self, _value: bool) -> Result<Self::Ok> {
        Ok(TypeId::Byte)
    }

    fn serialize_i8(self, _value: i8) -> Result<Self::Ok> {
        Ok(TypeId::Byte)
    }

    fn serialize_i16(self, _value: i16) -> Result<Self::Ok> {
        Ok(TypeId::Short)
    }

    fn serialize_i32(self, _value: i32) -> Result<Self::Ok> {
        Ok(TypeId::Int)
    }

    fn serialize_i64(self, _value: i64) -> Result<Self::Ok> {
        Ok(TypeId::Long)
    }

    fn serialize_i128(self, _value: i128) -> Result<Self::Ok> {
        Err(Error::invalid_type("`i128`"))
    }

    fn serialize_u8(self, _value: u8) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u8`"))
    }

    fn serialize_u16(self, _value: u16) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u16`"))
    }

    fn serialize_u32(self, _value: u32) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u32`"))
    }

    fn serialize_u64(self, _value: u64) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u64`"))
    }

    fn serialize_u128(self, _value: u128) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u128`"))
    }

    fn serialize_f32(self, _value: f32) -> Result<Self::Ok> {
        Ok(TypeId::Float)
    }

    fn serialize_f64(self, _value: f64) -> Result<Self::Ok> {
        Ok(TypeId::Double)
    }

    fn serialize_char(self, _value: char) -> Result<Self::Ok> {
        Ok(TypeId::String)
    }

    fn serialize_str(self, _value: &str) -> Result<Self::Ok> {
        Ok(TypeId::String)
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok> {
        Err(Error::invalid_type("`&[u8]`"))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::invalid_type("`None`"))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
        where
            T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::invalid_type("`()`"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(Error::invalid_type("unit struct"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(TypeId::String)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
        where
            T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
        where
            T: Serialize,
    {
        Ok(TypeId::Compound)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(NoOp::new(TypeId::List))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(NoOp::new(TypeId::List))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        let id = ArrayBrand::from_str(name).map_or(TypeId::List, ArrayBrand::id);
        Ok(NoOp::new(id))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(NoOp::new(TypeId::Compound))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(NoOp::new(TypeId::Compound))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(NoOp::new(TypeId::Compound))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(NoOp::new(TypeId::Compound))
    }

    fn collect_seq<I>(self, _iter: I) -> Result<Self::Ok>
        where
            I: IntoIterator,
            <I as IntoIterator>::Item: Serialize,
    {
        Ok(TypeId::List)
    }

    fn collect_map<K, V, I>(self, _iter: I) -> Result<Self::Ok>
        where
            K: Serialize,
            V: Serialize,
            I: IntoIterator<Item = (K, V)>,
    {
        Ok(TypeId::Compound)
    }

    fn collect_str<T>(self, _value: &T) -> Result<Self::Ok>
        where
            T: ?Sized + Display,
    {
        Ok(TypeId::String)
    }

    fn is_human_readable(&self) -> bool {
        self.is_human_readable
    }
}

pub(crate) struct NoOp<Ok> {
    ok: Ok,
}

impl<Ok> NoOp<Ok> {
    fn new(ok: Ok) -> Self {
        NoOp { ok }
    }
}

impl<Ok> ser::SerializeSeq for NoOp<Ok> {
    type Ok = Ok;
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.ok)
    }
}

impl<Ok> ser::SerializeTuple for NoOp<Ok> {
    type Ok = Ok;
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.ok)
    }
}

impl<Ok> ser::SerializeTupleStruct for NoOp<Ok> {
    type Ok = Ok;
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.ok)
    }
}

impl<Ok> ser::SerializeTupleVariant for NoOp<Ok> {
    type Ok = Ok;
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.ok)
    }
}

impl<Ok> ser::SerializeMap for NoOp<Ok> {
    type Ok = Ok;
    type Error = Error;

    fn serialize_key<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> Result<()>
        where
            K: ?Sized + Serialize,
            V: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.ok)
    }
}

impl<Ok> ser::SerializeStruct for NoOp<Ok> {
    type Ok = Ok;
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.ok)
    }
}

impl<Ok> ser::SerializeStructVariant for NoOp<Ok> {
    type Ok = Ok;
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.ok)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Key Queries
////////////////////////////////////////////////////////////////////////////////

pub(crate) struct KeyQuery {
    is_human_readable: bool,
}

impl KeyQuery {
    pub(crate) fn new(is_human_readable: bool) -> Self {
        KeyQuery { is_human_readable }
    }
}

impl ser::Serializer for KeyQuery {
    type Ok = Cow<'static, str>;
    type Error = Error;

    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;

    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
        let data = if value { "true" } else { "false" };
        Ok(Cow::Borrowed(data))
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value);
        Ok(Cow::Owned(data.to_string()))
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value);
        Ok(Cow::Owned(data.to_string()))
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value);
        Ok(Cow::Owned(data.to_string()))
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value);
        Ok(Cow::Owned(data.to_string()))
    }

    fn serialize_i128(self, value: i128) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value);
        Ok(Cow::Owned(data.to_string()))
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value);
        Ok(Cow::Owned(data.to_string()))
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value);
        Ok(Cow::Owned(data.to_string()))
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value);
        Ok(Cow::Owned(data.to_string()))
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value);
        Ok(Cow::Owned(data.to_string()))
    }

    fn serialize_u128(self, value: u128) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value);
        Ok(Cow::Owned(data.to_string()))
    }

    fn serialize_f32(self, _value: f32) -> Result<Self::Ok> {
        Err(Error::invalid_key("`f32`"))
    }

    fn serialize_f64(self, _value: f64) -> Result<Self::Ok> {
        Err(Error::invalid_key("`f64`"))
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok> {
        Ok(Cow::Owned(value.to_string()))
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        Ok(Cow::Owned(value.to_string()))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        match str::from_utf8(value) {
            Ok(key) => self.serialize_str(key),
            Err(_) => Err(Error::invalid_key("non-UTF-8 `&[u8]`")),
        }
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::invalid_key("`None`"))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
        where
            T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::invalid_key("`()`"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(Error::invalid_key("unit struct"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(Cow::Borrowed(variant))
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
        where
            T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
        where
            T: ?Sized + Serialize,
    {
        Err(Error::invalid_key("newtype variant"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::invalid_key("seq"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::invalid_key("tuple"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::invalid_key("tuple struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::invalid_key("tuple variant"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::invalid_key("map"))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::invalid_key("struct"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::invalid_key("struct variant"))
    }

    fn collect_seq<I>(self, _iter: I) -> Result<Self::Ok>
        where
            I: IntoIterator,
            <I as IntoIterator>::Item: Serialize,
    {
        Err(Error::invalid_key("seq"))
    }

    fn collect_map<K, V, I>(self, _iter: I) -> Result<Self::Ok>
        where
            K: Serialize,
            V: Serialize,
            I: IntoIterator<Item = (K, V)>,
    {
        Err(Error::invalid_key("map"))
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok>
        where
            T: ?Sized + Display,
    {
        Ok(Cow::Owned(value.to_string()))
    }

    fn is_human_readable(&self) -> bool {
        self.is_human_readable
    }
}
