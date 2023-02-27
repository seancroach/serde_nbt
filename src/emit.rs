use crate::{
    error::{Error, Path, Result, ZcResultExt},
    ser::{collect_key, query_kind, serialize_invalid_types, serialize_passthroughs},
    value::ValueKind,
    ArrayBrand, SeqKind,
};

use alloc::borrow::Cow;

use serde::{ser, Serialize};
use zc_io::Write;

pub struct Serializer<E, W> {
    pub(crate) emitter: E,
    pub(crate) writer: W,
    pub(crate) path: Path,
}

impl<E, W> Serializer<E, W>
where
    E: Emit,
    W: Write,
{
    #[must_use]
    #[inline]
    pub(crate) fn new(emitter: E, writer: W) -> Self {
        Serializer {
            emitter,
            writer,
            path: Path::new(),
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Enum Utilities
    ////////////////////////////////////////////////////////////////////////////

    #[inline]
    fn begin_variant(&mut self, hint: ValueKind, variant: &'static str) -> Result<()> {
        self.begin_map()?;

        self.path.enter_scope(Cow::Borrowed(variant));

        self.before_key(hint)?;
        self.emit_key(variant)?;
        self.after_key()?;

        self.before_value()
    }

    #[inline]
    fn end_variant(&mut self) -> Result<()> {
        self.after_value()?;
        self.path.leave_scope();
        self.end_map()
    }

    ////////////////////////////////////////////////////////////////////////////
    // Seq Utilities
    ////////////////////////////////////////////////////////////////////////////

    #[inline]
    fn emit_element<T>(
        &mut self,
        index: usize,
        brand: Option<ArrayBrand>,
        len: Option<usize>,
        expected: &mut Option<ValueKind>,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.path.enter_element(index);

        let actual = self.query_kind(value)?;

        if index == 0 && brand.is_none() {
            self.path.leave_element();
            let kind = brand.map_or(SeqKind::List(actual.to_id()), ArrayBrand::to_seq_kind);
            self.begin_seq(kind, len)?;
            self.path.enter_element(index);
        }

        if let Some(expected) = *expected {
            if actual != expected {
                return Err(Error::invalid_seq(
                    actual.to_id(),
                    expected.to_id(),
                    &mut self.path,
                ));
            }
        } else {
            *expected = Some(actual);
        }

        self.before_element()?;
        value.serialize(self)?;
        self.after_element()?;

        self.path.leave_element();

        Ok(())
    }

    #[inline]
    fn finish_seq(&mut self, index: usize, brand: Option<ArrayBrand>) -> Result<()> {
        if index == 0 && brand.is_none() {
            self.begin_seq(Default::default(), Some(0))?;
        }
        self.end_seq()
    }

    ////////////////////////////////////////////////////////////////////////////
    // Map Utilities
    ////////////////////////////////////////////////////////////////////////////

    #[inline]
    fn collect_key<T>(&mut self, key: &T) -> Result<Cow<'static, str>>
    where
        T: ?Sized + Serialize,
    {
        self.path.enter_unresolved();
        let key = collect_key(key, &mut self.path, self.emitter.is_human_readable())?;
        self.path.leave_unresolved();

        Ok(key)
    }

    #[inline]
    fn emit_entry<T>(&mut self, key: Cow<'static, str>, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.path.enter_scope(key);

        let hint = self.query_kind(value)?;

        let key = self.path.leave_scope();

        self.before_key(hint)?;
        self.emit_key(&key)?;
        self.after_key()?;

        self.path.enter_scope(key);

        self.before_value()?;
        value.serialize(self)?;
        self.after_value()?;

        self.path.leave_scope();

        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////
    // Emit Utilities
    ////////////////////////////////////////////////////////////////////////////

    #[inline]
    fn emit_bool(&mut self, value: bool) -> Result<()> {
        self.emitter
            .emit_bool(&mut self.writer, value)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn emit_i8(&mut self, value: i8) -> Result<()> {
        self.emitter
            .emit_i8(&mut self.writer, value)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn emit_i16(&mut self, value: i16) -> Result<()> {
        self.emitter
            .emit_i16(&mut self.writer, value)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn emit_i32(&mut self, value: i32) -> Result<()> {
        self.emitter
            .emit_i32(&mut self.writer, value)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn emit_i64(&mut self, value: i64) -> Result<()> {
        self.emitter
            .emit_i64(&mut self.writer, value)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn emit_f32(&mut self, value: f32) -> Result<()> {
        self.emitter
            .emit_f32(&mut self.writer, value)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn emit_f64(&mut self, value: f64) -> Result<()> {
        self.emitter
            .emit_f64(&mut self.writer, value)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn emit_str(&mut self, value: &str) -> Result<()> {
        self.emitter
            .emit_str(&mut self.writer, value)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn begin_seq(&mut self, kind: SeqKind, len: Option<usize>) -> Result<()> {
        self.emitter
            .begin_seq(&mut self.writer, kind, len)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn before_element(&mut self) -> Result<()> {
        self.emitter
            .before_element(&mut self.writer)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn after_element(&mut self) -> Result<()> {
        self.emitter
            .after_element(&mut self.writer)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn end_seq(&mut self) -> Result<()> {
        self.emitter
            .end_seq(&mut self.writer)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn begin_map(&mut self) -> Result<()> {
        self.emitter
            .begin_map(&mut self.writer)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn before_key(&mut self, hint: ValueKind) -> Result<()> {
        self.emitter
            .before_key(&mut self.writer, hint)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn emit_key(&mut self, key: &str) -> Result<()> {
        self.emitter
            .emit_key(&mut self.writer, key)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn after_key(&mut self) -> Result<()> {
        self.emitter
            .after_key(&mut self.writer)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn before_value(&mut self) -> Result<()> {
        self.emitter
            .after_key(&mut self.writer)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn after_value(&mut self) -> Result<()> {
        self.emitter
            .after_value(&mut self.writer)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn end_map(&mut self) -> Result<()> {
        self.emitter
            .end_seq(&mut self.writer)
            .attach_path(&mut self.path)
    }

    ////////////////////////////////////////////////////////////////////////////////

    #[inline]
    fn query_kind<T>(&mut self, value: &T) -> Result<ValueKind>
    where
        T: ?Sized + Serialize,
    {
        query_kind(value, &mut self.path, self.emitter.is_human_readable())
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<'a, E, W> ser::Serializer for &'a mut Serializer<E, W>
where
    E: Emit,
    W: Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a, E, W>;
    type SerializeTuple = SeqSerializer<'a, E, W>;
    type SerializeTupleStruct = SeqSerializer<'a, E, W>;
    type SerializeTupleVariant = SeqSerializer<'a, E, W>;

    type SerializeMap = MapSerializer<'a, E, W>;
    type SerializeStruct = MapSerializer<'a, E, W>;
    type SerializeStructVariant = MapSerializer<'a, E, W>;

    serialize_invalid_types!();
    serialize_passthroughs!();

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
        self.emit_bool(value)
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
        self.emit_i8(value)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        self.emit_i16(value)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        self.emit_i32(value)
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        self.emit_i64(value)
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<Self::Ok> {
        self.emit_f32(value)
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<Self::Ok> {
        self.emit_f64(value)
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        self.emit_str(value)
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        let hint = self.query_kind(value)?;

        self.begin_variant(hint, variant)?;
        value.serialize(self)?;
        self.end_variant()
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        SeqSerializer::new(self, None, len)
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        SeqSerializer::new(self, None, Some(len))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        let brand = ArrayBrand::from_str(name);
        SeqSerializer::new(self, brand, Some(len))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        let brand = ArrayBrand::from_str(name);
        let hint = brand.map_or(ValueKind::List, ArrayBrand::kind);

        self.begin_variant(hint, variant)?;

        SeqSerializer::new(self, brand, Some(len))
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        MapSerializer::new(self, false)
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        MapSerializer::new(self, false)
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.begin_variant(ValueKind::Compound, variant)?;
        MapSerializer::new(self, false)
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        self.emitter.is_human_readable()
    }
}

pub struct SeqSerializer<'a, E, W> {
    serializer: &'a mut Serializer<E, W>,
    brand: Option<ArrayBrand>,
    len: Option<usize>,
    expected: Option<ValueKind>,
    index: usize,
}

impl<'a, E, W> SeqSerializer<'a, E, W>
where
    E: Emit,
    W: Write,
{
    #[inline]
    fn new(
        serializer: &'a mut Serializer<E, W>,
        brand: Option<ArrayBrand>,
        len: Option<usize>,
    ) -> Result<Self> {
        let expected = if let Some(brand) = brand {
            serializer.begin_seq(brand.to_seq_kind(), len)?;
            Some(brand.element_kind())
        } else {
            None
        };

        Ok(SeqSerializer {
            serializer,
            brand,
            len,
            expected,
            index: 0,
        })
    }
}

impl<'a, E, W> ser::SerializeSeq for SeqSerializer<'a, E, W>
where
    E: Emit,
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serializer.emit_element(
            self.index,
            self.brand,
            self.len,
            &mut self.expected,
            value,
        )?;
        self.index += 1;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        self.serializer.finish_seq(self.index, self.brand)
    }
}

impl<'a, E, W> ser::SerializeTuple for SeqSerializer<'a, E, W>
where
    E: Emit,
    W: Write,
{
    type Ok = ();
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
        ser::SerializeTuple::end(self)
    }
}

impl<'a, E, W> ser::SerializeTupleStruct for SeqSerializer<'a, E, W>
where
    E: Emit,
    W: Write,
{
    type Ok = ();
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
        ser::SerializeTuple::end(self)
    }
}

impl<'a, E, W> ser::SerializeTupleVariant for SeqSerializer<'a, E, W>
where
    E: Emit,
    W: Write,
{
    type Ok = ();
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
        self.serializer.finish_seq(self.index, self.brand)?;
        self.serializer.end_variant()
    }
}

pub struct MapSerializer<'a, E, W> {
    serializer: &'a mut Serializer<E, W>,
    cached: Option<Cow<'static, str>>,
    is_root: bool,
}

impl<'a, E, W> MapSerializer<'a, E, W>
where
    E: Emit,
    W: Write,
{
    #[inline]
    fn new(serializer: &'a mut Serializer<E, W>, is_root: bool) -> Result<Self> {
        serializer.begin_map()?;
        Ok(MapSerializer {
            serializer,
            cached: None,
            is_root,
        })
    }
}

impl<'a, E, W> ser::SerializeMap for MapSerializer<'a, E, W>
where
    E: Emit,
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let key = self.serializer.collect_key(key)?;
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
        self.serializer.emit_entry(key, value)
    }

    #[inline]
    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<()>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        let key = self.serializer.collect_key(key)?;
        self.serializer.emit_entry(key, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        self.serializer.end_map()?;

        if self.is_root {
            self.serializer.end_variant()?;
        }

        Ok(())
    }
}

impl<'a, E, W> ser::SerializeStruct for MapSerializer<'a, E, W>
where
    E: Emit,
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serializer.emit_entry(Cow::Borrowed(key), value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        ser::SerializeMap::end(self)
    }
}

impl<'a, E, W> ser::SerializeStructVariant for MapSerializer<'a, E, W>
where
    E: Emit,
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serializer.emit_entry(Cow::Borrowed(key), value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        self.serializer.end_map()?;
        self.serializer.end_variant()?;

        if self.is_root {
            self.serializer.end_variant()?;
        }

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[allow(clippy::missing_errors_doc)]
pub trait Emit {
    fn emit_bool<W>(&mut self, writer: &mut W, value: bool) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn emit_i8<W>(&mut self, writer: &mut W, value: i8) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn emit_i16<W>(&mut self, writer: &mut W, value: i16) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn emit_i32<W>(&mut self, writer: &mut W, value: i32) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn emit_i64<W>(&mut self, writer: &mut W, value: i64) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn emit_f32<W>(&mut self, writer: &mut W, value: f32) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn emit_f64<W>(&mut self, writer: &mut W, value: f64) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn emit_str<W>(&mut self, writer: &mut W, value: &str) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn begin_seq<W>(
        &mut self,
        writer: &mut W,
        kind: SeqKind,
        len: Option<usize>,
    ) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn before_element<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn after_element<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn end_seq<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn begin_map<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn before_key<W>(&mut self, writer: &mut W, hint: ValueKind) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn emit_key<W>(&mut self, writer: &mut W, key: &str) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn after_key<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn before_value<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn after_value<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn end_map<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    #[must_use]
    fn is_human_readable(&self) -> bool;
}
