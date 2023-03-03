use crate::{
    error::Result,
    ser::{Emit, Serializer},
    str::EscapeJava,
    util::{needs_escaped, SeqKind},
    value::{Id, Kind},
};

#[cfg(feature = "std")]
use std::io;

use serde::Serialize;
#[cfg(feature = "std")]
use zc_io::IoWriter;
use zc_io::Write;

////////////////////////////////////////////////////////////////////////////////

/// TODO
///
/// # Errors
///
/// TODO
#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(all(feature = "snbt", feature = "std"))))]
pub fn to_snbt_writer_pretty<W, T>(writer: &mut W, value: &T) -> Result<()>
where
    W: ?Sized + io::Write,
    T: ?Sized + Serialize,
{
    let mut writer = IoWriter::new(writer);
    let emitter = TextEmitter::new(PrettyFormatter::new(false));
    let mut serializer = Serializer::new(emitter, &mut writer);
    value.serialize(&mut serializer)
}

/// TODO
///
/// # Errors
///
/// TODO
#[cfg_attr(doc_cfg, doc(cfg(feature = "snbt")))]
pub fn to_string_pretty<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let mut vec = Vec::new();
    let emitter = TextEmitter::new(PrettyFormatter::new(false));
    let mut serializer = Serializer::new(emitter, &mut vec);
    value.serialize(&mut serializer)?;

    // SAFETY: PrettyFormatter never emits invalid UTF-8 data.
    Ok(unsafe { String::from_utf8_unchecked(vec) })
}

/// TODO
///
/// # Errors
///
/// TODO
#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(all(feature = "snbt", feature = "std"))))]
pub fn to_snbt_writer<W, T>(writer: &mut W, value: &T) -> Result<()>
where
    W: ?Sized + io::Write,
    T: ?Sized + Serialize,
{
    let mut writer = IoWriter::new(writer);
    let emitter = TextEmitter::new(CompactFormatter::new(false));
    let mut serializer = Serializer::new(emitter, &mut writer);
    value.serialize(&mut serializer)
}

/// TODO
///
/// # Errors
///
/// TODO
#[cfg_attr(doc_cfg, doc(cfg(feature = "snbt")))]
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let mut vec = Vec::new();
    let emitter = TextEmitter::new(CompactFormatter::new(false));
    let mut serializer = Serializer::new(emitter, &mut vec);
    value.serialize(&mut serializer)?;

    // SAFETY: CompactFormatter never emits invalid UTF-8 data.
    Ok(unsafe { String::from_utf8_unchecked(vec) })
}

////////////////////////////////////////////////////////////////////////////////

#[cfg_attr(doc_cfg, doc(cfg(feature = "snbt")))]
pub struct TextEmitter<F: Format> {
    formatter: F,
}

impl<F: Format> TextEmitter<F> {
    #[must_use]
    #[inline]
    pub fn new(formatter: F) -> Self {
        TextEmitter { formatter }
    }
}

impl<F: Format> Emit for TextEmitter<F> {
    #[inline]
    fn emit_header_id<W>(&mut self, _writer: &mut W, _id: Id) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        Ok(())
    }

    #[inline]
    fn emit_bool<W>(&mut self, writer: &mut W, value: bool) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_bool(writer, value)
    }

    #[inline]
    fn emit_i8<W>(&mut self, writer: &mut W, value: i8) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_i8(writer, value)
    }

    #[inline]
    fn emit_i16<W>(&mut self, writer: &mut W, value: i16) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_i16(writer, value)
    }

    #[inline]
    fn emit_i32<W>(&mut self, writer: &mut W, value: i32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_i32(writer, value)
    }

    #[inline]
    fn emit_i64<W>(&mut self, writer: &mut W, value: i64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_i64(writer, value)
    }

    #[inline]
    fn emit_f32<W>(&mut self, writer: &mut W, value: f32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_f32(writer, value)
    }

    #[inline]
    fn emit_f64<W>(&mut self, writer: &mut W, value: f64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_f64(writer, value)
    }

    #[inline]
    fn emit_str<W>(&mut self, writer: &mut W, value: &str) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_str(writer, value)
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
        self.formatter.begin_seq(writer, kind)
    }

    #[inline]
    fn before_element<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.before_element(writer)
    }

    #[inline]
    fn after_element<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.after_element(writer)
    }

    #[inline]
    fn end_seq<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.end_seq(writer)
    }

    #[inline]
    fn begin_map<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.begin_map(writer)
    }

    #[inline]
    fn before_key<W>(&mut self, writer: &mut W, _hint: Kind) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.before_key(writer)
    }

    #[inline]
    fn emit_key<W>(&mut self, writer: &mut W, key: &str) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.write_key(writer, key)
    }

    #[inline]
    fn after_key<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.after_key(writer)
    }

    #[inline]
    fn before_value<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.before_value(writer)
    }

    #[inline]
    fn after_value<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.after_value(writer)
    }

    #[inline]
    fn end_map<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.formatter.end_map(writer)
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        true
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg_attr(doc_cfg, doc(cfg(feature = "snbt")))]
pub struct PrettyFormatter<'indent> {
    delegate: CompactFormatter,
    indent: &'indent str,
    depth: usize,
}

impl<'indent> PrettyFormatter<'indent> {
    #[must_use]
    #[inline]
    pub fn new(normalize: bool) -> Self {
        PrettyFormatter::with_indent(normalize, "  ")
    }

    #[must_use]
    #[inline]
    pub fn with_indent(normalize: bool, indent: &'indent str) -> Self {
        PrettyFormatter {
            delegate: CompactFormatter::new(normalize),
            indent,
            depth: 0,
        }
    }

    #[inline]
    fn write_newline_indent<W>(&self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"\n")?;
        for _ in 0..self.depth {
            writer.write_all(self.indent.as_bytes())?;
        }

        Ok(())
    }
}

impl<'indent> Format for PrettyFormatter<'indent> {
    #[inline]
    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.write_bool(writer, value)
    }

    #[inline]
    fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.write_i8(writer, value)
    }

    #[inline]
    fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.write_i16(writer, value)
    }

    #[inline]
    fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.write_i32(writer, value)
    }

    #[inline]
    fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.write_i64(writer, value)
    }

    #[inline]
    fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.write_f32(writer, value)
    }

    #[inline]
    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.write_f64(writer, value)
    }

    #[inline]
    fn write_str<W>(&mut self, writer: &mut W, value: &str) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.write_str(writer, value)
    }

    #[inline]
    fn begin_seq<W>(&mut self, writer: &mut W, kind: SeqKind) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.depth += 1;
        self.delegate.begin_seq(writer, kind)
    }

    #[inline]
    fn before_element<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.before_element(writer)?;
        self.write_newline_indent(writer)
    }

    #[inline]
    fn after_element<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.after_element(writer)
    }

    #[inline]
    fn end_seq<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.depth -= 1;

        if self.delegate.has_value {
            self.write_newline_indent(writer)?;
        }

        self.delegate.end_seq(writer)
    }

    #[inline]
    fn begin_map<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.depth += 1;
        self.delegate.begin_map(writer)
    }

    #[inline]
    fn before_key<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.before_key(writer)?;
        self.write_newline_indent(writer)
    }

    #[inline]
    fn write_key<W>(&mut self, writer: &mut W, key: &str) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.write_key(writer, key)
    }

    #[inline]
    fn after_key<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b": ")
    }

    #[inline]
    fn before_value<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.before_value(writer)
    }

    #[inline]
    fn after_value<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.delegate.after_value(writer)
    }

    #[inline]
    fn end_map<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.depth -= 1;

        if self.delegate.has_value {
            self.write_newline_indent(writer)?;
        }

        self.delegate.end_map(writer)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg_attr(doc_cfg, doc(cfg(feature = "snbt")))]
pub struct CompactFormatter {
    normalize: bool,
    has_value: bool,
}

impl CompactFormatter {
    #[must_use]
    #[inline]
    pub fn new(normalize: bool) -> Self {
        CompactFormatter {
            normalize,
            has_value: false,
        }
    }
}

impl Format for CompactFormatter {
    #[inline]
    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        let data: &[u8] = if value { b"true" } else { b"false" };
        writer.write_all(data)
    }

    #[inline]
    fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        write_integer(writer, value)?;
        writer.write_all(b"b")
    }

    #[inline]
    fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        write_integer(writer, value)?;
        writer.write_all(b"s")
    }

    #[inline]
    fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        write_integer(writer, value)
    }

    #[inline]
    fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        write_integer(writer, value)?;
        writer.write_all(b"L")
    }

    #[inline]
    fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        write_float(writer, value)?;
        writer.write_all(b"f")
    }

    #[inline]
    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        write_float(writer, value)?;
        writer.write_all(b"d")
    }

    #[inline]
    fn write_str<W>(&mut self, writer: &mut W, value: &str) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        if needs_escaped(value) {
            write_quoted_escaped(writer, value)
        } else {
            write_quoted(writer, value)
        }
    }

    #[inline]
    fn begin_seq<W>(&mut self, writer: &mut W, kind: SeqKind) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.has_value = false;
        match kind {
            SeqKind::ByteArray => writer.write_all(b"[B;"),
            SeqKind::IntArray => writer.write_all(b"[I;"),
            SeqKind::LongArray => writer.write_all(b"[L;"),
            SeqKind::List(_) => writer.write_all(b"["),
        }
    }

    #[inline]
    fn before_element<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        if self.has_value {
            writer.write_all(b",")
        } else {
            Ok(())
        }
    }

    #[inline]
    fn after_element<W>(&mut self, _writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.has_value = true;
        Ok(())
    }

    #[inline]
    fn end_seq<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"]")
    }

    #[inline]
    fn begin_map<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.has_value = false;
        writer.write_all(b"{")
    }

    #[inline]
    fn before_key<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        if self.has_value {
            writer.write_all(b",")
        } else {
            Ok(())
        }
    }

    #[inline]
    fn write_key<W>(&mut self, writer: &mut W, key: &str) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        if needs_escaped(key) {
            write_quoted_escaped(writer, key)
        } else if self.normalize {
            write_quoted(writer, key)
        } else {
            writer.write_all(key.as_bytes())
        }
    }

    #[inline]
    fn after_key<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b":")
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
        self.has_value = true;
        Ok(())
    }

    #[inline]
    fn end_map<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(b"}")
    }
}

////////////////////////////////////////////////////////////////////////////////

#[inline]
fn write_quoted_escaped<W>(writer: &mut W, value: &str) -> zc_io::Result<()>
where
    W: ?Sized + Write,
{
    let escaped = EscapeJava::new(value);
    let data = format!("\"{escaped}\"");
    writer.write_all(data.as_bytes())
}

#[inline]
fn write_quoted<W>(writer: &mut W, value: &str) -> zc_io::Result<()>
where
    W: ?Sized + Write,
{
    writer.write_all(b"\"")?;
    writer.write_all(value.as_bytes())?;
    writer.write_all(b"\"")
}

#[inline]
fn write_float<W, F>(writer: &mut W, value: F) -> zc_io::Result<()>
where
    W: ?Sized + Write,
    F: Float,
{
    if value == F::INFINITY {
        writer.write_all(b"Infinity")
    } else if value == F::NEG_INFINITY {
        writer.write_all(b"-Infinity")
    } else if value.is_nan() {
        writer.write_all(b"NaN")
    } else {
        let mut buf = ryu::Buffer::new();
        let data = buf.format_finite(value);
        writer.write_all(data.as_bytes())
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

#[inline]
fn write_integer<W, I>(writer: &mut W, value: I) -> zc_io::Result<()>
where
    W: ?Sized + Write,
    I: itoa::Integer,
{
    let mut buf = itoa::Buffer::new();
    let data = buf.format(value);
    writer.write_all(data.as_bytes())
}

////////////////////////////////////////////////////////////////////////////////

#[cfg_attr(doc_cfg, doc(cfg(feature = "snbt")))]
#[allow(clippy::missing_errors_doc)]
pub trait Format {
    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn write_str<W>(&mut self, writer: &mut W, value: &str) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn begin_seq<W>(&mut self, writer: &mut W, kind: SeqKind) -> zc_io::Result<()>
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

    fn before_key<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn write_key<W>(&mut self, writer: &mut W, key: &str) -> zc_io::Result<()>
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
}
