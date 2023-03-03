//! TODO

#[cfg(feature = "emit")]
mod emit;

#[cfg(feature = "emit")]
pub use self::emit::{Emit, MapSerializer, RootSerializer, SeqSerializer, Serializer};

use crate::{
    error::{Error, Path, Result},
    util::ArrayBrand,
};

#[cfg(all(feature = "be", feature = "std"))]
pub use crate::binary::ser::to_be_writer;
#[cfg(all(feature = "le", feature = "std"))]
pub use crate::binary::ser::to_le_writer;
#[cfg(all(feature = "varint", feature = "std"))]
pub use crate::binary::ser::to_varint_writer;
#[cfg(feature = "be")]
pub use crate::binary::ser::{to_be_vec, BeEncoder};
#[cfg(feature = "le")]
pub use crate::binary::ser::{to_le_vec, LeEncoder};
#[cfg(feature = "varint")]
pub use crate::binary::ser::{to_varint_vec, VarintEncoder};
#[cfg(feature = "binary")]
pub use crate::binary::ser::{BinaryEmitter, Encode};
#[cfg(all(feature = "snbt", feature = "std"))]
pub use crate::snbt::ser::{to_snbt_writer, to_snbt_writer_pretty};
#[cfg(feature = "snbt")]
pub use crate::snbt::ser::{
    to_string, to_string_pretty, CompactFormatter, Format, PrettyFormatter, TextEmitter,
};
pub use crate::value::ser::{to_value, ValueSerializer};

use alloc::borrow::Cow;
use core::{fmt::Display, str};

use serde::{ser, Serialize};

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn collect_key<T>(
    key: &T,
    path: &mut Path,
    is_human_readable: bool,
) -> Result<Cow<'static, str>>
where
    T: ?Sized + Serialize,
{
    path.enter_unresolved();
    let serializer = KeySerializer::new(path, is_human_readable);
    let key = key.serialize(serializer)?;
    path.leave_unresolved();
    Ok(key)
}

struct KeySerializer<'path> {
    path: &'path mut Path,
    is_human_readable: bool,
}

impl<'path> KeySerializer<'path> {
    #[must_use]
    #[inline]
    fn new(path: &'path mut Path, is_human_readable: bool) -> Self {
        KeySerializer {
            path,
            is_human_readable,
        }
    }

    #[inline]
    fn handle_tuple<T>(self, name: &'static str) -> Result<T> {
        let display_type = match ArrayBrand::from_str(name) {
            Some(ArrayBrand::Byte) => "byte array",
            Some(ArrayBrand::Int) => "int array",
            Some(ArrayBrand::Long) => "long array",
            None => "sequence",
        };
        Err(Error::invalid_key(display_type, self.path))
    }
}

macro_rules! integer_to_key {
    {
        $($method:ident for $int:ty,)+
    } => {
        $(
            #[inline]
            fn $method(self, value: $int) -> Result<Self::Ok> {
                let mut buf = itoa::Buffer::new();
                let data = buf.format(value).to_owned();
                Ok(Cow::Owned(data))
            }
        )+
    }
}

impl<'path> ser::Serializer for KeySerializer<'path> {
    type Ok = Cow<'static, str>;
    type Error = Error;

    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
        let data = if value { "true" } else { "false" };
        Ok(Cow::Borrowed(data))
    }

    integer_to_key! {
        serialize_i8 for i8,
        serialize_i16 for i16,
        serialize_i32 for i32,
        serialize_i64 for i64,
        serialize_i128 for i128,

        serialize_u8 for u8,
        serialize_u16 for u16,
        serialize_u32 for u32,
        serialize_u64 for u64,
        serialize_u128 for u128,
    }

    fn serialize_f32(self, _value: f32) -> Result<Self::Ok> {
        Err(Error::invalid_key("`f32`", self.path))
    }

    fn serialize_f64(self, _value: f64) -> Result<Self::Ok> {
        Err(Error::invalid_key("`f64`", self.path))
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<Self::Ok> {
        let mut buf = [0; 4];
        self.serialize_str(value.encode_utf8(&mut buf))
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        let data = value.to_owned();
        Ok(Cow::Owned(data))
    }

    #[inline]
    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        if let Ok(data) = str::from_utf8(value) {
            self.serialize_str(data)
        } else {
            Err(Error::invalid_key("non-UTF8 `&[u8]`", self.path))
        }
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::invalid_key("`None`", self.path))
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::invalid_key("`()`", self.path))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(Error::invalid_key("unit struct", self.path))
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(Cow::Borrowed(variant))
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::invalid_key("newtype variant", self.path))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::invalid_key("sequence", self.path))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::invalid_key("tuple", self.path))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.handle_tuple(name)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.handle_tuple(name)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::invalid_key("map", self.path))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::invalid_key("struct", self.path))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::invalid_key("struct variant", self.path))
    }

    fn collect_seq<I>(self, _iter: I) -> Result<Self::Ok>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        Err(Error::invalid_key("sequence", self.path))
    }

    fn collect_map<K, V, I>(self, _iter: I) -> Result<Self::Ok>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        Err(Error::invalid_key("map", self.path))
    }

    #[inline]
    fn collect_str<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Display,
    {
        let data = value.to_string();
        Ok(Cow::Owned(data))
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        self.is_human_readable
    }
}
