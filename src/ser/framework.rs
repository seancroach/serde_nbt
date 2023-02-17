use super::{Emit, EmitMap, EmitSeq};

use crate::{
    error::{Error, Path, Result, ZcResultExt},
    value::Id,
    ArrayBrand, SeqKind,
};

use alloc::borrow::Cow;
use core::{fmt::Display, str};

use serde::{ser, Serialize};

pub struct Serializer<E: Emit> {
    emitter: E,
    path: Path,
}

impl<E: Emit> Serializer<E> {
    pub(crate) fn new(emitter: E) -> Self {
        Serializer {
            emitter,
            path: Path::new(),
        }
    }

    pub(crate) fn handle<T>(&mut self, result: zc_io::Result<T>) -> Result<T> {
        result.attach_path(&mut self.path)
    }
}

impl<'a, E> ser::Serializer for &'a mut Serializer<E>
where
    E: Emit,
{
    type Ok = E::Output;
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a, E>;
    type SerializeTuple = SeqSerializer<'a, E>;
    type SerializeTupleStruct = SeqSerializer<'a, E>;
    type SerializeTupleVariant = TupleVariantSerializer<'a, E>;
    type SerializeMap = MapSerializer<'a, E>;
    type SerializeStruct = MapSerializer<'a, E>;
    type SerializeStructVariant = StructVariantSerializer<'a, E>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
        self.emitter.emit_bool(value).attach_path(&mut self.path)
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
        self.emitter.emit_i8(value).attach_path(&mut self.path)
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        self.emitter.emit_i16(value).attach_path(&mut self.path)
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        self.emitter.emit_i32(value).attach_path(&mut self.path)
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        self.emitter.emit_i64(value).attach_path(&mut self.path)
    }

    fn serialize_i128(self, _value: i128) -> Result<Self::Ok> {
        Err(Error::invalid_type("`i128`", &mut self.path))
    }

    fn serialize_u8(self, _value: u8) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u8`", &mut self.path))
    }

    fn serialize_u16(self, _value: u16) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u16`", &mut self.path))
    }

    fn serialize_u32(self, _value: u32) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u32`", &mut self.path))
    }

    fn serialize_u64(self, _value: u64) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u64`", &mut self.path))
    }

    fn serialize_u128(self, _value: u128) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u128`", &mut self.path))
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok> {
        self.emitter.emit_f32(value).attach_path(&mut self.path)
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok> {
        self.emitter.emit_f64(value).attach_path(&mut self.path)
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok> {
        let mut buf = [0; 4];
        self.serialize_str(value.encode_utf8(&mut buf))
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        self.emitter.emit_str(value).attach_path(&mut self.path)
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok> {
        Err(Error::invalid_type("`&[u8]`", &mut self.path))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::invalid_type("`None`", &mut self.path))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::invalid_type("`()`", &mut self.path))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(Error::invalid_type("unit struct", &mut self.path))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let mut ser = self.serialize_struct(name, 1)?;
        ser::SerializeStruct::serialize_field(&mut ser, variant, value)?;
        ser::SerializeStruct::end(ser)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SeqSerializer::new(self, None, len))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(SeqSerializer::new(self, None, Some(len)))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        let brand = ArrayBrand::from_str(name);
        Ok(SeqSerializer::new(self, brand, Some(len)))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        let mut outer = self.emitter.emit_map(Some(1)).attach_path(&mut self.path)?;

        let brand = ArrayBrand::from_str(name);
        let id = brand.map_or(Id::List, ArrayBrand::id);

        outer.begin_key(id).attach_path(&mut self.path)?;
        outer.emit_key(variant).attach_path(&mut self.path)?;
        outer.end_key().attach_path(&mut self.path)?;

        outer.begin_value().attach_path(&mut self.path)?;

        Ok(TupleVariantSerializer::new(self, outer, brand, Some(len)))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        let map = self.emitter.emit_map(len).attach_path(&mut self.path)?;
        Ok(MapSerializer::new(self, map))
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        let mut outer = self.emitter.emit_map(Some(1)).attach_path(&mut self.path)?;

        self.path.enter_scope(variant);

        outer.begin_key(Id::Compound).attach_path(&mut self.path)?;
        outer.emit_key(variant).attach_path(&mut self.path)?;
        outer.end_key().attach_path(&mut self.path)?;

        outer.begin_value().attach_path(&mut self.path)?;

        let inner = self
            .emitter
            .emit_map(Some(len))
            .attach_path(&mut self.path)?;
        Ok(StructVariantSerializer::new(self, outer, inner))
    }

    fn is_human_readable(&self) -> bool {
        self.emitter.is_human_readable()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct SeqSerializer<'a, E: Emit> {
    ser: &'a mut Serializer<E>,
    state: SeqState<E>,
    index: usize,
}

enum SeqState<E: Emit> {
    Init {
        brand: Option<ArrayBrand>,
        len: Option<usize>,
    },
    Rest {
        seq: E::EmitSeq,
        expected: Id,
    },
}

impl<'a, E: Emit> SeqSerializer<'a, E> {
    #[must_use]
    #[allow(clippy::similar_names)]
    fn new(ser: &'a mut Serializer<E>, brand: Option<ArrayBrand>, len: Option<usize>) -> Self {
        SeqSerializer {
            ser,
            state: SeqState::Init { brand, len },
            index: 0,
        }
    }

    fn initialize(&mut self, actual: Id) -> Result<()> {
        if let SeqState::Init { brand, len } = self.state {
            let kind = brand.map_or(SeqKind::List(actual), ArrayBrand::to_seq_kind);
            let seq = self
                .ser
                .emitter
                .emit_seq(kind, len)
                .attach_path(&mut self.ser.path)?;
            let expected = brand.map_or(actual, ArrayBrand::element_id);
            self.state = SeqState::Rest { seq, expected };
        }

        Ok(())
    }
}

impl<'a, E: Emit> ser::SerializeSeq for SeqSerializer<'a, E> {
    type Ok = E::Output;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.ser.path.enter_element(self.index);

        let actual = value.serialize(IdQuery::new(self.ser))?;
        self.initialize(actual)?;

        let SeqState::Rest { ref mut seq, expected } = self.state else { unreachable!() };

        if actual != expected {
            return Err(Error::invalid_seq(actual, expected, &mut self.ser.path));
        }

        seq.begin_element().attach_path(&mut self.ser.path)?;
        let value = value.serialize(&mut *self.ser)?;
        seq.handle_element(value)
            .and_then(|_| seq.end_element())
            .attach_path(&mut self.ser.path)?;

        let current = self.ser.path.leave_element();
        debug_assert_eq!(current, self.index);
        self.index += 1;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok> {
        self.initialize(Id::End)?;
        let SeqState::Rest { seq, .. } = self.state else { unreachable!() };
        seq.finish().attach_path(&mut self.ser.path)
    }
}

impl<'a, E: Emit> ser::SerializeTuple for SeqSerializer<'a, E> {
    type Ok = E::Output;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, E: Emit> ser::SerializeTupleStruct for SeqSerializer<'a, E> {
    type Ok = E::Output;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeSeq::end(self)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct TupleVariantSerializer<'a, E: Emit> {
    map: E::EmitMap,
    delegate: SeqSerializer<'a, E>,
}

impl<'a, E: Emit> TupleVariantSerializer<'a, E> {
    #[allow(clippy::similar_names)]
    fn new(
        ser: &'a mut Serializer<E>,
        map: E::EmitMap,
        brand: Option<ArrayBrand>,
        len: Option<usize>,
    ) -> Self {
        TupleVariantSerializer {
            map,
            delegate: SeqSerializer::new(ser, brand, len),
        }
    }
}

impl<'a, E: Emit> ser::SerializeTupleVariant for TupleVariantSerializer<'a, E> {
    type Ok = E::Output;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(&mut self.delegate, value)
    }

    fn end(mut self) -> Result<Self::Ok> {
        self.delegate.initialize(Id::End)?;
        let SeqState::Rest { seq, .. } = self.delegate.state else { unreachable!() };

        seq.finish()
            .and_then(|value| {
                self.map.handle_value(value)?;
                self.map.end_value()?;

                self.delegate.ser.path.leave_scope();

                self.map.finish()
            })
            .attach_path(&mut self.delegate.ser.path)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct MapSerializer<'a, E: Emit> {
    ser: &'a mut Serializer<E>,
    map: E::EmitMap,
    first: bool,
    key: Option<Cow<'static, str>>,
}

impl<'a, E: Emit> MapSerializer<'a, E> {
    #[must_use]
    fn new(ser: &'a mut Serializer<E>, map: E::EmitMap) -> Self {
        MapSerializer {
            ser,
            map,
            first: false,
            key: None,
        }
    }
}

impl<'a, E: Emit> ser::SerializeMap for MapSerializer<'a, E> {
    type Ok = E::Output;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.ser.path.enter_unresolved();
        let key = key.serialize(KeyQuery::new(self.ser))?;
        self.ser.path.leave_unresolved();

        self.key = Some(key);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // We temporarily enter the scope of the value so invalid errors resolve
        // correctly.
        self.ser.path.enter_scope(self.key.take().unwrap());
        let id = value.serialize(IdQuery::new(self.ser))?;
        let key = self.ser.path.leave_scope();

        self.map
            .begin_key(id)
            .and_then(|_| {
                self.map.emit_key(&key)?;
                self.map.end_key()?;

                self.ser.path.enter_scope(key);

                self.map.begin_value()
            })
            .attach_path(&mut self.ser.path)?;

        let output = value.serialize(&mut *self.ser)?;

        self.map
            .handle_value(output)
            .and_then(|_| self.map.end_value())
            .attach_path(&mut self.ser.path)?;

        self.ser.path.leave_scope();

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.map.finish().attach_path(&mut self.ser.path)
    }
}

impl<'a, E: Emit> ser::SerializeStruct for MapSerializer<'a, E> {
    type Ok = E::Output;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self::Ok> {
        ser::SerializeMap::end(self)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct StructVariantSerializer<'a, E: Emit> {
    map: E::EmitMap,
    delegate: MapSerializer<'a, E>,
}

impl<'a, E: Emit> StructVariantSerializer<'a, E> {
    fn new(ser: &'a mut Serializer<E>, outer: E::EmitMap, inner: E::EmitMap) -> Self {
        StructVariantSerializer {
            map: outer,
            delegate: MapSerializer::new(ser, inner),
        }
    }
}

impl<'a, E: Emit> ser::SerializeStructVariant for StructVariantSerializer<'a, E> {
    type Ok = E::Output;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeStruct::serialize_field(&mut self.delegate, key, value)
    }

    fn end(mut self) -> Result<Self::Ok> {
        self.delegate
            .map
            .finish()
            .and_then(|value| {
                self.map.handle_value(value)?;
                self.map.end_value()?;

                self.delegate.ser.path.leave_scope();

                self.map.finish()
            })
            .attach_path(&mut self.delegate.ser.path)
    }
}

////////////////////////////////////////////////////////////////////////////////

struct IdQuery<'a, E: Emit> {
    ser: &'a mut Serializer<E>,
}

impl<'a, E: Emit> IdQuery<'a, E> {
    #[must_use]
    fn new(ser: &'a mut Serializer<E>) -> Self {
        IdQuery { ser }
    }
}

impl<'a, E: Emit> ser::Serializer for IdQuery<'a, E> {
    type Ok = Id;
    type Error = Error;

    type SerializeSeq = NoOp<Self::Ok>;
    type SerializeTuple = NoOp<Self::Ok>;
    type SerializeTupleStruct = NoOp<Self::Ok>;
    type SerializeTupleVariant = NoOp<Self::Ok>;

    type SerializeMap = NoOp<Self::Ok>;
    type SerializeStruct = NoOp<Self::Ok>;
    type SerializeStructVariant = NoOp<Self::Ok>;

    fn serialize_bool(self, _value: bool) -> Result<Self::Ok> {
        Ok(Id::Byte)
    }

    fn serialize_i8(self, _value: i8) -> Result<Self::Ok> {
        Ok(Id::Byte)
    }

    fn serialize_i16(self, _value: i16) -> Result<Self::Ok> {
        Ok(Id::Short)
    }

    fn serialize_i32(self, _value: i32) -> Result<Self::Ok> {
        Ok(Id::Int)
    }

    fn serialize_i64(self, _value: i64) -> Result<Self::Ok> {
        Ok(Id::Long)
    }

    fn serialize_i128(self, _value: i128) -> Result<Self::Ok> {
        Err(Error::invalid_type("`i128`", &mut self.ser.path))
    }

    fn serialize_u8(self, _value: u8) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u8`", &mut self.ser.path))
    }

    fn serialize_u16(self, _value: u16) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u16`", &mut self.ser.path))
    }

    fn serialize_u32(self, _value: u32) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u32`", &mut self.ser.path))
    }

    fn serialize_u64(self, _value: u64) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u64`", &mut self.ser.path))
    }

    fn serialize_u128(self, _value: u128) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u128`", &mut self.ser.path))
    }

    fn serialize_f32(self, _value: f32) -> Result<Self::Ok> {
        Ok(Id::Float)
    }

    fn serialize_f64(self, _value: f64) -> Result<Self::Ok> {
        Ok(Id::Double)
    }

    fn serialize_char(self, _value: char) -> Result<Self::Ok> {
        Ok(Id::String)
    }

    fn serialize_str(self, _value: &str) -> Result<Self::Ok> {
        Ok(Id::String)
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok> {
        Err(Error::invalid_type("`&[u8]`", &mut self.ser.path))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::invalid_type("`None`", &mut self.ser.path))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::invalid_type("`()`", &mut self.ser.path))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(Error::invalid_type("unit struct", &mut self.ser.path))
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(Id::String)
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
        Ok(Id::Compound)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(NoOp::new(Id::List))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(NoOp::new(Id::List))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        let id = ArrayBrand::from_str(name).map_or(Id::List, ArrayBrand::id);
        Ok(NoOp::new(id))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(NoOp::new(Id::Compound))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(NoOp::new(Id::Compound))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(NoOp::new(Id::Compound))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(NoOp::new(Id::Compound))
    }

    fn collect_seq<I>(self, _iter: I) -> Result<Self::Ok>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        Ok(Id::List)
    }

    fn collect_map<K, V, I>(self, _iter: I) -> Result<Self::Ok>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        Ok(Id::Compound)
    }

    fn collect_str<T>(self, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Display,
    {
        Ok(Id::String)
    }

    fn is_human_readable(&self) -> bool {
        self.ser.is_human_readable()
    }
}

////////////////////////////////////////////////////////////////////////////////

struct NoOp<Ok> {
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

struct KeyQuery<'a, E: Emit> {
    ser: &'a mut Serializer<E>,
}

impl<'a, E: Emit> KeyQuery<'a, E> {
    #[must_use]
    fn new(ser: &'a mut Serializer<E>) -> Self {
        KeyQuery { ser }
    }
}

impl<'a, E: Emit> ser::Serializer for KeyQuery<'a, E> {
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
        Err(Error::invalid_key("`f32`", &mut self.ser.path))
    }

    fn serialize_f64(self, _value: f64) -> Result<Self::Ok> {
        Err(Error::invalid_key("`f64`", &mut self.ser.path))
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
            Err(_) => Err(Error::invalid_key("non-UTF-8 `&[u8]`", &mut self.ser.path)),
        }
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::invalid_key("`None`", &mut self.ser.path))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::invalid_key("`()`", &mut self.ser.path))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(Error::invalid_key("unit struct", &mut self.ser.path))
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
        Err(Error::invalid_key("newtype variant", &mut self.ser.path))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::invalid_key("seq", &mut self.ser.path))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::invalid_key("tuple", &mut self.ser.path))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::invalid_key("tuple struct", &mut self.ser.path))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::invalid_key("tuple variant", &mut self.ser.path))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::invalid_key("map", &mut self.ser.path))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::invalid_key("struct", &mut self.ser.path))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::invalid_key("struct variant", &mut self.ser.path))
    }

    fn collect_seq<I>(self, _iter: I) -> Result<Self::Ok>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        Err(Error::invalid_key("seq", &mut self.ser.path))
    }

    fn collect_map<K, V, I>(self, _iter: I) -> Result<Self::Ok>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        Err(Error::invalid_key("map", &mut self.ser.path))
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Display,
    {
        Ok(Cow::Owned(value.to_string()))
    }

    fn is_human_readable(&self) -> bool {
        self.ser.is_human_readable()
    }
}
