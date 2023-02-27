//! TODO

use super::{Byte, Compound, List, Value};

use crate::{
    error::{Error, Path, Result},
    ser::{serialize_invalid_types, serialize_passthroughs, KeyQuery},
    ArrayBrand,
};

use alloc::borrow::Cow;
use core::fmt::Display;

use serde::{ser, Serialize};

/// TODO
///
/// # Errors
///
/// TODO
///
/// # Panics
///
/// TODO
pub fn to_value<T>(value: &T) -> Result<Value>
where
    T: ?Sized + Serialize,
{
    let mut serializer = Serializer::new(true);
    value.serialize(&mut serializer)
}

pub struct Serializer {
    path: Path,
    is_human_readable: bool,
}

impl Serializer {
    #[must_use]
    #[inline]
    pub fn new(is_human_readable: bool) -> Self {
        Serializer {
            path: Path::new(),
            is_human_readable,
        }
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a>;
    type SerializeTuple = SeqSerializer<'a>;
    type SerializeTupleStruct = SeqSerializer<'a>;
    type SerializeTupleVariant = SeqSerializer<'a>;
    type SerializeMap = MapSerializer<'a>;
    type SerializeStruct = MapSerializer<'a>;
    type SerializeStructVariant = MapSerializer<'a>;

    serialize_invalid_types!();
    serialize_passthroughs!();

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
        Ok(Value::Byte(Byte::Boolean(value)))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
        Ok(Value::Byte(Byte::Integer(value)))
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        Ok(Value::Short(value))
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        Ok(Value::Int(value))
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        Ok(Value::Long(value))
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<Self::Ok> {
        Ok(Value::Float(value))
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<Self::Ok> {
        Ok(Value::Double(value))
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        Ok(Value::String(value.to_owned()))
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let mut map = self.serialize_map(Some(1))?;
        ser::SerializeStruct::serialize_field(&mut map, variant, value)?;
        ser::SerializeStruct::end(map)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SeqSerializer::new(self, None, len))
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
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
    ) -> Result<Self::SerializeTupleVariant> {
        self.path.enter_scope(Cow::Borrowed(variant));
        self.serialize_tuple_struct(name, len)
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(MapSerializer::new(self, len))
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.path.enter_scope(Cow::Borrowed(variant));
        self.serialize_map(Some(len))
    }

    #[inline]
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Display,
    {
        let data = value.to_string();
        Ok(Value::String(data))
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        self.is_human_readable
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! end_seq_serializer {
    ($self:ident) => {{
        match ($self.brand, $self.list) {
            (Some(ArrayBrand::Byte), List::Empty) => Value::ByteArray(Vec::new()),
            (Some(ArrayBrand::Int), List::Empty) => Value::IntArray(Vec::new()),
            (Some(ArrayBrand::Long), List::Empty) => Value::LongArray(Vec::new()),

            (Some(ArrayBrand::Byte), List::Byte(vec)) => Value::ByteArray(vec),
            (Some(ArrayBrand::Int), List::Int(vec)) => Value::IntArray(vec),
            (Some(ArrayBrand::Long), List::Long(vec)) => Value::LongArray(vec),
            (None, list) => Value::List(list),

            _ => unreachable!("unreachable sequence serializer state"),
        }
    }};
}

////////////////////////////////////////////////////////////////////////////////

pub struct SeqSerializer<'a> {
    serializer: &'a mut Serializer,
    brand: Option<ArrayBrand>,
    len: Option<usize>,
    list: List,
    index: usize,
}

impl<'a> SeqSerializer<'a> {
    #[must_use]
    #[inline]
    fn new(serializer: &'a mut Serializer, brand: Option<ArrayBrand>, len: Option<usize>) -> Self {
        SeqSerializer {
            serializer,
            brand,
            len,
            list: List::Empty,
            index: 0,
        }
    }
}

impl<'a> ser::SerializeSeq for SeqSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serializer.path.enter_element(self.index);

        let value = value.serialize(&mut *self.serializer)?;

        if self.index == 0 {
            let kind = self.brand.map_or(value.kind(), ArrayBrand::element_kind);
            let capacity = self.len.unwrap_or_default();
            self.list = List::with_capacity_and_kind(capacity, kind);
        }

        if let Err(value) = self.list.push_checked(value) {
            return Err(Error::invalid_seq(
                value.kind().to_id(),
                self.list.id(),
                &mut self.serializer.path,
            ));
        }

        self.serializer.path.leave_element();
        self.index += 1;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        Ok(end_seq_serializer!(self))
    }
}

impl<'a> ser::SerializeTuple for SeqSerializer<'a> {
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

impl<'a> ser::SerializeTupleStruct for SeqSerializer<'a> {
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

impl<'a> ser::SerializeTupleVariant for SeqSerializer<'a> {
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
        let value = end_seq_serializer!(self);
        let key = self.serializer.path.leave_scope().into_owned();
        let mut compound = Compound::with_capacity(1);
        compound.insert(key, value);
        Ok(Value::Compound(compound))
    }
}

pub struct MapSerializer<'a> {
    serializer: &'a mut Serializer,
    compound: Compound,
    cached: Option<Cow<'static, str>>,
}

impl<'a> MapSerializer<'a> {
    fn new(serializer: &'a mut Serializer, len: Option<usize>) -> Self {
        MapSerializer {
            serializer,
            compound: Compound::with_capacity(len.unwrap_or_default()),
            cached: None,
        }
    }

    #[inline]
    fn handle_key<T>(&mut self, key: &T) -> Result<Cow<'static, str>>
    where
        T: ?Sized + Serialize,
    {
        self.serializer.path.enter_unresolved();
        let key = key.serialize(KeyQuery::new(
            &mut self.serializer.path,
            self.serializer.is_human_readable,
        ))?;
        self.serializer.path.leave_unresolved();
        Ok(key)
    }

    #[inline]
    fn handle_value<T>(&mut self, key: Cow<'static, str>, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serializer.path.enter_scope(key);
        let value = value.serialize(&mut *self.serializer)?;
        let key = self.serializer.path.leave_scope().into_owned();
        self.compound.insert(key, value);
        Ok(())
    }
}

impl<'a> ser::SerializeMap for MapSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize,
    {
        let key = self.handle_key(key)?;
        self.cached = Some(key);
        Ok(())
    }

    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let key = self
            .cached
            .take()
            .expect("`serialize_key` should get called before `serialize_value`");
        self.handle_value(key, value)
    }

    #[inline]
    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> std::result::Result<(), Self::Error>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        let key = self.handle_key(key)?;
        self.handle_value(key, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        Ok(Value::Compound(self.compound))
    }
}

impl<'a> ser::SerializeStruct for MapSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.handle_value(Cow::Borrowed(key), value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        Ok(Value::Compound(self.compound))
    }
}

impl<'a> ser::SerializeStructVariant for MapSerializer<'a> {
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
    fn end(self) -> Result<Self::Ok> {
        let value = Value::Compound(self.compound);
        let key = self.serializer.path.leave_scope().into_owned();
        let mut compound = Compound::with_capacity(1);
        compound.insert(key, value);
        Ok(Value::Compound(compound))
    }
}
