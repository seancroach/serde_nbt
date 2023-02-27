//! TODO

use crate::char;

use core::{
    fmt::{self, Formatter, Write},
    iter::FusedIterator,
    str::Chars,
};

pub(crate) fn needs_escaped(s: &str) -> bool {
    !s.bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'+' | b'-' | b'.' | b'_'))
}

/// TODO
#[must_use]
#[derive(Debug, Clone)]
pub struct EscapeJava<'a> {
    chars: Chars<'a>,
    iter: Option<char::EscapeJava>,
}

impl<'a> EscapeJava<'a> {
    /// TODO
    #[inline]
    pub fn new(s: &'a str) -> Self {
        EscapeJava {
            chars: s.chars(),
            iter: None,
        }
    }
}

impl<'a> Iterator for EscapeJava<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.as_mut().and_then(Iterator::next).or_else(|| {
            let mut next = None;
            if let Some(c) = self.chars.next() {
                let mut iter = char::EscapeJava::new(c);
                next = iter.next();
                self.iter = Some(iter);
            }
            next
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (min_chars, Some(max_chars)) = self.chars.size_hint() else { unreachable!() };
        (min_chars, max_chars.checked_mul(6))
    }
}

impl<'a> FusedIterator for EscapeJava<'a> {}

impl fmt::Display for EscapeJava<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for c in self.clone() {
            f.write_char(c)?;
        }
        Ok(())
    }
}

/// TODO
#[must_use]
#[derive(Debug, Clone)]
pub struct EscapeJavaUnicode<'a> {
    // The intention was to create a named function like
    // core::str::EscapeUnicode does, but implementing `Fn` manually is unstable
    // so this is the alternative.
    chars: Chars<'a>,
    iter: Option<char::EscapeJavaUnicode>,
}

impl<'a> EscapeJavaUnicode<'a> {
    /// TODO
    #[inline]
    pub fn new(s: &'a str) -> Self {
        EscapeJavaUnicode {
            chars: s.chars(),
            iter: None,
        }
    }
}

impl<'a> Iterator for EscapeJavaUnicode<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.as_mut().and_then(Iterator::next).or_else(|| {
            let mut next = None;
            if let Some(c) = self.chars.next() {
                let mut iter = char::EscapeJavaUnicode::new(c);
                next = iter.next();
                self.iter = Some(iter);
            }
            next
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (min_chars, Some(max_chars)) = self.chars.size_hint() else { unreachable!() };
        (min_chars.saturating_mul(6), max_chars.checked_mul(6))
    }
}

impl<'a> FusedIterator for EscapeJavaUnicode<'a> {}

impl fmt::Display for EscapeJavaUnicode<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for c in self.clone() {
            f.write_char(c)?;
        }
        Ok(())
    }
}
