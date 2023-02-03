use crate::{
    error::{Error, Result},
    io::Writer,
    str::EscapeJava,
};

////////////////////////////////////////////////////////////////////////////////
// Formatting
////////////////////////////////////////////////////////////////////////////////

/// TODO
///
/// # Safety
///
/// TODO
#[allow(clippy::missing_errors_doc)]
pub unsafe trait Formatter {
    /// TODO
    #[inline]
    fn write_bool(&mut self, writer: &mut Writer, value: bool) -> Result<()> {
        let buf: &[u8] = if value { b"true" } else { b"false" };
        writer.write_all(buf)?;
        Ok(())
    }

    /// TODO
    #[inline]
    fn write_i8(&mut self, writer: &mut Writer, value: i8) -> Result<()> {
        let mut buf = itoa::Buffer::new();
        writer.write_all(buf.format(value).as_bytes())?;
        writer.write_all(b"b")?;
        Ok(())
    }

    /// TODO
    #[inline]
    fn write_i16(&mut self, writer: &mut Writer, value: i16) -> Result<()> {
        let mut buf = itoa::Buffer::new();
        writer.write_all(buf.format(value).as_bytes())?;
        writer.write_all(b"s")?;
        Ok(())
    }

    /// TODO
    #[inline]
    fn write_i32(&mut self, writer: &mut Writer, value: i32) -> Result<()> {
        let mut buf = itoa::Buffer::new();
        writer.write_all(buf.format(value).as_bytes())?;
        Ok(())
    }

    /// TODO
    #[inline]
    fn write_i64(&mut self, writer: &mut Writer, value: i64) -> Result<()> {
        let mut buf = itoa::Buffer::new();
        writer.write_all(buf.format(value).as_bytes())?;
        writer.write_all(b"L")?;
        Ok(())
    }

    /// TODO
    fn write_f32(&mut self, writer: &mut Writer, value: f32) -> Result<()> {
        if value.is_nan() {
            writer.write_all(b"NaNf")?;
        } else if value == f32::INFINITY {
            writer.write_all(b"Infinityf")?;
        } else if value == f32::NEG_INFINITY {
            writer.write_all(b"-Infinityf")?;
        } else {
            let mut buf = ryu::Buffer::new();
            writer.write_all(buf.format(value).as_bytes())?;
            writer.write_all(b"f")?;
        }

        Ok(())
    }

    /// TODO
    fn write_f64(&mut self, writer: &mut Writer, value: f64) -> Result<()> {
        if value.is_nan() {
            writer.write_all(b"NaNd")?;
        } else if value == f64::INFINITY {
            writer.write_all(b"Infinityd")?;
        } else if value == f64::NEG_INFINITY {
            writer.write_all(b"-Infinityd")?;
        } else {
            let mut buf = ryu::Buffer::new();
            writer.write_all(buf.format(value).as_bytes())?;
            writer.write_all(b"d")?;
        }

        Ok(())
    }

    /// TODO
    #[inline]
    fn write_string(&mut self, writer: &mut Writer, value: &str) -> Result<()> {
        let escaped = EscapeJava::new(value).to_string();

        writer.write_all(b"\"")?;
        writer.write_all(escaped.as_bytes())?;
        writer.write_all(b"\"")?;

        Ok(())
    }

    /// TODO
    #[inline]
    fn write_key(&mut self, writer: &mut Writer, value: &str) -> Result<()> {
        if valid_key(value) {
            writer.write_all(value.as_bytes())?;
        } else {
            self.write_string(writer, value)?;
        }

        Ok(())
    }

    /// TODO
    #[inline]
    fn begin_sequence(&mut self, writer: &mut Writer, kind: SequenceKind) -> Result<()> {
        let buf: &[u8] = match kind {
            SequenceKind::ByteArray => b"[B;",
            SequenceKind::List => b"[",
            SequenceKind::IntArray => b"[I;",
            SequenceKind::LongArray => b"[L;",
        };
        writer.write_all(buf)
    }

    /// TODO
    #[inline]
    fn end_sequence(&mut self, writer: &mut Writer) -> Result<()> {
        writer.write_all(b"]")
    }

    /// TODO
    #[inline]
    fn begin_sequence_value(&mut self, writer: &mut Writer, first: bool) -> Result<()> {
        if !first {
            writer.write_all(b",")?;
        }
        Ok(())
    }

    /// TODO
    #[inline]
    fn end_sequence_value(&mut self, writer: &mut Writer) -> Result<()> {
        let _ = writer;
        Ok(())
    }

    /// TODO
    #[inline]
    fn begin_map(&mut self, writer: &mut Writer) -> Result<()> {
        writer.write_all(b"{")?;
        Ok(())
    }

    /// TODO
    #[inline]
    fn end_map(&mut self, writer: &mut Writer) -> Result<()> {
        writer.write_all(b"}")?;
        Ok(())
    }

    /// TODO
    #[inline]
    fn begin_map_key(&mut self, writer: &mut Writer, first: bool) -> Result<()> {
        if !first {
            writer.write_all(b",")?;
        }
        Ok(())
    }

    /// TODO
    #[inline]
    fn end_map_key(&mut self, writer: &mut Writer) -> Result<()> {
        let _ = writer;
        Ok(())
    }

    /// TODO
    #[inline]
    fn begin_map_value(&mut self, writer: &mut Writer) -> Result<()> {
        writer.write_all(b":")?;
        Ok(())
    }

    /// TODO
    #[inline]
    fn end_map_value(&mut self, writer: &mut Writer) -> Result<()> {
        let _ = writer;
        Ok(())
    }
}

/// TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequenceKind {
    /// TODO
    ByteArray,
    /// TODO
    List,
    /// TODO
    IntArray,
    /// TODO
    LongArray,
}

/// TODO
pub struct CompactFormatter {
    normalize: bool,
}

unsafe impl Formatter for CompactFormatter {
    #[inline]
    fn write_key(&mut self, writer: &mut Writer, value: &str) -> Result<()> {
        if !self.normalize && valid_key(value) {
            writer.write_all(value.as_bytes())?;
        } else {
            self.write_string(writer, value)?;
        }

        Ok(())
    }
}

/// TODO
pub struct PrettyFormatter<'a> {
    normalize: bool,
    indent: &'a [u8],

    current_indent: usize,
    has_value: bool,
}

impl PrettyFormatter<'_> {
    fn write_newline_indent(&mut self, writer: &mut Writer) -> Result<()> {
        writer.write_all(b"\n")?;

        for _ in 0..self.current_indent {
            writer.write_all(self.indent)?;
        }

        Ok(())
    }
}

unsafe impl Formatter for PrettyFormatter<'_> {
    #[inline]
    fn write_key(&mut self, writer: &mut Writer, value: &str) -> Result<()> {
        if !self.normalize && valid_key(value) {
            writer.write_all(value.as_bytes())?;
        } else {
            self.write_string(writer, value)?;
        }

        Ok(())
    }

    #[inline]
    fn begin_sequence(&mut self, writer: &mut Writer, kind: SequenceKind) -> Result<()> {
        self.current_indent += 1;
        self.has_value = false;

        writer.write_all(b"[")?;
        Ok(())
    }

    #[inline]
    fn end_sequence(&mut self, writer: &mut Writer) -> Result<()> {
        self.current_indent -= 1;

        if self.has_value {
            self.write_newline_indent(writer)?;
        }

        writer.write_all(b"]")?;
        Ok(())
    }

    #[inline]
    fn begin_sequence_value(&mut self, writer: &mut Writer, first: bool) -> Result<()> {
        if !first {
            writer.write_all(b",")?;
        }

        self.write_newline_indent(writer)?;
        Ok(())
    }

    #[inline]
    fn end_sequence_value(&mut self, _writer: &mut Writer) -> Result<()> {
        self.has_value = true;
        Ok(())
    }

    #[inline]
    fn begin_map(&mut self, writer: &mut Writer) -> Result<()> {
        self.current_indent += 1;
        self.has_value = false;

        writer.write_all(b"{")?;
        Ok(())
    }

    #[inline]
    fn end_map(&mut self, writer: &mut Writer) -> Result<()> {
        self.current_indent -= 1;

        if self.has_value {
            self.write_newline_indent(writer)?;
        }

        writer.write_all(b"}")?;
        Ok(())
    }

    #[inline]
    fn begin_map_key(&mut self, writer: &mut Writer, first: bool) -> Result<()> {
        if !first {
            writer.write_all(b",")?;
        }

        self.write_newline_indent(writer)?;
        Ok(())
    }

    #[inline]
    fn begin_map_value(&mut self, writer: &mut Writer) -> Result<()> {
        writer.write_all(b": ")?;
        Ok(())
    }

    #[inline]
    fn end_map_value(&mut self, _writer: &mut Writer) -> Result<()> {
        self.has_value = true;
        Ok(())
    }
}

#[must_use]
fn valid_key(value: &str) -> bool {
    value
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'+' | b'-' | b'.' | b'_'))
}
