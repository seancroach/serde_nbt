//! TODO

use core::{
    fmt::{self, Formatter, Write},
    iter::FusedIterator,
};

/// TODO
#[must_use]
#[derive(Debug, Clone)]
pub struct EscapeJava {
    state: EscapeJavaState,
}

impl EscapeJava {
    /// TODO
    #[inline]
    pub fn new(c: char) -> Self {
        let init = match c {
            '\x08' => EscapeJavaState::Backslash('b'),
            '\x09' => EscapeJavaState::Backslash('t'),
            '\x0A' => EscapeJavaState::Backslash('n'),
            '\x0C' => EscapeJavaState::Backslash('f'),
            '\x0D' => EscapeJavaState::Backslash('r'),
            '\x22' => EscapeJavaState::Backslash('"'),
            '\x20'..='\x7E' => EscapeJavaState::Char(c),
            _ => EscapeJavaState::Unicode(EscapeJavaUnicode::new(c)),
        };
        EscapeJava { state: init }
    }
}

impl Iterator for EscapeJava {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            EscapeJavaState::Backslash(c) => {
                self.state = EscapeJavaState::Char(c);
                Some('\\')
            }
            EscapeJavaState::Char(c) => {
                self.state = EscapeJavaState::Done;
                Some(c)
            }
            EscapeJavaState::Unicode(ref mut iter) => iter.next(),
            EscapeJavaState::Done => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.len();
        (n, Some(n))
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        match self.state {
            EscapeJavaState::Backslash(c) | EscapeJavaState::Char(c) => Some(c),
            EscapeJavaState::Unicode(iter) => iter.last(),
            EscapeJavaState::Done => None,
        }
    }
}

impl ExactSizeIterator for EscapeJava {
    fn len(&self) -> usize {
        match self.state {
            EscapeJavaState::Backslash(_) => 2,
            EscapeJavaState::Char(_) => 1,
            EscapeJavaState::Unicode(ref iter) => iter.len(),
            EscapeJavaState::Done => 0,
        }
    }
}

impl FusedIterator for EscapeJava {}

impl fmt::Display for EscapeJava {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for c in self.clone() {
            f.write_char(c)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum EscapeJavaState {
    Backslash(char),
    Char(char),
    Unicode(EscapeJavaUnicode),
    Done,
}

/// TODO
#[derive(Debug, Clone)]
#[must_use]
pub struct EscapeJavaUnicode {
    c: char,
    state: EscapeUnicodeState,
    hex_digit_index: usize,
}

impl EscapeJavaUnicode {
    /// TODO
    #[inline]
    pub fn new(c: char) -> Self {
        EscapeJavaUnicode {
            c,
            state: EscapeUnicodeState::Backslash,
            hex_digit_index: 3,
        }
    }
}

fn get_hex_digit(c: char, index: usize) -> char {
    debug_assert!(index <= 3);
    let hex_digit = ((c as u32) >> (index * 4)) & 0x0F;
    char::from_digit(hex_digit, 16).unwrap()
}

impl Iterator for EscapeJavaUnicode {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            EscapeUnicodeState::Backslash => {
                self.state = EscapeUnicodeState::Type;
                Some('\\')
            }
            EscapeUnicodeState::Type => {
                self.state = EscapeUnicodeState::Value;
                Some('u')
            }
            EscapeUnicodeState::Value => {
                let c = get_hex_digit(self.c, self.hex_digit_index);
                if self.hex_digit_index == 0 {
                    self.state = EscapeUnicodeState::Done;
                } else {
                    self.hex_digit_index -= 1;
                }
                Some(c)
            }
            EscapeUnicodeState::Done => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.len();
        (n, Some(n))
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        match self.state {
            EscapeUnicodeState::Done => None,
            _ => Some(get_hex_digit(self.c, 0)),
        }
    }
}

impl ExactSizeIterator for EscapeJavaUnicode {
    #[inline]
    fn len(&self) -> usize {
        self.hex_digit_index
            + match self.state {
                EscapeUnicodeState::Backslash => 2,
                EscapeUnicodeState::Type => 1,
                _ => 0,
            }
    }
}

impl FusedIterator for EscapeJavaUnicode {}

impl fmt::Display for EscapeJavaUnicode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for c in self.clone() {
            f.write_char(c)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum EscapeUnicodeState {
    Backslash,
    Type,
    Value,
    Done,
}
