use crate::{
    error::{Error, Path, Result},
    value::ValueKind,
    ArrayBrand, SeqKind,
};

use alloc::borrow::Cow;
use core::{fmt::Display, str};

use serde::{ser, Serialize};

////////////////////////////////////////////////////////////////////////////////

macro_rules! serialize_passthroughs {
    () => {
        #[inline]
        fn serialize_char(self, value: char) -> Result<Self::Ok> {
            let mut buf = [0; 4];
            self.serialize_str(value.encode_utf8(&mut buf))
        }

        #[inline]
        fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(self)
        }

        #[inline]
        fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(self)
        }

        #[inline]
        fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
        ) -> Result<Self::Ok> {
            self.serialize_str(variant)
        }
    }
}

macro_rules! serialize_invalid_types {
    () => {
        #[allow(unused_mut)]
        fn serialize_i128(mut self, _value: i128) -> Result<Self::Ok> {
            Err(Error::invalid_type("`i128`", &mut self.path))
        }

        #[allow(unused_mut)]
        fn serialize_u8(mut self, _value: u8) -> Result<Self::Ok> {
            Err(Error::invalid_type("`u8`", &mut self.path))
        }

        #[allow(unused_mut)]
        fn serialize_u16(mut self, _value: u16) -> Result<Self::Ok> {
            Err(Error::invalid_type("`u16`", &mut self.path))
        }

        #[allow(unused_mut)]
        fn serialize_u32(mut self, _value: u32) -> Result<Self::Ok> {
            Err(Error::invalid_type("`u32`", &mut self.path))
        }

        #[allow(unused_mut)]
        fn serialize_u64(mut self, _value: u64) -> Result<Self::Ok> {
            Err(Error::invalid_type("`u64`", &mut self.path))
        }

        #[allow(unused_mut)]
        fn serialize_u128(mut self, _value: u128) -> Result<Self::Ok> {
            Err(Error::invalid_type("`u128`", &mut self.path))
        }

        #[allow(unused_mut)]
        fn serialize_bytes(mut self, _value: &[u8]) -> Result<Self::Ok> {
            Err(Error::invalid_type("`&[u8]`", &mut self.path))
        }

        #[allow(unused_mut)]
        fn serialize_none(mut self) -> Result<Self::Ok> {
            Err(Error::invalid_type("`None`", &mut self.path))
        }

        #[allow(unused_mut)]
        fn serialize_unit(mut self) -> Result<Self::Ok> {
            Err(Error::invalid_type("`()`", &mut self.path))
        }

        #[allow(unused_mut)]
        fn serialize_unit_struct(mut self, _name: &'static str) -> Result<Self::Ok> {
            Err(Error::invalid_type("unit struct", &mut self.path))
        }
    };
}

pub(crate) use {serialize_invalid_types, serialize_passthroughs};

////////////////////////////////////////////////////////////////////////////////

#[inline]
pub(crate) fn collect_key<T>(key: &T, path: &mut Path, is_human_readable: bool) -> Result<Cow<'static, str>>
where
    T: ?Sized + Serialize
{
    let query = KeyQuery::new(path, is_human_readable);
    key.serialize(query)
}

struct KeyQuery<'path> {
    path: &'path mut Path,
    is_human_readable: bool,
}

impl<'path> KeyQuery<'path> {
    #[must_use]
    #[inline]
    fn new(path: &'path mut Path, is_human_readable: bool) -> Self {
        KeyQuery {
            path,
            is_human_readable,
        }
    }

    fn handle_tuple<T>(self, name: &'static str) -> Result<T> {
        let display_type = match ArrayBrand::from_str(name) {
            Some(ArrayBrand::Byte) => "byte array",
            Some(ArrayBrand::Int) => "int array",
            Some(ArrayBrand::Long) => "long array",
            None => "sequence",
        };
        Err(Error::invalid_key(display_type, self.path))
    }
}

macro_rules! integer_to_key {
    {
        $($method:ident for $int:ty,)+
    } => {
        $(
            #[inline]
            fn $method(self, value: $int) -> Result<Self::Ok> {
                let mut buf = itoa::Buffer::new();
                let data = buf.format(value).to_owned();
                Ok(Cow::Owned(data))
            }
        )+
    }
}

impl<'path> ser::Serializer for KeyQuery<'path> {
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
        let data = if value { "true" } else { "false" };
        Ok(Cow::Borrowed(data))
    }

    integer_to_key! {
        serialize_i8 for i8,
        serialize_i16 for i16,
        serialize_i32 for i32,
        serialize_i64 for i64,
        serialize_i128 for i128,

        serialize_u8 for u8,
        serialize_u16 for u16,
        serialize_u32 for u32,
        serialize_u64 for u64,
        serialize_u128 for u128,
    }

    fn serialize_f32(self, _value: f32) -> Result<Self::Ok> {
        Err(Error::invalid_key("`f32`", self.path))
    }

    fn serialize_f64(self, _value: f64) -> Result<Self::Ok> {
        Err(Error::invalid_key("`f64`", self.path))
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<Self::Ok> {
        let mut buf = [0; 4];
        self.serialize_str(value.encode_utf8(&mut buf))
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        let data = value.to_owned();
        Ok(Cow::Owned(data))
    }

    #[inline]
    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        if let Ok(data) = str::from_utf8(value) {
            self.serialize_str(data)
        } else {
            Err(Error::invalid_key("non-UTF8 `&[u8]`", self.path))
        }
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::invalid_key("`None`", self.path))
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::invalid_key("`()`", self.path))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(Error::invalid_key("unit struct", self.path))
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
        Err(Error::invalid_key("newtype variant", self.path))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::invalid_key("sequence", self.path))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::invalid_key("tuple", self.path))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.handle_tuple(name)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.handle_tuple(name)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::invalid_key("map", self.path))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::invalid_key("struct", self.path))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::invalid_key("struct variant", self.path))
    }

    fn collect_seq<I>(self, _iter: I) -> Result<Self::Ok>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        Err(Error::invalid_key("sequence", self.path))
    }

    fn collect_map<K, V, I>(self, _iter: I) -> Result<Self::Ok>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        Err(Error::invalid_key("map", self.path))
    }

    #[inline]
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Display,
    {
        let data = value.to_string();
        Ok(Cow::Owned(data))
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        self.is_human_readable
    }
}

////////////////////////////////////////////////////////////////////////////////

#[inline]
pub(crate) fn query_kind<T>(value: &T, path: &mut Path, is_human_readable: bool) -> Result<ValueKind>
    where
        T: ?Sized + Serialize
{
    let query = KindQuery::new(path, is_human_readable);
    value.serialize(query)
}

struct KindQuery<'path> {
    path: &'path mut Path,
    is_human_readable: bool,
}

impl<'path> KindQuery<'path> {
    #[must_use]
    #[inline]
    fn new(path: &'path mut Path, is_human_readable: bool) -> Self {
        KindQuery {
            path,
            is_human_readable,
        }
    }
}

impl<'path> ser::Serializer for KindQuery<'path> {
    type Ok = ValueKind;
    type Error = Error;

    type SerializeSeq = NoOp<ValueKind>;
    type SerializeTuple = NoOp<ValueKind>;
    type SerializeTupleStruct = NoOp<ValueKind>;
    type SerializeTupleVariant = NoOp<ValueKind>;
    type SerializeMap = NoOp<ValueKind>;
    type SerializeStruct = NoOp<ValueKind>;
    type SerializeStructVariant = NoOp<ValueKind>;

    serialize_invalid_types!();

    #[inline]
    fn serialize_bool(self, _value: bool) -> Result<Self::Ok> {
        Ok(ValueKind::Byte)
    }

    #[inline]
    fn serialize_i8(self, _value: i8) -> Result<Self::Ok> {
        Ok(ValueKind::Byte)
    }

    #[inline]
    fn serialize_i16(self, _value: i16) -> Result<Self::Ok> {
        Ok(ValueKind::Short)
    }

    #[inline]
    fn serialize_i32(self, _value: i32) -> Result<Self::Ok> {
        Ok(ValueKind::Int)
    }

    #[inline]
    fn serialize_i64(self, _value: i64) -> Result<Self::Ok> {
        Ok(ValueKind::Long)
    }

    #[inline]
    fn serialize_f32(self, _value: f32) -> Result<Self::Ok> {
        Ok(ValueKind::Float)
    }

    #[inline]
    fn serialize_f64(self, _value: f64) -> Result<Self::Ok> {
        Ok(ValueKind::Double)
    }

    #[inline]
    fn serialize_char(self, _value: char) -> Result<Self::Ok> {
        Ok(ValueKind::String)
    }

    #[inline]
    fn serialize_str(self, _value: &str) -> Result<Self::Ok> {
        Ok(ValueKind::String)
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(ValueKind::String)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
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
        Ok(ValueKind::Compound)
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(NoOp::new(ValueKind::List))
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(NoOp::new(ValueKind::List))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        let id = ArrayBrand::from_str(name).map_or(ValueKind::List, ArrayBrand::kind);
        Ok(NoOp::new(id))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(NoOp::new(ValueKind::Compound))
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(NoOp::new(ValueKind::Compound))
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(NoOp::new(ValueKind::Compound))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(NoOp::new(ValueKind::Compound))
    }

    #[inline]
    fn collect_seq<I>(self, _iter: I) -> Result<Self::Ok>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        Ok(ValueKind::List)
    }

    #[inline]
    fn collect_map<K, V, I>(self, _iter: I) -> Result<Self::Ok>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        Ok(ValueKind::Compound)
    }

    #[inline]
    fn collect_str<T>(self, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Display,
    {
        Ok(ValueKind::String)
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        self.is_human_readable
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) struct NoOp<Ok> {
    ok: Ok,
}

impl<Ok> NoOp<Ok> {
    #[must_use]
    #[inline]
    fn new(ok: Ok) -> Self {
        NoOp { ok }
    }
}

impl<Ok> ser::SerializeSeq for NoOp<Ok> {
    type Ok = Ok;
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        Ok(self.ok)
    }
}

impl<Ok> ser::SerializeTuple for NoOp<Ok> {
    type Ok = Ok;
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        Ok(self.ok)
    }
}

impl<Ok> ser::SerializeTupleStruct for NoOp<Ok> {
    type Ok = Ok;
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        Ok(self.ok)
    }
}

impl<Ok> ser::SerializeTupleVariant for NoOp<Ok> {
    type Ok = Ok;
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        Ok(self.ok)
    }
}

impl<Ok> ser::SerializeMap for NoOp<Ok> {
    type Ok = Ok;
    type Error = Error;

    #[inline]
    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> Result<()>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        Ok(self.ok)
    }
}

impl<Ok> ser::SerializeStruct for NoOp<Ok> {
    type Ok = Ok;
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        Ok(self.ok)
    }
}

impl<Ok> ser::SerializeStructVariant for NoOp<Ok> {
    type Ok = Ok;
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        Ok(self.ok)
    }
}
