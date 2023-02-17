//! TODO

use crate::{
    error::Result,
    ser::{Emit, EmitMap, EmitSeq, Serializer},
    str::{needs_escaped, EscapeJava},
    value::Id,
    SeqKind,
};

use serde::Serialize;
use zc_io::Write;

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let mut bytes = Vec::new();
    let formatter = CompactFormatter::new(&mut bytes, false);
    let mut emitter = SnbtEmitter::new(formatter);
    let mut serializer = Serializer::new(&mut emitter);
    value.serialize(&mut serializer)?;

    // SAFETY: CompactFormatter only emits valid UTF-8 data.
    Ok(unsafe { String::from_utf8_unchecked(bytes) })
}

pub fn to_string_pretty<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let mut bytes = Vec::new();
    let formatter = PrettyFormatter::new(&mut bytes, false);
    let mut emitter = SnbtEmitter::new(formatter);
    let mut serializer = Serializer::new(&mut emitter);
    value.serialize(&mut serializer)?;

    // SAFETY: PrettyFormatter only emits valid UTF-8 data.
    Ok(unsafe { String::from_utf8_unchecked(bytes) })
}

pub struct SnbtEmitter<F: Format> {
    formatter: F,
}

impl<F: Format> SnbtEmitter<F> {
    pub fn new(formatter: F) -> Self {
        SnbtEmitter { formatter }
    }
}

impl<'a, F: Format> Emit for &'a mut SnbtEmitter<F> {
    type Output = ();

    type EmitSeq = SnbtSeqEmitter<'a, F>;
    type EmitMap = SnbtMapEmitter<'a, F>;

    fn emit_bool(&mut self, value: bool) -> zc_io::Result<Self::Output> {
        self.formatter.write_bool(value)
    }

    fn emit_i8(&mut self, value: i8) -> zc_io::Result<Self::Output> {
        self.formatter.write_i8(value)
    }

    fn emit_i16(&mut self, value: i16) -> zc_io::Result<Self::Output> {
        self.formatter.write_i16(value)
    }

    fn emit_i32(&mut self, value: i32) -> zc_io::Result<Self::Output> {
        self.formatter.write_i32(value)
    }

    fn emit_i64(&mut self, value: i64) -> zc_io::Result<Self::Output> {
        self.formatter.write_i64(value)
    }

    fn emit_f32(&mut self, value: f32) -> zc_io::Result<Self::Output> {
        self.formatter.write_f32(value)
    }

    fn emit_f64(&mut self, value: f64) -> zc_io::Result<Self::Output> {
        self.formatter.write_f64(value)
    }

    fn emit_str(&mut self, value: &str) -> zc_io::Result<Self::Output> {
        self.formatter.write_str(value)
    }

    fn emit_seq(&mut self, kind: SeqKind, _len: Option<usize>) -> zc_io::Result<Self::EmitSeq> {
        self.formatter.begin_seq(kind)?;
        Ok(SnbtSeqEmitter::new(&mut self.formatter))
    }

    fn emit_map(&mut self, _len: Option<usize>) -> zc_io::Result<Self::EmitMap> {
        self.formatter.begin_map()?;
        Ok(SnbtMapEmitter::new(&mut self.formatter))
    }

    fn is_human_readable(&self) -> bool {
        true
    }
}

pub struct SnbtSeqEmitter<'a, F: Format> {
    formatter: &'a mut F,
}

impl<'a, F: Format> SnbtSeqEmitter<'a, F> {
    fn new(formatter: &'a mut F) -> Self {
        SnbtSeqEmitter { formatter }
    }
}

impl<'a, F: Format> EmitSeq for SnbtSeqEmitter<'a, F> {
    type Output = ();

    fn begin_element(&mut self) -> zc_io::Result<()> {
        self.formatter.begin_element()
    }

    fn handle_element(&mut self, _value: Self::Output) -> zc_io::Result<()> {
        Ok(())
    }

    fn end_element(&mut self) -> zc_io::Result<()> {
        self.formatter.end_element()
    }

    fn finish(self) -> zc_io::Result<Self::Output> {
        self.formatter.end_seq()
    }
}

pub struct SnbtMapEmitter<'a, F: Format> {
    formatter: &'a mut F,
}

impl<'a, F: Format> SnbtMapEmitter<'a, F> {
    fn new(formatter: &'a mut F) -> Self {
        SnbtMapEmitter { formatter }
    }
}

impl<'a, F: Format> EmitMap for SnbtMapEmitter<'a, F> {
    type Output = ();

    fn begin_key(&mut self, _hint: Id) -> zc_io::Result<()> {
        self.formatter.begin_key()
    }

    fn emit_key(&mut self, key: &str) -> zc_io::Result<()> {
        self.formatter.write_key(key)
    }

    fn end_key(&mut self) -> zc_io::Result<()> {
        self.formatter.end_key()
    }

    fn begin_value(&mut self) -> zc_io::Result<()> {
        self.formatter.begin_value()
    }

    fn handle_value(&mut self, _value: Self::Output) -> zc_io::Result<()> {
        Ok(())
    }

    fn end_value(&mut self) -> zc_io::Result<()> {
        self.formatter.end_value()
    }

    fn finish(self) -> zc_io::Result<Self::Output> {
        self.formatter.end_map()
    }
}

pub struct PrettyFormatter<'a, W: Write> {
    delegate: CompactFormatter<W>,
    depth: usize,
    indent: &'a str,
}

impl<'a, W: Write> PrettyFormatter<'a, W> {
    pub fn new(writer: W, normalize: bool) -> Self {
        PrettyFormatter::with_indent(writer, normalize, "  ")
    }

    pub fn with_indent(writer: W, normalize: bool, indent: &'a str) -> Self {
        PrettyFormatter {
            delegate: CompactFormatter::new(writer, normalize),
            depth: 0,
            indent,
        }
    }

    fn indent(&mut self) -> zc_io::Result<()> {
        for _ in 0..self.depth {
            self.delegate.writer.write_all(self.indent.as_bytes())?;
        }
        Ok(())
    }
}

impl<'a, W: Write> Format for PrettyFormatter<'a, W> {
    fn write_bool(&mut self, value: bool) -> zc_io::Result<()> {
        self.delegate.write_bool(value)
    }

    fn write_i8(&mut self, value: i8) -> zc_io::Result<()> {
        self.delegate.write_i8(value)
    }

    fn write_i16(&mut self, value: i16) -> zc_io::Result<()> {
        self.delegate.write_i16(value)
    }

    fn write_i32(&mut self, value: i32) -> zc_io::Result<()> {
        self.delegate.write_i32(value)
    }

    fn write_i64(&mut self, value: i64) -> zc_io::Result<()> {
        self.delegate.write_i64(value)
    }

    fn write_f32(&mut self, value: f32) -> zc_io::Result<()> {
        self.delegate.write_f32(value)
    }

    fn write_f64(&mut self, value: f64) -> zc_io::Result<()> {
        self.delegate.write_f64(value)
    }

    fn write_str(&mut self, value: &str) -> zc_io::Result<()> {
        self.delegate.write_str(value)
    }

    fn begin_seq(&mut self, kind: SeqKind) -> zc_io::Result<()> {
        self.depth += 1;
        self.delegate.begin_seq(kind)
    }

    fn begin_element(&mut self) -> zc_io::Result<()> {
        self.delegate.begin_element()?;
        self.indent()
    }

    fn end_element(&mut self) -> zc_io::Result<()> {
        self.delegate.end_element()
    }

    fn end_seq(&mut self) -> zc_io::Result<()> {
        self.depth -= 1;

        if self.delegate.has_value {
            self.indent()?;
        }

        self.delegate.end_seq()
    }

    fn begin_map(&mut self) -> zc_io::Result<()> {
        self.depth += 1;
        self.delegate.begin_map()
    }

    fn begin_key(&mut self) -> zc_io::Result<()> {
        self.delegate.begin_key()?;
        self.indent()
    }

    fn write_key(&mut self, key: &str) -> zc_io::Result<()> {
        self.delegate.write_key(key)
    }

    fn end_key(&mut self) -> zc_io::Result<()> {
        self.delegate.end_key()
    }

    fn begin_value(&mut self) -> zc_io::Result<()> {
        self.delegate.writer.write_all(b": ")
    }

    fn end_value(&mut self) -> zc_io::Result<()> {
        self.delegate.end_value()
    }

    fn end_map(&mut self) -> zc_io::Result<()> {
        self.depth -= 1;

        if self.delegate.has_value {
            self.indent()?;
        }

        self.delegate.end_map()
    }
}

pub struct CompactFormatter<W: Write> {
    writer: W,
    normalize: bool,
    has_value: bool,
}

impl<W: Write> CompactFormatter<W> {
    pub fn new(writer: W, normalize: bool) -> Self {
        CompactFormatter {
            writer,
            normalize,
            has_value: false,
        }
    }
}

impl<W: Write> Format for CompactFormatter<W> {
    fn write_bool(&mut self, value: bool) -> zc_io::Result<()> {
        let data: &[u8] = if value { b"true" } else { b"false" };
        self.writer.write_all(data)
    }

    fn write_i8(&mut self, value: i8) -> zc_io::Result<()> {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value).as_bytes();
        self.writer.write_all(data)?;
        self.writer.write_all(b"b")
    }

    fn write_i16(&mut self, value: i16) -> zc_io::Result<()> {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value).as_bytes();
        self.writer.write_all(data)?;
        self.writer.write_all(b"s")
    }

    fn write_i32(&mut self, value: i32) -> zc_io::Result<()> {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value).as_bytes();
        self.writer.write_all(data)
    }

    fn write_i64(&mut self, value: i64) -> zc_io::Result<()> {
        let mut buf = itoa::Buffer::new();
        let data = buf.format(value).as_bytes();
        self.writer.write_all(data)?;
        self.writer.write_all(b"L")
    }

    fn write_f32(&mut self, value: f32) -> zc_io::Result<()> {
        write_ryu(&mut self.writer, value)?;
        self.writer.write_all(b"f")
    }

    fn write_f64(&mut self, value: f64) -> zc_io::Result<()> {
        write_ryu(&mut self.writer, value)?;
        self.writer.write_all(b"d")
    }

    fn write_str(&mut self, value: &str) -> zc_io::Result<()> {
        let escaped = EscapeJava::new(value).to_string();
        self.writer.write_all(b"\"")?;
        self.writer.write_all(escaped.as_bytes())?;
        self.writer.write_all(b"\"")
    }

    fn begin_seq(&mut self, kind: SeqKind) -> zc_io::Result<()> {
        self.has_value = false;
        let data: &[u8] = match kind {
            SeqKind::ByteArray => b"[B;",
            SeqKind::IntArray => b"[I;",
            SeqKind::LongArray => b"[L;",
            SeqKind::List(_) => b"[",
        };
        self.writer.write_all(data)
    }

    fn begin_element(&mut self) -> zc_io::Result<()> {
        if self.has_value {
            self.writer.write_all(b",")
        } else {
            Ok(())
        }
    }

    fn end_element(&mut self) -> zc_io::Result<()> {
        self.has_value = true;
        Ok(())
    }

    fn end_seq(&mut self) -> zc_io::Result<()> {
        self.writer.write_all(b"]")
    }

    fn begin_map(&mut self) -> zc_io::Result<()> {
        self.has_value = false;
        self.writer.write_all(b"{")
    }

    fn begin_key(&mut self) -> zc_io::Result<()> {
        if self.has_value {
            self.writer.write_all(b",")
        } else {
            Ok(())
        }
    }

    fn write_key(&mut self, key: &str) -> zc_io::Result<()> {
        if self.normalize || needs_escaped(key) {
            self.write_str(key)
        } else {
            self.writer.write_all(key.as_bytes())
        }
    }

    fn end_key(&mut self) -> zc_io::Result<()> {
        Ok(())
    }

    fn begin_value(&mut self) -> zc_io::Result<()> {
        self.writer.write_all(b":")
    }

    fn end_value(&mut self) -> zc_io::Result<()> {
        self.has_value = true;
        Ok(())
    }

    fn end_map(&mut self) -> zc_io::Result<()> {
        self.writer.write_all(b"}")
    }
}

fn write_ryu<W, F>(writer: &mut W, value: F) -> zc_io::Result<()>
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
        let data = buf.format_finite(value).as_bytes();
        writer.write_all(data)
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

    fn is_nan(self) -> bool {
        self.is_nan()
    }
}

impl Float for f64 {
    const INFINITY: Self = f64::INFINITY;
    const NEG_INFINITY: Self = f64::NEG_INFINITY;

    fn is_nan(self) -> bool {
        self.is_nan()
    }
}

pub trait Format {
    fn write_bool(&mut self, value: bool) -> zc_io::Result<()>;

    fn write_i8(&mut self, value: i8) -> zc_io::Result<()>;

    fn write_i16(&mut self, value: i16) -> zc_io::Result<()>;

    fn write_i32(&mut self, value: i32) -> zc_io::Result<()>;

    fn write_i64(&mut self, value: i64) -> zc_io::Result<()>;

    fn write_f32(&mut self, value: f32) -> zc_io::Result<()>;

    fn write_f64(&mut self, value: f64) -> zc_io::Result<()>;

    fn write_str(&mut self, value: &str) -> zc_io::Result<()>;

    fn begin_seq(&mut self, kind: SeqKind) -> zc_io::Result<()>;

    fn begin_element(&mut self) -> zc_io::Result<()>;

    fn end_element(&mut self) -> zc_io::Result<()>;

    fn end_seq(&mut self) -> zc_io::Result<()>;

    fn begin_map(&mut self) -> zc_io::Result<()>;

    fn begin_key(&mut self) -> zc_io::Result<()>;

    fn write_key(&mut self, key: &str) -> zc_io::Result<()>;

    fn end_key(&mut self) -> zc_io::Result<()>;

    fn begin_value(&mut self) -> zc_io::Result<()>;

    fn end_value(&mut self) -> zc_io::Result<()>;

    fn end_map(&mut self) -> zc_io::Result<()>;
}
