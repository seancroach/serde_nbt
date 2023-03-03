use super::mutf8;

use crate::{
    error::{Error, Position, Result, Path},
    value::Id,
};

use alloc::borrow::Cow;

use zc_io::{error, Read};
use serde::de;

struct Deserializer<R> {
    reader: R,
    path: Path,
}

impl<'de, 'a, R> de::Deserializer<'de> for &'a mut Deserializer<R>
where
    R: Read<'de>,
{
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        let value = self.reader.read
        visitor.visit_i8
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        Err(Error::)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value> where V: de::Visitor<'de> {
        todo!()
    }

    fn is_human_readable(&self) -> bool {
        todo!()
    }
}

trait Decode {
    fn decode_i16<'de, R>(&mut self, reader: &mut R) -> zc_io::Result<i16>
    where
        R: Read<'de>;

    fn decode_i32<'de, R>(&mut self, reader: &mut R) -> zc_io::Result<i32>
    where
        R: Read<'de>;

    fn decode_i64<'de, R>(&mut self, reader: &mut R) -> zc_io::Result<i64>
    where
        R: Read<'de>;

    fn decode_f32<'de, R>(&mut self, reader: &mut R) -> zc_io::Result<f32>
    where
        R: Read<'de>;

    fn decode_f64<'de, R>(&mut self, reader: &mut R) -> zc_io::Result<f64>
    where
        R: Read<'de>;

    fn decode_str_len<'de, R>(&mut self, reader: &mut R) -> zc_io::Result<usize>
    where
        R: Read<'de>;

    // #[inline]
    // fn decode_str<'de, R>(&mut self, reader: &mut R) -> zc_io::Result<Cow<'de, str>>
    // where
    //     R: Read<'de>,
    // {
    //     let len = self.decode_str_len(reader)?;
    //     let bytes = reader.read_slice(len)?;
//
    //     match bytes {
    //         Cow::Borrowed(bytes) => mutf8::decode(bytes),
    //         Cow::Owned(vec) => {
    //             let decoded = mutf8::decode(&vec)?;
    //             Ok(Cow::Owned(decoded.into_owned()))
    //         }
    //     }
    // }

    fn decode_seq_len<'de, R>(&mut self, reader: &mut R) -> zc_io::Result<usize>
    where
        R: Read<'de>;
}
