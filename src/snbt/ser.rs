use super::SeqKind;

use crate::{
    error::{Error, Path, Result},
    str::EscapeJava,
    TypeId,
};

use core::fmt::Display;

use serde::{ser, Serialize};

pub struct ValueSerializer<'a, W, F = CompactFormatter>
where
    W: ?Sized + Write,
    F: ?Sized + Formatter,
{
    writer: &'a mut W,
    formatter: &'a mut F,
    path: &'a Path,
}

impl<'a, W, F> ser::Serializer for ValueSerializer<'a, W, F>
where
    W: ?Sized + Write,
    F: ?Sized + Formatter,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = ();
    type SerializeTuple = ();
    type SerializeTupleStruct = ();
    type SerializeTupleVariant = ();

    type SerializeMap = ();
    type SerializeStruct = ();
    type SerializeStructVariant = ();

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, v: i64) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i128(self, v: i128) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, v: u16) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, v: u64) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u128(self, v: u128) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, v: char) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(
        self,
        name: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> std::result::Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> std::result::Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> std::result::Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }

    fn collect_seq<I>(self, iter: I) -> std::result::Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        todo!()
    }

    fn collect_map<K, V, I>(self, iter: I) -> std::result::Result<Self::Ok, Self::Error>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        todo!()
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Display,
    {
        todo!()
    }

    fn is_human_readable(&self) -> bool {
        todo!()
    }
}

pub struct SeqSerializer<'a, W, F>
where
    W: ?Sized + Write,
    F: ?Sized + Formatter,
{
    writer: &'a mut W,
    formatter: &'a mut F,
    path: &'a mut Path,
    expecting: Option<TypeId>,
    is_root: bool,
}

pub struct PrettyFormatter<'a> {
    delegate: CompactFormatter,
    indent_depth: usize,
    indent: &'a str,
}

impl<'a> PrettyFormatter<'a> {
    #[inline]
    fn newline_indent<W>(&self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        for _ in 0..self.indent_depth {
            writer.write_str(self.indent)?;
        }
        Ok(())
    }
}

impl<'a> Formatter for PrettyFormatter<'a> {
    #[inline]
    fn fmt_bool<W>(&mut self, writer: &mut W, value: bool) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.fmt_bool(writer, value)
    }

    #[inline]
    fn fmt_i8<W>(&mut self, writer: &mut W, value: i8) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.fmt_i8(writer, value)
    }

    #[inline]
    fn fmt_i16<W>(&mut self, writer: &mut W, value: i16) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.fmt_i16(writer, value)
    }

    #[inline]
    fn fmt_i32<W>(&mut self, writer: &mut W, value: i32) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.fmt_i32(writer, value)
    }

    #[inline]
    fn fmt_i64<W>(&mut self, writer: &mut W, value: i64) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.fmt_i64(writer, value)
    }

    #[inline]
    fn fmt_f32<W>(&mut self, writer: &mut W, value: f32) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.fmt_f32(writer, value)
    }

    #[inline]
    fn fmt_f64<W>(&mut self, writer: &mut W, value: f64) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.fmt_f64(writer, value)
    }

    #[inline]
    fn fmt_str<W>(&mut self, writer: &mut W, value: &str) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.fmt_str(writer, value)
    }

    #[inline]
    fn begin_seq<W>(&mut self, writer: &mut W, kind: SeqKind) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.indent_depth += 1;
        self.delegate.begin_seq(writer, kind)
    }

    #[inline]
    fn begin_element<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.newline_indent(writer)?;
        self.delegate.begin_element(writer)
    }

    #[inline]
    fn end_element<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.end_element(writer)
    }

    #[inline]
    fn end_seq<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.indent_depth -= 1;
        if self.delegate.has_value {
            self.newline_indent(writer)?;
        }
        self.delegate.end_seq(writer)
    }

    #[inline]
    fn begin_map<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.indent_depth += 1;
        self.delegate.begin_map(writer)
    }

    #[inline]
    fn begin_key<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.newline_indent(writer)?;
        self.delegate.begin_key(writer)
    }

    #[inline]
    fn fmt_key<W>(&mut self, writer: &mut W, value: &str) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.fmt_key(writer, value)
    }

    #[inline]
    fn end_key<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.end_key(writer)
    }

    #[inline]
    fn begin_value<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_str(": ")
    }

    #[inline]
    fn end_value<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.end_value(writer)
    }

    #[inline]
    fn end_map<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.indent_depth -= 1;
        if self.delegate.has_value {
            self.newline_indent(writer)?;
        }
        self.delegate.end_map(writer)
    }
}

pub struct CompactFormatter {
    normalize: bool,
    has_value: bool,
}

impl Formatter for CompactFormatter {
    #[inline]
    fn fmt_bool<W>(&mut self, writer: &mut W, value: bool) -> Result<()>
    where
        W: ?Sized + Write,
    {
        let data = if value { "true" } else { "false" };
        writer.write_str(data)
    }

    #[inline]
    fn fmt_i8<W>(&mut self, writer: &mut W, value: i8) -> Result<()>
    where
        W: ?Sized + Write,
    {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value);
        writer.write_str(data)?;
        writer.write_str("b")
    }

    #[inline]
    fn fmt_i16<W>(&mut self, writer: &mut W, value: i16) -> Result<()>
    where
        W: ?Sized + Write,
    {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value);
        writer.write_str(data)?;
        writer.write_str("s")
    }

    #[inline]
    fn fmt_i32<W>(&mut self, writer: &mut W, value: i32) -> Result<()>
    where
        W: ?Sized + Write,
    {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value);
        writer.write_str(data)
    }

    #[inline]
    fn fmt_i64<W>(&mut self, writer: &mut W, value: i64) -> Result<()>
    where
        W: ?Sized + Write,
    {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value);
        writer.write_str(data)?;
        writer.write_str("L")
    }

    #[inline]
    fn fmt_f32<W>(&mut self, writer: &mut W, value: f32) -> Result<()>
    where
        W: ?Sized + Write,
    {
        fmt_ryu(writer, value)?;
        writer.write_str("f")
    }

    #[inline]
    fn fmt_f64<W>(&mut self, writer: &mut W, value: f64) -> Result<()>
    where
        W: ?Sized + Write,
    {
        fmt_ryu(writer, value)?;
        writer.write_str("d")
    }

    #[inline]
    fn fmt_str<W>(&mut self, writer: &mut W, value: &str) -> Result<()>
    where
        W: ?Sized + Write,
    {
        let escaped = EscapeJava::new(value).to_string();

        writer.write_str("\"")?;
        writer.write_str(&escaped)?;
        writer.write_str("\"")
    }

    #[inline]
    fn begin_seq<W>(&mut self, writer: &mut W, kind: SeqKind) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.has_value = false;
        let data = match kind {
            SeqKind::List => "[",
            SeqKind::ByteArray => "[B;",
            SeqKind::IntArray => "[I;",
            SeqKind::LongArray => "[L;",
        };
        writer.write_str(data)
    }

    #[inline]
    fn begin_element<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        if self.has_value {
            writer.write_str(",")
        } else {
            Ok(())
        }
    }

    #[inline]
    fn end_element<W>(&mut self, _writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.has_value = true;
        Ok(())
    }

    #[inline]
    fn end_seq<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_str("]")
    }

    #[inline]
    fn begin_map<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.has_value = false;
        writer.write_str("{")
    }

    #[inline]
    fn begin_key<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        if self.has_value {
            writer.write_str(",")
        } else {
            Ok(())
        }
    }

    #[inline]
    fn fmt_key<W>(&mut self, writer: &mut W, value: &str) -> Result<()>
    where
        W: ?Sized + Write,
    {
        if self.normalize || needs_escaped(value) {
            self.fmt_str(writer, value)
        } else {
            writer.write_str(value)
        }
    }

    #[inline]
    fn end_key<W>(&mut self, _writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        Ok(())
    }

    #[inline]
    fn begin_value<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_str(":")
    }

    #[inline]
    fn end_value<W>(&mut self, _writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        self.has_value = true;
        Ok(())
    }

    #[inline]
    fn end_map<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_str("}")
    }
}

#[must_use]
fn needs_escaped(key: &str) -> bool {
    key.bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'+' | b'-' | b'.' | b'_'))
}

fn fmt_ryu<W, F>(writer: &mut W, value: F) -> Result<()>
where
    W: ?Sized + Write,
    F: Float,
{
    if value == F::INFINITY {
        writer.write_str("Infinity")
    } else if value == F::NEG_INFINITY {
        writer.write_str("-Infinity")
    } else if value.is_nan() {
        writer.write_str("NaN")
    } else {
        let mut buf = ryu::Buffer::new();
        let data = buf.format_finite(value);
        writer.write_str(data)
    }
}

trait Float: ryu::Float + PartialEq {
    const INFINITY: Self;
    const NEG_INFINITY: Self;

    #[must_use]
    fn is_nan(self) -> bool;
}

impl Float for f32 {
    const INFINITY: Self = f32::INFINITY;
    const NEG_INFINITY: Self = f32::NEG_INFINITY;

    #[inline]
    fn is_nan(self) -> bool {
        self.is_nan()
    }
}

impl Float for f64 {
    const INFINITY: Self = f64::INFINITY;
    const NEG_INFINITY: Self = f64::NEG_INFINITY;

    #[inline]
    fn is_nan(self) -> bool {
        self.is_nan()
    }
}

pub trait Formatter {
    ////////////////////////////////////////////////////////////////////////////
    // Primitive Methods
    ////////////////////////////////////////////////////////////////////////////

    fn fmt_bool<W>(&mut self, writer: &mut W, value: bool) -> Result<()>
    where
        W: ?Sized + Write;

    fn fmt_i8<W>(&mut self, writer: &mut W, value: i8) -> Result<()>
    where
        W: ?Sized + Write;

    fn fmt_i16<W>(&mut self, writer: &mut W, value: i16) -> Result<()>
    where
        W: ?Sized + Write;

    fn fmt_i32<W>(&mut self, writer: &mut W, value: i32) -> Result<()>
    where
        W: ?Sized + Write;

    fn fmt_i64<W>(&mut self, writer: &mut W, value: i64) -> Result<()>
    where
        W: ?Sized + Write;

    fn fmt_f32<W>(&mut self, writer: &mut W, value: f32) -> Result<()>
    where
        W: ?Sized + Write;

    fn fmt_f64<W>(&mut self, writer: &mut W, value: f64) -> Result<()>
    where
        W: ?Sized + Write;

    fn fmt_str<W>(&mut self, writer: &mut W, value: &str) -> Result<()>
    where
        W: ?Sized + Write;

    ////////////////////////////////////////////////////////////////////////////
    // Sequence Methods
    ////////////////////////////////////////////////////////////////////////////

    fn begin_seq<W>(&mut self, writer: &mut W, kind: SeqKind) -> Result<()>
    where
        W: ?Sized + Write;

    fn begin_element<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write;

    fn end_element<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write;

    fn end_seq<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write;

    ////////////////////////////////////////////////////////////////////////////
    // Map Methods
    ////////////////////////////////////////////////////////////////////////////

    fn begin_map<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write;

    fn begin_key<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write;

    fn fmt_key<W>(&mut self, writer: &mut W, value: &str) -> Result<()>
    where
        W: ?Sized + Write;

    fn end_key<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write;

    fn begin_value<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write;

    fn end_value<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write;

    fn end_map<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: ?Sized + Write;
}

pub trait Write {
    fn write_str(&mut self, data: &str) -> Result<()>;
}
