//! TODO

use crate::error::{self, Path, Result};

use zc_io::Write;

////////////////////////////////////////////////////////////////////////////////
// Write
////////////////////////////////////////////////////////////////////////////////

/// TODO
pub struct Writer<'a> {
    buf: &'a mut (dyn Write),
    path: Path,
}

impl<'a> Writer<'a> {
    #[must_use]
    pub(crate) fn new(buf: &'a mut (dyn Write)) -> Writer<'a> {
        Writer {
            buf,
            path: Path::new(),
        }
    }

    #[inline]
    pub fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.buf.write(buf).map_err(Into::into)
    }

    #[inline]
    pub fn flush(&mut self) -> Result<()> {
        self.buf.flush().map_err(Into::into)
    }

    #[inline]
    pub fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.buf.write_all(buf).map_err(Into::into)
    }
}

impl Write for Writer<'_> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> zc_io::Result<usize> {
        self.buf.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> zc_io::Result<()> {
        self.buf.flush()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> zc_io::Result<()> {
        self.buf.write_all(buf)
    }
}
