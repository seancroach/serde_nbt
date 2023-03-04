//! TODO

mod path;

pub(crate) use self::path::Path;

use alloc::borrow::Cow;
use core::{fmt, mem, result};
#[cfg(feature = "std")]
use std::{backtrace::Backtrace, io::ErrorKind};

use serde::{de, ser};

/// TODO
pub type Result<T> = result::Result<T, Error>;

/// TODO
pub struct Error {
    inner: Box<Inner>,
}

impl Error {
    /// TODO
    #[must_use]
    #[cold]
    #[inline(never)]
    #[track_caller]
    pub fn new<T>(category: Category, message: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Error::with_position(category, message, Position::None)
    }

    /// TODO
    #[must_use]
    #[cold]
    #[inline(never)]
    #[track_caller]
    pub(crate) fn with_position<T>(category: Category, message: T, position: Position) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Error {
            inner: Box::new(Inner {
                category,
                message: message.into(),
                position,
                #[cfg(feature = "std")]
                backtrace: Backtrace::capture(),
            }),
        }
    }

    /// TODO
    pub(crate) fn attach_path(mut self, path: &mut Path) -> Self {
        if matches!(self.inner.position, Position::None) {
            let path = mem::take(path);
            self.inner.position = Position::Path(path);
        }

        self
    }

    ////////////////////////////////////////////////////////////////////////////
    //
    ////////////////////////////////////////////////////////////////////////////

    #[must_use]
    #[cold]
    #[inline(never)]
    #[track_caller]
    pub(crate) fn recursion_limit_exceeded(path: &mut Path) -> Self {
        let path = mem::take(path);

        Error::with_position(
            Category::RecursionLimitExceeded,
            "recursion limit exceeded",
            Position::Path(path),
        )
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

#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
impl std::error::Error for Error {}

impl de::Error for Error {
    #[track_caller]
    #[cold]
    fn custom<T>(message: T) -> Self
    where
        T: fmt::Display,
    {
        Error::new(Category::Custom, message.to_string())
    }

    #[track_caller]
    #[cold]
    fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        let message = format!("invalid type: {unexp}, expected {exp}");
        Error::new(Category::InvalidData, message)
    }

    #[track_caller]
    #[cold]
    fn invalid_value(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        let message = format!("invalid value: {unexp}, expected {exp}");
        Error::new(Category::InvalidData, message)
    }

    #[track_caller]
    #[cold]
    fn invalid_length(len: usize, exp: &dyn de::Expected) -> Self {
        let message = format!("invalid length: {len}, expected {exp}");
        Error::new(Category::InvalidData, message)
    }

    #[track_caller]
    #[cold]
    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
        let message = if expected.is_empty() {
            format!("unknown variant `{variant}`, there are no variants")
        } else {
            format!(
                "unknown variant `{variant}`, expected {}",
                OneOf { names: expected }
            )
        };
        Error::new(Category::InvalidData, message)
    }

    #[track_caller]
    #[cold]
    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        let message = if expected.is_empty() {
            format!("unknown field `{field}`, there are no fields")
        } else {
            format!(
                "unknown field `{field}`, expected {}",
                OneOf { names: expected }
            )
        };
        Error::new(Category::InvalidData, message)
    }

    #[track_caller]
    #[cold]
    fn missing_field(field: &'static str) -> Self {
        Error::new(Category::InvalidData, format!("missing field `{field}`"))
    }

    #[track_caller]
    #[cold]
    fn duplicate_field(field: &'static str) -> Self {
        Error::new(Category::InvalidData, format!("duplicate field `{field}`"))
    }
}

struct Inner {
    category: Category,
    message: Cow<'static, str>,
    position: Position,
    #[cfg(feature = "std")]
    backtrace: Backtrace,
}

pub(crate) enum Position {
    None,
    Byte(u64),
    Cursor(Cursor),
    Path(Path),
}

pub(crate) struct Cursor {
    line: u64,
    column: u64,
}

/// TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// TODO
    RecursionLimitExceeded,
}

////////////////////////////////////////////////////////////////////////////////

/// Used in error messages.
///
/// - expected `a`
/// - expected `a` or `b`
/// - expected one of `a`, `b`, `c`
///
/// The slice of names must not be empty.
struct OneOf {
    names: &'static [&'static str],
}

impl fmt::Display for OneOf {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.names.len() {
            0 => panic!(), // special case elsewhere
            1 => write!(formatter, "`{}`", self.names[0]),
            2 => write!(formatter, "`{}` or `{}`", self.names[0], self.names[1]),
            _ => {
                write!(formatter, "one of ")?;
                for (i, alt) in self.names.iter().enumerate() {
                    if i > 0 {
                        write!(formatter, ", ")?;
                    }
                    write!(formatter, "`{}`", alt)?;
                }
                Ok(())
            }
        }
    }
}
