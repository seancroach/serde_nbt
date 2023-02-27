use super::mutf8;

use crate::{
    emit::{Emit, MapSerializer, SeqSerializer, Serializer},
    error::{Error, Result, ZcResultExt},
    value::{Id, ValueKind},
    SeqKind,
};

use serde::{ser, Serialize};
use zc_io::Write;

////////////////////////////////////////////////////////////////////////////////

pub struct RootSerializer<'header, E, W> {
    header: Option<&'header str>,
    supports_list: bool,
    serializer: Serializer<BinaryEmitter<E>, W>,
}

impl<E, W> Serializer<BinaryEmitter<E>, W>
where
    E: Encode,
    W: Write,
{
    #[inline]
    fn encode_id(&mut self, id: Id) -> Result<()> {
        self.emitter
            .encoder
            .encode_id(&mut self.writer, id)
            .attach_path(&mut self.path)
    }
}

impl<'a, 'header, E, W> ser::Serializer for &'a mut RootSerializer<'header, E, W>
where
    E: Encode,
    W: Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a, BinaryEmitter<E>, W>;
    type SerializeTuple = SeqSerializer<'a, BinaryEmitter<E>, W>;
    type SerializeTupleStruct = SeqSerializer<'a, BinaryEmitter<E>, W>;
    type SerializeTupleVariant = SeqSerializer<'a, BinaryEmitter<E>, W>;

    type SerializeMap = MapSerializer<'a, BinaryEmitter<E>, W>;
    type SerializeStruct = MapSerializer<'a, BinaryEmitter<E>, W>;
    type SerializeStructVariant = MapSerializer<'a, BinaryEmitter<E>, W>;

    fn serialize_bool(mut self, _value: bool) -> Result<Self::Ok> {
        Err(Error::invalid_root("bool", &mut self.serializer.path))
    }

    fn serialize_i8(mut self, _value: i8) -> Result<Self::Ok> {
        Err(Error::invalid_root("i8", &mut self.serializer.path))
    }

    fn serialize_i16(mut self, _value: i16) -> Result<Self::Ok> {
        Err(Error::invalid_root("i16", &mut self.serializer.path))
    }

    fn serialize_i32(mut self, _value: i32) -> Result<Self::Ok> {
        Err(Error::invalid_root("i32", &mut self.serializer.path))
    }

    fn serialize_i64(mut self, _value: i64) -> Result<Self::Ok> {
        Err(Error::invalid_root("i64", &mut self.serializer.path))
    }

    fn serialize_i128(mut self, _value: i128) -> Result<Self::Ok> {
        Err(Error::invalid_type("i128", &mut self.serializer.path))
    }

    fn serialize_u8(mut self, _value: u8) -> Result<Self::Ok> {
        Err(Error::invalid_type("u8", &mut self.serializer.path))
    }

    fn serialize_u16(mut self, _value: u16) -> Result<Self::Ok> {
        Err(Error::invalid_type("u16", &mut self.serializer.path))
    }

    fn serialize_u32(mut self, _value: u32) -> Result<Self::Ok> {
        Err(Error::invalid_type("u32", &mut self.serializer.path))
    }

    fn serialize_u64(mut self, _value: u64) -> Result<Self::Ok> {
        Err(Error::invalid_type("u64", &mut self.serializer.path))
    }

    fn serialize_u128(mut self, _value: u128) -> Result<Self::Ok> {
        Err(Error::invalid_type("u128", &mut self.serializer.path))
    }

    fn serialize_f32(mut self, _value: f32) -> Result<Self::Ok> {
        Err(Error::invalid_root("f32", &mut self.serializer.path))
    }

    fn serialize_f64(mut self, _value: f64) -> Result<Self::Ok> {
        Err(Error::invalid_root("f64", &mut self.serializer.path))
    }

    fn serialize_char(mut self, _value: char) -> Result<Self::Ok> {
        Err(Error::invalid_root("char", &mut self.serializer.path))
    }

    fn serialize_str(mut self, _value: &str) -> Result<Self::Ok> {
        Err(Error::invalid_root("&str", &mut self.serializer.path))
    }

    fn serialize_bytes(mut self, _value: &[u8]) -> Result<Self::Ok> {
        Err(Error::invalid_type("&[u8]", &mut self.serializer.path))
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok> {
        self.serializer.encode_id(Id::End)
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok> {
        self.serializer.encode_id(Id::End)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serializer.encode_id(Id::End)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        Err(Error::invalid_root(
            "unit variant",
            &mut self.serializer.path,
        ))
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
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        self.serializer.encode_id(Id::Compound)?;
        self.serializer
            .serialize_newtype_variant(name, variant_index, variant, value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {}

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

    #[inline]
    fn is_human_readable(&self) -> bool {
        self.serializer.emitter.is_human_readable()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct BinaryEmitter<E> {
    encoder: E,
}

impl<E> BinaryEmitter<E> {
    #[must_use]
    #[inline]
    pub fn new(encoder: E) -> Self {
        BinaryEmitter { encoder }
    }
}

impl<E> Emit for BinaryEmitter<E>
where
    E: Encode,
{
    #[inline]
    fn emit_bool<W>(&mut self, writer: &mut W, value: bool) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.emit_i8(writer, value.into())
    }

    #[inline]
    fn emit_i8<W>(&mut self, writer: &mut W, value: i8) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.encoder.encode_i8(writer, value)
    }

    #[inline]
    fn emit_i16<W>(&mut self, writer: &mut W, value: i16) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.encoder.encode_i16(writer, value)
    }

    #[inline]
    fn emit_i32<W>(&mut self, writer: &mut W, value: i32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.encoder.encode_i32(writer, value)
    }

    #[inline]
    fn emit_i64<W>(&mut self, writer: &mut W, value: i64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.encoder.encode_i64(writer, value)
    }

    #[inline]
    fn emit_f32<W>(&mut self, writer: &mut W, value: f32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.encoder.encode_f32(writer, value)
    }

    #[inline]
    fn emit_f64<W>(&mut self, writer: &mut W, value: f64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.encoder.encode_f64(writer, value)
    }

    #[inline]
    fn emit_str<W>(&mut self, writer: &mut W, value: &str) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        let encoded = mutf8::encode(value);
        self.encoder.encode_str_len(writer, encoded.len())?;
        writer.write_all(&encoded)
    }

    #[inline]
    fn begin_seq<W>(
        &mut self,
        writer: &mut W,
        kind: SeqKind,
        len: Option<usize>,
    ) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        if let SeqKind::List(id) = kind {
            self.encoder.encode_id(writer, id)?;
        }

        if let Some(len) = len {
            self.encoder.encode_seq_len(writer, len)
        } else {
            Err(zc_io::error!(
                InvalidInput,
                "`serde_nbt` binary formats do not support unsized sequences"
            ))
        }
    }

    #[inline]
    fn before_element<W>(&mut self, _writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        Ok(())
    }

    #[inline]
    fn after_element<W>(&mut self, _writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        Ok(())
    }

    #[inline]
    fn end_seq<W>(&mut self, _writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        Ok(())
    }

    #[inline]
    fn begin_map<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.encoder.encode_id(writer, Id::Compound)
    }

    #[inline]
    fn before_key<W>(&mut self, writer: &mut W, hint: ValueKind) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.encoder.encode_id(writer, hint.to_id())
    }

    #[inline]
    fn emit_key<W>(&mut self, writer: &mut W, key: &str) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.emit_str(writer, key)
    }

    #[inline]
    fn after_key<W>(&mut self, _riter: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        Ok(())
    }

    #[inline]
    fn before_value<W>(&mut self, _writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        Ok(())
    }

    #[inline]
    fn after_value<W>(&mut self, _writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        Ok(())
    }

    #[inline]
    fn end_map<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.encoder.encode_id(writer, Id::End)
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait Encode {
    fn encode_id<W>(&mut self, writer: &mut W, id: Id) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn encode_i8<W>(&mut self, writer: &mut W, value: i8) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn encode_i16<W>(&mut self, writer: &mut W, value: i16) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn encode_i32<W>(&mut self, writer: &mut W, value: i32) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn encode_i64<W>(&mut self, writer: &mut W, value: i64) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn encode_f32<W>(&mut self, writer: &mut W, value: f32) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn encode_f64<W>(&mut self, writer: &mut W, value: f64) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn encode_str_len<W>(&mut self, writer: &mut W, value: usize) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn encode_seq_len<W>(&mut self, writer: &mut W, value: usize) -> zc_io::Result<()>
    where
        W: ?Sized + Write;
}
