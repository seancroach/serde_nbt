/// TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Byte {
    /// TODO
    Boolean(bool),
    /// TODO
    Integer(i8),
}

impl Byte {
    /// TODO
    #[must_use]
    #[inline]
    pub const fn to_bool(self) -> bool {
        match self {
            Byte::Boolean(b) => b,
            Byte::Integer(n) => n != 0,
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn to_i8(self) -> i8 {
        match self {
            Byte::Boolean(false) => 0,
            Byte::Boolean(true) => 1,
            Byte::Integer(n) => n,
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn get_bool(self) -> Option<bool> {
        match self {
            Byte::Boolean(b) => Some(b),
            Byte::Integer(_) => None,
        }
    }

    /// TODO
    #[must_use]
    #[inline]
    pub const fn get_i8(self) -> Option<i8> {
        match self {
            Byte::Boolean(_) => None,
            Byte::Integer(n) => Some(n),
        }
    }
}

impl Default for Byte {
    /// TODO
    #[inline]
    fn default() -> Self {
        Byte::Integer(0)
    }
}

impl From<bool> for Byte {
    #[inline]
    fn from(value: bool) -> Self {
        Byte::Boolean(value)
    }
}

impl From<i8> for Byte {
    #[inline]
    fn from(value: i8) -> Self {
        Byte::Integer(value)
    }
}
