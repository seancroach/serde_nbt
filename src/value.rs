//! TODO

use alloc::borrow::Cow;
use core::ops::Deref;

/// TODO
pub struct TagString {
    inner: Cow<'static, str>,
    quotes: QuoteKind,
}

impl Deref for TagString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteKind {
    /// TODO
    None,
    /// TODO
    Single,
    /// TODO
    Double,
}
