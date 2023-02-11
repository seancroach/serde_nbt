//! TODO

use alloc::borrow::Cow;
use core::{fmt, num::NonZeroU64, result};
#[cfg(feature = "std")]
use std::{backtrace::Backtrace, error, io::ErrorKind};

use serde::ser;

/// TODO
pub type Result<T> = result::Result<T, Error>;

/// TODO
#[repr(transparent)]
pub struct Error {
    inner: Box<Inner>,
}

impl Error {
    /// TODO
    #[must_use]
    #[cold]
    #[track_caller]
    pub fn new<T>(category: Category, message: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Error {
            inner: Box::new(Inner {
                message: message.into(),
                category,
                position: Position::None,
                #[cfg(feature = "std")]
                backtrace: Backtrace::capture(),
            }),
        }
    }

    #[cold]
    #[track_caller]
    pub(crate) fn attach_position(&mut self, position: Position) {
        debug_assert!(self.inner.position == Position::None);
        self.inner.position = position;
    }

    ////////////////////////////////////////////////////////////////////////////
    // Error constructors
    ////////////////////////////////////////////////////////////////////////////

    #[cold]
    pub(crate) fn invalid_key(display_type: &str) -> Self {
        Error::new(
            Category::InvalidInput,
            format!("`NBT does not support {display_type} keys"),
        )
    }

    #[cold]
    pub(crate) fn invalid_type(display_type: &str) -> Self {
        Error::new(
            Category::InvalidInput,
            format!("NBT does not support {display_type} values"),
        )
    }
}

impl From<zc_io::Error> for Error {
    fn from(value: zc_io::Error) -> Self {
        #[cfg(feature = "std")]
        {
            let category = value.kind().into();
            Error::new(category, value.to_string())
        }
        #[cfg(not(feature = "std"))]
        {
            Error::new(Category::Io, value.to_string())
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl error::Error for Error {}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        todo!()
    }
}

struct Inner {
    message: Cow<'static, str>,
    category: Category,
    position: Position,
    #[cfg(feature = "std")]
    backtrace: Backtrace,
}

/// TODO
#[allow(clippy::module_name_repetitions)]
pub enum Category {
    /// TODO
    Custom,
    /// TODO
    InvalidData,
    /// TODO
    InvalidInput,
    /// TODO
    Io,
    /// TODO
    UnexpectedEof,
}

#[cfg(feature = "std")]
impl From<ErrorKind> for Category {
    fn from(value: ErrorKind) -> Self {
        match value {
            ErrorKind::InvalidData => Category::InvalidData,
            ErrorKind::InvalidInput => Category::InvalidInput,
            ErrorKind::Other => Category::Custom,
            ErrorKind::UnexpectedEof => Category::UnexpectedEof,
            _ => Category::Io,
        }
    }
}

/// TODO
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Position {
    None,
    Path(Path),
    Byte(NonZeroU64),
    Cursor(Cursor),
}

/// TODO
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor {
    line: NonZeroU64,
    column: NonZeroU64,
}

impl Cursor {
    /// TODO
    #[doc(hidden)]
    #[must_use]
    pub(crate) const fn new(line: NonZeroU64, column: NonZeroU64) -> Self {
        Cursor { line, column }
    }
}

/// TODO
#[doc(hidden)]
pub type Path = Vec<PathSegment>;

/// TODO
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathSegment {
    Identifier(Cow<'static, str>),
    Index(u64),
}

impl From<&'static str> for PathSegment {
    #[inline]
    fn from(value: &'static str) -> Self {
        PathSegment::Identifier(Cow::Borrowed(value))
    }
}
