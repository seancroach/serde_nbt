use crate::{
    error::{Error, Path, Result},
    ser::collect_key,
    util::ArrayBrand,
    value::{Byte, Compound, List, Value},
};

use alloc::borrow::Cow;
use core::fmt::Display;

use serde::{ser, Serialize};

/// TODO
///
/// # Errors
///
/// TODO
pub fn to_value<T>(value: &T) -> Result<Value>
where
    T: ?Sized + Serialize,
{
    let mut serializer = ValueSerializer::new(true);
    value.serialize(&mut serializer)
}

pub struct ValueSerializer {
    is_human_readable: bool,
    path: Path,
}

impl ValueSerializer {
    /// TODO
    #[must_use]
    #[inline]
    pub fn new(is_human_readable: bool) -> Self {
        ValueSerializer {
            is_human_readable,
            path: Path::new(),
        }
    }
}

impl<'ser> ser::Serializer for &'ser mut ValueSerializer {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SeqSerializer<'ser>;
    type SerializeTuple = SeqSerializer<'ser>;
    type SerializeTupleStruct = SeqSerializer<'ser>;
    type SerializeTupleVariant = SeqSerializer<'ser>;
    type SerializeMap = MapSerializer<'ser>;
    type SerializeStruct = MapSerializer<'ser>;
    type SerializeStructVariant = MapSerializer<'ser>;

    #[inline]
    fn serialize_bool(self, value: bool) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(Value::Byte(Byte::Boolean(value)))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(Value::Byte(Byte::Integer(value)))
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(Value::Short(value))
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(Value::Int(value))
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(Value::Long(value))
    }

    fn serialize_i128(self, _value: i128) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`i128`", &mut self.path))
    }

    fn serialize_u8(self, _value: u8) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`u8`", &mut self.path))
    }

    fn serialize_u16(self, _value: u16) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`u16`", &mut self.path))
    }

    fn serialize_u32(self, _value: u32) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`u32`", &mut self.path))
    }

    fn serialize_u64(self, _value: u64) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`u64`", &mut self.path))
    }

    fn serialize_u128(self, _value: u128) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`u128`", &mut self.path))
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(Value::Float(value))
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(Value::Double(value))
    }

    #[inline]
    fn serialize_char(self, value: char) -> std::result::Result<Self::Ok, Self::Error> {
        let mut buf = [0; 4];
        self.serialize_str(value.encode_utf8(&mut buf))
    }

    #[inline]
    fn serialize_str(self, value: &str) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(Value::String(value.to_owned()))
    }

    fn serialize_bytes(self, _value: &[u8]) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`&[u8]`", &mut self.path))
    }

    fn serialize_none(self) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`None`", &mut self.path))
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`()`", &mut self.path))
    }

    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("unit struct", &mut self.path))
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
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
        variant: &'static str,
        value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut map = self.serialize_struct(Default::default(), 1)?;
        ser::SerializeStruct::serialize_field(&mut map, variant, value)?;
        ser::SerializeStruct::end(map)
    }

    #[inline]
    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> std::result::Result<Self::SerializeSeq, Self::Error> {
        Ok(SeqSerializer::new(self, None, len))
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> std::result::Result<Self::SerializeTuple, Self::Error> {
        Ok(SeqSerializer::new(self, None, Some(len)))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeTupleStruct, Self::Error> {
        let brand = ArrayBrand::from_str(name);
        Ok(SeqSerializer::new(self, brand, Some(len)))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
        let brand = ArrayBrand::from_str(name);
        self.path.enter_scope(Cow::Borrowed(variant));
        Ok(SeqSerializer::new(self, brand, Some(len)))
    }

    #[inline]
    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> std::result::Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer::new(self, len))
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeStruct, Self::Error> {
        Ok(MapSerializer::new(self, Some(len)))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
        self.path.enter_scope(Cow::Borrowed(variant));
        Ok(MapSerializer::new(self, Some(len)))
    }

    #[inline]
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Display,
    {
        Ok(Value::String(value.to_string()))
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        self.is_human_readable
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! end_sequence {
    ($self:ident) => {{
        match ($self.brand, $self.list) {
            (Some(ArrayBrand::Byte), List::Empty) => Ok(Value::ByteArray(Vec::new())),
            (Some(ArrayBrand::Int), List::Empty) => Ok(Value::IntArray(Vec::new())),
            (Some(ArrayBrand::Long), List::Empty) => Ok(Value::LongArray(Vec::new())),

            (Some(ArrayBrand::Byte), List::Byte(vec)) => Ok(Value::ByteArray(vec)),
            (Some(ArrayBrand::Int), List::Int(vec)) => Ok(Value::IntArray(vec)),
            (Some(ArrayBrand::Long), List::Long(vec)) => Ok(Value::LongArray(vec)),

            (None, list) => Ok(Value::List(list)),

            _ => unreachable!(),
        }
    }};
}

////////////////////////////////////////////////////////////////////////////////

pub struct SeqSerializer<'ser> {
    serializer: &'ser mut ValueSerializer,
    brand: Option<ArrayBrand>,
    len: Option<usize>,
    list: List,
    index: usize,
}

impl<'ser> SeqSerializer<'ser> {
    #[must_use]
    #[inline]
    fn new(
        serializer: &'ser mut ValueSerializer,
        brand: Option<ArrayBrand>,
        len: Option<usize>,
    ) -> Self {
        let list = brand
            .map(|brand| {
                let capacity = len.unwrap_or_default();
                let kind = brand.element_kind();
                List::with_capacity_and_kind(capacity, kind)
            })
            .unwrap_or_default();
        SeqSerializer {
            serializer,
            brand,
            len,
            list,
            index: 0,
        }
    }
}

impl<'ser> ser::SerializeSeq for SeqSerializer<'ser> {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serializer.path.enter_element(self.index);

        let value = value.serialize(&mut *self.serializer)?;

        if let List::Empty = self.list {
            self.serializer.path.leave_element();

            let capacity = self.len.unwrap_or_default();
            let kind = value.kind();
            let mut list = List::with_capacity_and_kind(capacity, kind);
            unsafe { list.push_unchecked(value) };
        } else {
            if let Err(value) = self.list.push_checked(value) {
                return Err(Error::invalid_seq(
                    value.kind().to_id(),
                    self.list.id(),
                    &mut self.serializer.path,
                ));
            }
            self.serializer.path.leave_element();
        }

        self.index += 1;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        end_sequence!(self)
    }
}

impl<'ser> ser::SerializeTuple for SeqSerializer<'ser> {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}

impl<'ser> ser::SerializeTupleStruct for SeqSerializer<'ser> {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}

impl<'ser> ser::SerializeTupleVariant for SeqSerializer<'ser> {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        let list = end_sequence!(self)?;

        let mut compound = Compound::with_capacity(1);
        let key = self.serializer.path.leave_scope().into_owned();
        compound.insert(key, list);

        Ok(Value::Compound(compound))
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct MapSerializer<'ser> {
    serializer: &'ser mut ValueSerializer,
    compound: Compound,
}

impl<'ser> MapSerializer<'ser> {
    #[must_use]
    #[inline]
    fn new(serializer: &'ser mut ValueSerializer, len: Option<usize>) -> Self {
        let capacity = len.unwrap_or_default();
        MapSerializer {
            serializer,
            compound: Compound::with_capacity(capacity),
        }
    }
}

impl<'ser> ser::SerializeMap for MapSerializer<'ser> {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let key = collect_key(
            key,
            &mut self.serializer.path,
            self.serializer.is_human_readable,
        )?;
        self.serializer.path.enter_scope(key);
        Ok(())
    }

    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(&mut *self.serializer)?;

        let key = self.serializer.path.leave_scope().into_owned();
        self.compound.insert(key, value);

        Ok(())
    }

    #[inline]
    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(Value::Compound(self.compound))
    }
}

impl<'ser> ser::SerializeStruct for MapSerializer<'ser> {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serializer.path.enter_scope(Cow::Borrowed(key));

        let value = value.serialize(&mut *self.serializer)?;
        self.compound.insert(key.to_owned(), value);

        self.serializer.path.leave_scope();

        Ok(())
    }

    #[inline]
    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(Value::Compound(self.compound))
    }
}

impl<'ser> ser::SerializeStructVariant for MapSerializer<'ser> {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeStruct::serialize_field(self, key, value)
    }

    #[inline]
    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        let inner = Value::Compound(self.compound);

        let mut outer = Compound::with_capacity(1);
        let key = self.serializer.path.leave_scope().into_owned();
        outer.insert(key, inner);

        Ok(Value::Compound(outer))
    }
}
