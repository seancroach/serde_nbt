//! TODO

use crate::{util::needs_escaped, value::Id};

use alloc::borrow::Cow;
use core::{fmt, mem, num::NonZeroU64, result};
#[cfg(feature = "std")]
use std::{backtrace::Backtrace, error, io::ErrorKind};

use serde::{de, ser};

/// TODO
pub type Result<T> = result::Result<T, Error>;

pub(crate) trait ZcResultExt<T> {
    fn attach_path(self, path: &mut Path) -> Result<T>;
}

impl<T> ZcResultExt<T> for zc_io::Result<T> {
    #[inline]
    fn attach_path(self, path: &mut Path) -> Result<T> {
        self.map_err(|error| {
            let path = mem::take(path);
            Error::io(&error, Position::Path(path))
        })
    }
}

/// TODO
#[repr(transparent)]
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

    #[cold]
    #[inline(never)]
    #[track_caller]
    pub(crate) fn with_position<M, P>(category: Category, message: M, position: P) -> Self
    where
        M: Into<Cow<'static, str>>,
        P: Into<Position>,
    {
        Error {
            inner: Box::new(Inner {
                message: message.into(),
                category,
                position: position.into(),
                #[cfg(feature = "std")]
                backtrace: Backtrace::capture(),
            }),
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Error constructors
    ////////////////////////////////////////////////////////////////////////////

    #[cold]
    #[inline(never)]
    pub(crate) fn io(error: &zc_io::Error, position: Position) -> Self {
        #[cfg(feature = "std")]
        {
            let category = error.kind().into();
            Error::with_position(category, error.to_string(), position)
        }
        #[cfg(not(feature = "std"))]
        {
            Error::with_position(Category::Io, value.to_string(), position)
        }
    }

    #[cold]
    #[inline(never)]
    pub(crate) fn invalid_key(display_type: &str, path: &mut Path) -> Self {
        Error::with_position(
            Category::InvalidInput,
            format!("`NBT does not support {display_type} keys"),
            Position::Path(mem::take(path)),
        )
    }

    #[cold]
    #[inline(never)]
    pub(crate) fn invalid_type(display_type: &str, path: &mut Path) -> Self {
        Error::with_position(
            Category::InvalidInput,
            format!("NBT does not support {display_type} values"),
            Position::Path(mem::take(path)),
        )
    }

    #[cold]
    #[inline(never)]
    pub(crate) fn invalid_root(display_type: &str, path: &mut Path) -> Self {
        Error::with_position(
            Category::InvalidInput,
            format!("NBT does not support {display_type} root values"),
            Position::Path(mem::take(path)),
        )
    }

    #[cold]
    #[inline(never)]
    pub(crate) fn invalid_seq(actual: Id, expected: Id, path: &mut Path) -> Self {
        Error::with_position(
            Category::InvalidInput,
            format!("NBT does not support mixed sequences (got {actual}, expected {expected})"),
            Position::Path(mem::take(path)),
        )
    }

    #[cold]
    #[inline(never)]
    pub(crate) fn invalid_seq_hint(actual: usize, expected: usize, path: &mut Path) -> Self {
        Error::with_position(
            Category::InvalidInput,
            format!("invalid sequence length hint (got {actual}, expected {expected})"),
            Position::Path(mem::take(path)),
        )
    }

    #[cold]
    #[inline(never)]
    pub(crate) fn invalid_seq_len(actual: usize, expected: usize, path: &mut Path) -> Self {
        Error::with_position(
            Category::InvalidInput,
            format!("oversized sequence length (got {actual}, expected {expected})"),
            Position::Path(mem::take(path)),
        )
    }

    #[cold]
    #[inline(never)]
    pub(crate) fn seq_len_overflow(max: usize, path: &mut Path) -> Self {
        Error::with_position(
            Category::InvalidInput,
            format!("selected specification cannot serialize a sequence with greater than `{max}` elements"),
            Position::Path(mem::take(path)),
        )
    }

    #[cold]
    #[inline(never)]
    pub(crate) fn str_len_overflow(max: usize, path: &mut Path) -> Self {
        Error::with_position(
            Category::InvalidInput,
            format!(
                "selected specification cannot serialize a string with greater than `{max}` bytes"
            ),
            Position::Path(mem::take(path)),
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
        self.inner.fmt(f)
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

impl de::Error for Error {
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

impl fmt::Display for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {:?}{}", self.message, self.position)
    }
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Position {
    None,
    Path(Path),
    Byte(NonZeroU64),
    Cursor(Cursor),
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Position::None => Ok(()),
            Position::Path(path) => write!(f, " at path `{path}`"),
            Position::Byte(_) => todo!(),
            Position::Cursor(_) => todo!(),
        }
    }
}

impl From<Path> for Position {
    fn from(value: Path) -> Self {
        Position::Path(value)
    }
}

/// TODO
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor {
    line: NonZeroU64,
    column: NonZeroU64,
}

impl Cursor {
    /// TODO
    #[must_use]
    pub const fn new(line: NonZeroU64, column: NonZeroU64) -> Self {
        Cursor { line, column }
    }
}

/// TODO
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Path {
    inner: Vec<PathSegment>,
}

impl Path {
    pub(crate) fn new() -> Self {
        Path { inner: Vec::new() }
    }

    pub(crate) fn enter_unresolved(&mut self) {
        self.inner.push(PathSegment::Unresolved);
    }

    pub(crate) fn leave_unresolved(&mut self) {
        let segment = self.inner.pop().unwrap();
        debug_assert_eq!(segment, PathSegment::Unresolved);
    }

    pub(crate) fn enter_scope(&mut self, scope: Cow<'static, str>) {
        self.inner.push(PathSegment::Identifier(scope));
    }

    pub(crate) fn leave_scope(&mut self) -> Cow<'static, str> {
        self.inner.pop().unwrap().unwrap_identifier()
    }

    pub(crate) fn enter_element(&mut self, index: usize) {
        self.inner.push(PathSegment::Index(index));
    }

    pub(crate) fn leave_element(&mut self) -> usize {
        self.inner.pop().unwrap().unwrap_index()
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.inner.is_empty() {
            return f.write_str("{}");
        }

        for (index, segment) in self.inner.iter().enumerate() {
            match segment {
                PathSegment::Unresolved => write!(f, "[????]")?,
                PathSegment::Identifier(key) => {
                    if index != 0 {
                        f.write_str(".")?;
                    }

                    if needs_escaped(key) {
                        write!(f, "{key:?}")?;
                    } else {
                        write!(f, "{key}")?;
                    }
                }
                PathSegment::Index(index) => write!(f, "[{index}]")?,
            }
        }

        Ok(())
    }
}

/// TODO
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PathSegment {
    Unresolved,
    Identifier(Cow<'static, str>),
    Index(usize),
}

impl PathSegment {
    pub(crate) fn unwrap_identifier(self) -> Cow<'static, str> {
        if let PathSegment::Identifier(identifier) = self {
            identifier
        } else {
            unreachable!()
        }
    }

    pub(crate) fn unwrap_index(self) -> usize {
        if let PathSegment::Index(index) = self {
            index
        } else {
            unreachable!()
        }
    }
}

impl fmt::Display for PathSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathSegment::Unresolved => write!(f, "[????]"),
            PathSegment::Identifier(key) => {
                if needs_escaped(key) {
                    write!(f, "{key:?}")
                } else {
                    write!(f, "{key}")
                }
            }
            PathSegment::Index(index) => write!(f, "[{index}]"),
        }
    }
}

impl From<&'static str> for PathSegment {
    #[inline]
    fn from(value: &'static str) -> Self {
        PathSegment::Identifier(Cow::Borrowed(value))
    }
}

impl From<usize> for PathSegment {
    #[inline]
    fn from(value: usize) -> Self {
        PathSegment::Index(value)
    }
}
