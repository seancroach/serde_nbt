use super::collect_key;

use crate::{
    error::{Error, Path, Result, ZcResultExt},
    util::{ArrayBrand, SeqKind},
    value::{Id, Kind},
};

use alloc::borrow::Cow;
use core::fmt::Display;

use serde::{ser, Serialize};
use zc_io::Write;

////////////////////////////////////////////////////////////////////////////////

macro_rules! tri_macro {
    ($self:ident) => {
        macro_rules! tri {
            ($expr:expr) => {{
                $expr.attach_path(&mut $self.path)?
            }};
        }
    };
}

////////////////////////////////////////////////////////////////////////////////

#[cfg_attr(doc_cfg, doc(cfg(feature = "emit")))]
pub struct RootSerializer<'header, 'writer, E, W>
where
    E: Emit,
    W: ?Sized + Write,
{
    header: &'header str,
    supports_list: bool,
    serializer: Serializer<'writer, E, W>,
}

impl<'header, 'writer, E, W> RootSerializer<'header, 'writer, E, W>
where
    E: Emit,
    W: ?Sized + Write,
{
    #[must_use]
    #[inline]
    pub fn new(
        header: &'header str,
        supports_list: bool,
        emitter: E,
        writer: &'writer mut W,
    ) -> Self {
        RootSerializer {
            header,
            supports_list,
            serializer: Serializer::new(emitter, writer),
        }
    }
}

impl<'ser, 'header, 'writer, E, W> ser::Serializer
    for &'ser mut RootSerializer<'header, 'writer, E, W>
where
    E: Emit,
    W: ?Sized + Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer<'ser, 'writer, E, W>;
    type SerializeTuple = SeqSerializer<'ser, 'writer, E, W>;
    type SerializeTupleStruct = SeqSerializer<'ser, 'writer, E, W>;
    type SerializeTupleVariant = SeqSerializer<'ser, 'writer, E, W>;

    type SerializeMap = MapSerializer<'ser, 'writer, E, W>;
    type SerializeStruct = MapSerializer<'ser, 'writer, E, W>;
    type SerializeStructVariant = MapSerializer<'ser, 'writer, E, W>;

    fn serialize_bool(self, _value: bool) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`bool`", &mut self.serializer.path))
    }

    fn serialize_i8(self, _value: i8) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`i8`", &mut self.serializer.path))
    }

    fn serialize_i16(self, _value: i16) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`i16`", &mut self.serializer.path))
    }

    fn serialize_i32(self, _value: i32) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`i32`", &mut self.serializer.path))
    }

    fn serialize_i64(self, _value: i64) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`i64`", &mut self.serializer.path))
    }

    fn serialize_i128(self, _value: i128) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`i128`", &mut self.serializer.path))
    }

    fn serialize_u8(self, _value: u8) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`u8`", &mut self.serializer.path))
    }

    fn serialize_u16(self, _value: u16) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`u16`", &mut self.serializer.path))
    }

    fn serialize_u32(self, _value: u32) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`u32`", &mut self.serializer.path))
    }

    fn serialize_u64(self, _value: u64) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`u64`", &mut self.serializer.path))
    }

    fn serialize_u128(self, _value: u128) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`u128`", &mut self.serializer.path))
    }

    fn serialize_f32(self, _value: f32) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`f32`", &mut self.serializer.path))
    }

    fn serialize_f64(self, _value: f64) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`f64`", &mut self.serializer.path))
    }

    fn serialize_char(self, _value: char) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`char`", &mut self.serializer.path))
    }

    fn serialize_str(self, _value: &str) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`&str`", &mut self.serializer.path))
    }

    fn serialize_bytes(self, _value: &[u8]) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type("`&[u8]`", &mut self.serializer.path))
    }

    #[inline]
    fn serialize_none(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.serializer.emit_header_id(Id::End)
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.serializer.emit_header_id(Id::End)
    }

    #[inline]
    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        self.serializer.emit_header_id(Id::End)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::invalid_type(
            "unit variant",
            &mut self.serializer.path,
        ))
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
        if self.supports_list {
            self.serializer.emit_header_id(Id::List)?;
            self.serializer
                .begin_variant(&Cow::Owned(self.header.to_owned()), Kind::List)?;
            SeqSerializer::new(&mut self.serializer, None, len, true)
        } else {
            Err(Error::invalid_root("sequence", &mut self.serializer.path))
        }
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> std::result::Result<Self::SerializeTuple, Self::Error> {
        if self.supports_list {
            self.serializer.emit_header_id(Id::List)?;
            self.serializer
                .begin_variant(&Cow::Owned(self.header.to_owned()), Kind::List)?;
            SeqSerializer::new(&mut self.serializer, None, Some(len), true)
        } else {
            Err(Error::invalid_root("tuple", &mut self.serializer.path))
        }
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeTupleStruct, Self::Error> {
        let brand = ArrayBrand::from_str(name);

        if self.supports_list && brand.is_none() {
            self.serializer.emit_header_id(Id::List)?;
            self.serializer
                .begin_variant(&Cow::Owned(self.header.to_owned()), Kind::List)?;
            SeqSerializer::new(&mut self.serializer, None, Some(len), true)
        } else {
            let brand = ArrayBrand::from_str(name);
            let display_type = brand.map_or("tuple struct", ArrayBrand::display);
            Err(Error::invalid_root(display_type, &mut self.serializer.path))
        }
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

        if self.supports_list {
            self.serializer.emit_header_id(Id::Compound)?;
            self.serializer
                .begin_variant(&Cow::Owned(self.header.to_owned()), Kind::Compound)?;

            let kind = brand.map_or(Kind::List, ArrayBrand::kind);
            self.serializer
                .begin_variant(&Cow::Borrowed(variant), kind)?;

            SeqSerializer::new(&mut self.serializer, brand, Some(len), true)
        } else {
            let brand = ArrayBrand::from_str(name);
            let display_type = brand.map_or("tuple variant", ArrayBrand::display);
            Err(Error::invalid_root(display_type, &mut self.serializer.path))
        }
    }

    #[inline]
    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> std::result::Result<Self::SerializeMap, Self::Error> {
        self.serializer
            .begin_variant(&Cow::Owned(self.header.to_owned()), Kind::Compound)?;
        MapSerializer::new(&mut self.serializer, true)
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeStruct, Self::Error> {
        self.serializer
            .begin_variant(&Cow::Owned(self.header.to_owned()), Kind::Compound)?;
        MapSerializer::new(&mut self.serializer, true)
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
        self.serializer
            .begin_variant(&Cow::Owned(self.header.to_owned()), Kind::Compound)?;
        self.serializer
            .begin_variant(&Cow::Borrowed(variant), Kind::Compound)?;
        MapSerializer::new(&mut self.serializer, true)
    }

    fn collect_str<T>(self, _value: &T) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Display,
    {
        Err(Error::invalid_root("`&str`", &mut self.serializer.path))
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        self.serializer.emitter.is_human_readable()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg_attr(doc_cfg, doc(cfg(feature = "emit")))]
pub struct Serializer<'writer, E, W>
where
    E: Emit,
    W: ?Sized + Write,
{
    emitter: E,
    writer: &'writer mut W,
    path: Path,
}

impl<'writer, E, W> Serializer<'writer, E, W>
where
    E: Emit,
    W: ?Sized + Write,
{
    #[must_use]
    #[inline]
    pub fn new(emitter: E, writer: &'writer mut W) -> Self {
        Serializer {
            emitter,
            writer,
            path: Path::new(),
        }
    }

    #[inline]
    #[allow(clippy::ptr_arg)]
    fn begin_variant(&mut self, variant: &Cow<'static, str>, hint: Kind) -> Result<()> {
        tri_macro!(self);

        tri!(self.emitter.begin_map(self.writer));

        self.path.enter_scope(variant.clone());

        tri!(self.emitter.before_key(self.writer, hint));
        tri!(self.emitter.emit_key(self.writer, variant));
        tri!(self.emitter.after_key(self.writer));

        tri!(self.emitter.before_value(self.writer));

        Ok(())
    }

    #[inline]
    fn end_variant(&mut self) -> Result<()> {
        tri_macro!(self);

        tri!(self.emitter.after_value(self.writer));
        self.path.leave_scope();

        tri!(self.emitter.end_map(self.writer));

        Ok(())
    }

    #[inline]
    fn begin_seq(&mut self, kind: SeqKind, len: Option<usize>) -> Result<()> {
        self.emitter
            .begin_seq(self.writer, kind, len)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn emit_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        tri_macro!(self);

        tri!(self.emitter.before_value(self.writer));
        value.serialize(&mut *self)?;
        tri!(self.emitter.after_value(self.writer));

        Ok(())
    }

    #[inline]
    fn end_seq(&mut self) -> Result<()> {
        self.emitter
            .end_seq(self.writer)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn begin_map(&mut self) -> Result<()> {
        self.emitter
            .begin_map(self.writer)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn query_kind<T>(&mut self, value: &T) -> Result<Kind>
    where
        T: ?Sized + Serialize,
    {
        let path = &mut self.path;
        let is_human_readable = self.emitter.is_human_readable();
        let serializer = KindSerializer::new(path, is_human_readable);
        value.serialize(serializer)
    }

    #[inline]
    fn collect_key<T>(&mut self, key: &T) -> Result<Cow<'static, str>>
    where
        T: ?Sized + Serialize,
    {
        collect_key(key, &mut self.path, self.emitter.is_human_readable())
    }

    #[inline]
    #[allow(clippy::ptr_arg)]
    fn handle_entry<T>(&mut self, key: &Cow<'static, str>, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        tri_macro!(self);

        self.path.enter_scope(key.clone());

        let kind = query_kind(value, &mut self.path, self.emitter.is_human_readable())?;

        tri!(self.emitter.before_key(self.writer, kind));
        tri!(self.emitter.emit_key(self.writer, key));
        tri!(self.emitter.after_key(self.writer));

        tri!(self.emitter.before_value(self.writer));
        value.serialize(&mut *self)?;
        tri!(self.emitter.after_value(self.writer));

        self.path.leave_scope();
        Ok(())
    }

    #[inline]
    fn end_map(&mut self) -> Result<()> {
        self.emitter
            .end_map(self.writer)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn emit_header_id(&mut self, id: Id) -> Result<()> {
        self.emitter
            .emit_header_id(self.writer, id)
            .attach_path(&mut self.path)
    }
}

impl<'ser, 'writer, E, W> ser::Serializer for &'ser mut Serializer<'writer, E, W>
where
    E: Emit,
    W: ?Sized + Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer<'ser, 'writer, E, W>;
    type SerializeTuple = SeqSerializer<'ser, 'writer, E, W>;
    type SerializeTupleStruct = SeqSerializer<'ser, 'writer, E, W>;
    type SerializeTupleVariant = SeqSerializer<'ser, 'writer, E, W>;

    type SerializeMap = MapSerializer<'ser, 'writer, E, W>;
    type SerializeStruct = MapSerializer<'ser, 'writer, E, W>;
    type SerializeStructVariant = MapSerializer<'ser, 'writer, E, W>;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
        self.emitter
            .emit_bool(self.writer, value)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
        self.emitter
            .emit_i8(self.writer, value)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        self.emitter
            .emit_i16(self.writer, value)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        self.emitter
            .emit_i32(self.writer, value)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        self.emitter
            .emit_i64(self.writer, value)
            .attach_path(&mut self.path)
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

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<Self::Ok> {
        self.emitter
            .emit_f32(self.writer, value)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<Self::Ok> {
        self.emitter
            .emit_f64(self.writer, value)
            .attach_path(&mut self.path)
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<Self::Ok> {
        let mut buf = [0; 4];
        self.serialize_str(value.encode_utf8(&mut buf))
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        self.emitter
            .emit_str(self.writer, value)
            .attach_path(&mut self.path)
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok> {
        Err(Error::invalid_type("`&[u8]`", &mut self.path))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::invalid_type("`None`", &mut self.path))
    }

    #[inline]
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

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(variant)
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
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let kind = self.query_kind(value)?;
        self.begin_variant(&Cow::Borrowed(variant), kind)?;
        value.serialize(&mut *self)?;
        self.end_variant()
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        SeqSerializer::new(self, None, len, false)
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        SeqSerializer::new(self, None, Some(len), false)
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        let brand = ArrayBrand::from_str(name);
        SeqSerializer::new(self, brand, Some(len), false)
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
        let kind = brand.map_or(Kind::List, ArrayBrand::kind);
        self.begin_variant(&Cow::Borrowed(variant), kind)?;
        SeqSerializer::new(self, brand, Some(len), false)
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
        self.begin_variant(&Cow::Borrowed(variant), Kind::Compound)?;
        MapSerializer::new(self, false)
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        self.emitter.is_human_readable()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg_attr(doc_cfg, doc(cfg(feature = "emit")))]
pub struct SeqSerializer<'ser, 'writer, E, W>
where
    E: Emit,
    W: ?Sized + Write,
{
    serializer: &'ser mut Serializer<'writer, E, W>,
    brand: Option<ArrayBrand>,
    len: Option<usize>,
    is_root: bool,

    expected: Option<Kind>,
    index: usize,
}

impl<'ser, 'writer, E, W> SeqSerializer<'ser, 'writer, E, W>
where
    E: Emit,
    W: ?Sized + Write,
{
    #[inline]
    fn new(
        serializer: &'ser mut Serializer<'writer, E, W>,
        brand: Option<ArrayBrand>,
        len: Option<usize>,
        is_root: bool,
    ) -> Result<Self> {
        if let Some(brand) = brand {
            let kind = brand.to_seq_kind();
            serializer.begin_seq(kind, len)?;
        }
        let expected = brand.map(ArrayBrand::element_kind);

        Ok(SeqSerializer {
            serializer,
            brand,
            len,
            is_root,

            expected,
            index: 0,
        })
    }
}

impl<'ser, 'writer, E, W> ser::SerializeSeq for SeqSerializer<'ser, 'writer, E, W>
where
    E: Emit,
    W: ?Sized + Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serializer.path.enter_element(self.index);

        let actual = self.serializer.query_kind(value)?;

        if let Some(expected) = self.expected {
            if actual != expected {
                return Err(Error::invalid_seq(
                    actual.to_id(),
                    expected.to_id(),
                    &mut self.serializer.path,
                ));
            }
        }

        if self.index == 0 && self.brand.is_none() {
            self.serializer.path.leave_element();
            self.serializer
                .begin_seq(SeqKind::List(actual.to_id()), self.len)?;
            self.serializer.path.enter_element(self.index);
        }

        self.serializer.emit_element(value)?;

        self.serializer.path.leave_element();
        self.index += 1;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        if self.index == 0 && self.brand.is_none() {
            self.serializer.begin_seq(SeqKind::default(), Some(0))?;
        }

        self.serializer.end_seq()?;

        if self.is_root {
            self.serializer.end_variant()?;
        }

        Ok(())
    }
}

impl<'ser, 'writer, E, W> ser::SerializeTuple for SeqSerializer<'ser, 'writer, E, W>
where
    E: Emit,
    W: ?Sized + Write,
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
        ser::SerializeSeq::end(self)
    }
}

impl<'ser, 'writer, E, W> ser::SerializeTupleStruct for SeqSerializer<'ser, 'writer, E, W>
where
    E: Emit,
    W: ?Sized + Write,
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
        ser::SerializeSeq::end(self)
    }
}

impl<'ser, 'writer, E, W> ser::SerializeTupleVariant for SeqSerializer<'ser, 'writer, E, W>
where
    E: Emit,
    W: ?Sized + Write,
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
        if self.index == 0 && self.brand.is_none() {
            self.serializer.begin_seq(SeqKind::default(), Some(0))?;
        }

        self.serializer.end_seq()?;
        self.serializer.end_variant()?;

        if self.is_root {
            self.serializer.end_variant()?;
        }

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg_attr(doc_cfg, doc(cfg(feature = "emit")))]
pub struct MapSerializer<'ser, 'writer, E, W>
where
    E: Emit,
    W: ?Sized + Write,
{
    serializer: &'ser mut Serializer<'writer, E, W>,
    cached: Option<Cow<'static, str>>,
    is_root: bool,
}

impl<'ser, 'writer, E, W> MapSerializer<'ser, 'writer, E, W>
where
    E: Emit,
    W: ?Sized + Write,
{
    #[inline]
    fn new(serializer: &'ser mut Serializer<'writer, E, W>, is_root: bool) -> Result<Self> {
        serializer.begin_map()?;
        Ok(MapSerializer {
            serializer,
            cached: None,
            is_root,
        })
    }
}

impl<'ser, 'writer, E, W> ser::SerializeMap for MapSerializer<'ser, 'writer, E, W>
where
    E: Emit,
    W: ?Sized + Write,
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
    #[track_caller]
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let key = self
            .cached
            .take()
            .expect("`serialize_key` must get called before `serialize_value`");
        self.serializer.handle_entry(&key, value)
    }

    #[inline]
    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<()>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        let key = self.serializer.collect_key(key)?;
        self.serializer.handle_entry(&key, value)
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

impl<'ser, 'writer, E, W> ser::SerializeStruct for MapSerializer<'ser, 'writer, E, W>
where
    E: Emit,
    W: ?Sized + Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serializer.handle_entry(&Cow::Borrowed(key), value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok> {
        ser::SerializeMap::end(self)
    }
}

impl<'ser, 'writer, E, W> ser::SerializeStructVariant for MapSerializer<'ser, 'writer, E, W>
where
    E: Emit,
    W: ?Sized + Write,
{
    type Ok = ();
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
        self.serializer.end_map()?;
        self.serializer.end_variant()?;

        if self.is_root {
            self.serializer.end_variant()?;
        }

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg_attr(doc_cfg, doc(cfg(feature = "emit")))]
#[allow(clippy::missing_errors_doc)]
pub trait Emit {
    fn emit_header_id<W>(&mut self, writer: &mut W, id: Id) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

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

    fn before_key<W>(&mut self, writer: &mut W, hint: Kind) -> zc_io::Result<()>
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

////////////////////////////////////////////////////////////////////////////////

fn query_kind<T>(value: &T, path: &mut Path, is_human_readable: bool) -> Result<Kind>
where
    T: ?Sized + Serialize,
{
    let serializer = KindSerializer::new(path, is_human_readable);
    value.serialize(serializer)
}

struct KindSerializer<'path> {
    path: &'path mut Path,
    is_human_readable: bool,
}

impl<'path> KindSerializer<'path> {
    #[must_use]
    #[inline]
    fn new(path: &'path mut Path, is_human_readable: bool) -> Self {
        KindSerializer {
            path,
            is_human_readable,
        }
    }
}

impl<'path> ser::Serializer for KindSerializer<'path> {
    type Ok = Kind;
    type Error = Error;

    type SerializeSeq = NoOp<Self::Ok>;
    type SerializeTuple = NoOp<Self::Ok>;
    type SerializeTupleStruct = NoOp<Self::Ok>;
    type SerializeTupleVariant = NoOp<Self::Ok>;
    type SerializeMap = NoOp<Self::Ok>;
    type SerializeStruct = NoOp<Self::Ok>;
    type SerializeStructVariant = NoOp<Self::Ok>;

    #[inline]
    fn serialize_bool(self, _value: bool) -> Result<Self::Ok> {
        Ok(Kind::Byte)
    }

    #[inline]
    fn serialize_i8(self, _value: i8) -> Result<Self::Ok> {
        Ok(Kind::Byte)
    }

    #[inline]
    fn serialize_i16(self, _value: i16) -> Result<Self::Ok> {
        Ok(Kind::Short)
    }

    #[inline]
    fn serialize_i32(self, _value: i32) -> Result<Self::Ok> {
        Ok(Kind::Int)
    }

    #[inline]
    fn serialize_i64(self, _value: i64) -> Result<Self::Ok> {
        Ok(Kind::Long)
    }

    fn serialize_i128(self, _value: i128) -> Result<Self::Ok> {
        Err(Error::invalid_type("`i128`", self.path))
    }

    fn serialize_u8(self, _value: u8) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u8`", self.path))
    }

    fn serialize_u16(self, _value: u16) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u16`", self.path))
    }

    fn serialize_u32(self, _value: u32) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u32`", self.path))
    }

    fn serialize_u64(self, _value: u64) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u64`", self.path))
    }

    fn serialize_u128(self, _value: u128) -> Result<Self::Ok> {
        Err(Error::invalid_type("`u128`", self.path))
    }

    #[inline]
    fn serialize_f32(self, _value: f32) -> Result<Self::Ok> {
        Ok(Kind::Float)
    }

    #[inline]
    fn serialize_f64(self, _value: f64) -> Result<Self::Ok> {
        Ok(Kind::Double)
    }

    #[inline]
    fn serialize_char(self, _value: char) -> Result<Self::Ok> {
        Ok(Kind::String)
    }

    #[inline]
    fn serialize_str(self, _value: &str) -> Result<Self::Ok> {
        Ok(Kind::String)
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok> {
        Err(Error::invalid_type("`&[u8]`", self.path))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::invalid_type("`None`", self.path))
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::invalid_type("`()`", self.path))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(Error::invalid_type("unit struct", self.path))
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(Kind::String)
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
        Ok(Kind::Compound)
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(NoOp::new(Kind::List))
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(NoOp::new(Kind::List))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        let brand = ArrayBrand::from_str(name);
        let kind = brand.map_or(Kind::List, ArrayBrand::kind);
        Ok(NoOp::new(kind))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        let brand = ArrayBrand::from_str(name);
        let kind = brand.map_or(Kind::List, ArrayBrand::kind);
        Ok(NoOp::new(kind))
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(NoOp::new(Kind::Compound))
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(NoOp::new(Kind::Compound))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(NoOp::new(Kind::Compound))
    }

    #[inline]
    fn collect_seq<I>(self, _iter: I) -> Result<Self::Ok>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        Ok(Kind::List)
    }

    #[inline]
    fn collect_map<K, V, I>(self, _iter: I) -> Result<Self::Ok>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        Ok(Kind::Compound)
    }

    #[inline]
    fn collect_str<T>(self, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Display,
    {
        Ok(Kind::String)
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        self.is_human_readable
    }
}

////////////////////////////////////////////////////////////////////////////////

struct NoOp<Ok> {
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
