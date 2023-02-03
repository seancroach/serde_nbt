//! TODO

use crate::error::{Error, Result};

use alloc::borrow::Cow;
use core::{fmt::Display, str};

use serde::{ser, Serialize};

pub(crate) struct KeyQuery {
    is_human_readable: bool,
}

impl KeyQuery {
    #[must_use]
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

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
        let key = if value { "true" } else { "false" };
        Ok(Cow::Borrowed(key))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let key = buf.format(value).to_string();
        Ok(Cow::Owned(key))
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let key = buf.format(value).to_string();
        Ok(Cow::Owned(key))
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let key = buf.format(value).to_string();
        Ok(Cow::Owned(key))
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let key = buf.format(value).to_string();
        Ok(Cow::Owned(key))
    }

    #[inline]
    fn serialize_i128(self, value: i128) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let key = buf.format(value).to_string();
        Ok(Cow::Owned(key))
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let key = buf.format(value).to_string();
        Ok(Cow::Owned(key))
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let key = buf.format(value).to_string();
        Ok(Cow::Owned(key))
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let key = buf.format(value).to_string();
        Ok(Cow::Owned(key))
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let key = buf.format(value).to_string();
        Ok(Cow::Owned(key))
    }

    #[inline]
    fn serialize_u128(self, value: u128) -> Result<Self::Ok> {
        let mut buf = itoa::Buffer::new();
        let key = buf.format(value).to_string();
        Ok(Cow::Owned(key))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        todo!()
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<Self::Ok> {
        let mut buf = [0; 4];
        let key = (*value.encode_utf8(&mut buf)).to_string();
        Ok(Cow::Owned(key))
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        let key = value.to_string();
        Ok(Cow::Owned(key))
    }

    #[inline]
    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        match str::from_utf8(value) {
            Ok(key) => self.serialize_str(key),
            Err(_) => todo!(),
        }
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        todo!()
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        todo!()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(Cow::Borrowed(variant))
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }

    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        todo!()
    }

    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        todo!()
    }

    #[inline]
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Display,
    {
        let key = value.to_string();
        Ok(Cow::Owned(key))
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        self.is_human_readable
    }
}
