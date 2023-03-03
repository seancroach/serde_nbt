use super::mutf8;

use crate::{
    error::Result,
    ser::{Emit, RootSerializer},
    util::SeqKind,
    value::{Id, Kind},
};

#[cfg(feature = "std")]
use std::io;

use serde::Serialize;
#[cfg(feature = "std")]
use zc_io::IoWriter;
use zc_io::Write;
#[cfg(feature = "varint")]
use zende::Zigzag;

/// TODO
///
/// # Errors
///
/// TODO
#[cfg(all(feature = "varint", feature = "std"))]
#[cfg_attr(doc_cfg, doc(cfg(all(feature = "varint", feature = "std"))))]
pub fn to_varint_writer<W, T>(header: &str, writer: &mut W, value: &T) -> Result<()>
where
    W: ?Sized + io::Write,
    T: ?Sized + Serialize,
{
    let mut writer = IoWriter::new(writer);
    let emitter = BinaryEmitter::new(VarintEncoder);
    let mut serializer = RootSerializer::new(header, false, emitter, &mut writer);
    value.serialize(&mut serializer)
}

/// TODO
///
/// # Errors
///
/// TODO
#[cfg(feature = "varint")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "varint")))]
pub fn to_varint_vec<T>(header: &str, value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut vec = Vec::new();
    let emitter = BinaryEmitter::new(VarintEncoder);
    let mut serializer = RootSerializer::new(header, true, emitter, &mut vec);
    value.serialize(&mut serializer)?;
    Ok(vec)
}

/// TODO
///
/// # Errors
///
/// TODO
#[cfg(all(feature = "le", feature = "std"))]
#[cfg_attr(doc_cfg, doc(cfg(all(feature = "le", feature = "std"))))]
pub fn to_le_writer<W, T>(header: &str, writer: &mut W, value: &T) -> Result<()>
where
    W: ?Sized + io::Write,
    T: ?Sized + Serialize,
{
    let mut writer = IoWriter::new(writer);
    let emitter = BinaryEmitter::new(LeEncoder);
    let mut serializer = RootSerializer::new(header, false, emitter, &mut writer);
    value.serialize(&mut serializer)
}

/// TODO
///
/// # Errors
///
/// TODO
#[cfg(feature = "le")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "le")))]
pub fn to_le_vec<T>(header: &str, value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut vec = Vec::new();
    let emitter = BinaryEmitter::new(LeEncoder);
    let mut serializer = RootSerializer::new(header, true, emitter, &mut vec);
    value.serialize(&mut serializer)?;
    Ok(vec)
}

/// TODO
///
/// # Errors
///
/// TODO
#[cfg(all(feature = "be", feature = "std"))]
#[cfg_attr(doc_cfg, doc(cfg(all(feature = "be", feature = "std"))))]
pub fn to_be_writer<W, T>(header: &str, writer: &mut W, value: &T) -> Result<()>
where
    W: ?Sized + io::Write,
    T: ?Sized + Serialize,
{
    let mut writer = IoWriter::new(writer);
    let emitter = BinaryEmitter::new(BeEncoder);
    let mut serializer = RootSerializer::new(header, false, emitter, &mut writer);
    value.serialize(&mut serializer)
}

/// TODO
///
/// # Errors
///
/// TODO
#[cfg(feature = "be")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "be")))]
pub fn to_be_vec<T>(header: &str, value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut vec = Vec::new();
    let emitter = BinaryEmitter::new(BeEncoder);
    let mut serializer = RootSerializer::new(header, false, emitter, &mut vec);
    value.serialize(&mut serializer)?;
    Ok(vec)
}

#[cfg_attr(doc_cfg, doc(cfg(feature = "binary")))]
pub struct BinaryEmitter<E: Encode> {
    encoder: E,
}

impl<E: Encode> BinaryEmitter<E> {
    #[must_use]
    #[inline]
    pub fn new(encoder: E) -> Self {
        BinaryEmitter { encoder }
    }
}

impl<E: Encode> BinaryEmitter<E> {}

impl<E: Encode> Emit for BinaryEmitter<E> {
    #[inline]
    fn emit_header_id<W>(&mut self, writer: &mut W, id: Id) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        encode_id(writer, id)
    }

    #[inline]
    fn emit_bool<W>(&mut self, writer: &mut W, value: bool) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.emit_i8(writer, value.into())
    }

    #[inline]
    fn emit_i8<W>(&mut self, writer: &mut W, value: i8) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_ne_bytes())
    }

    #[inline]
    fn emit_i16<W>(&mut self, writer: &mut W, value: i16) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.encoder.encode_i16(writer, value)
    }

    #[inline]
    fn emit_i32<W>(&mut self, writer: &mut W, value: i32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.encoder.encode_i32(writer, value)
    }

    #[inline]
    fn emit_i64<W>(&mut self, writer: &mut W, value: i64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.encoder.encode_i64(writer, value)
    }

    #[inline]
    fn emit_f32<W>(&mut self, writer: &mut W, value: f32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.encoder.encode_f32(writer, value)
    }

    #[inline]
    fn emit_f64<W>(&mut self, writer: &mut W, value: f64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.encoder.encode_f64(writer, value)
    }

    #[inline]
    fn emit_str<W>(&mut self, writer: &mut W, value: &str) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        let encoded = mutf8::encode(value);
        self.encoder.encode_str_len(writer, encoded.len())?;
        writer.write_all(&encoded)
    }

    #[inline]
    fn begin_seq<W>(
        &mut self,
        writer: &mut W,
        kind: SeqKind,
        len: Option<usize>,
    ) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        if let SeqKind::List(id) = kind {
            encode_id(writer, id)?;
        }

        if let Some(len) = len {
            self.encoder.encode_seq_len(writer, len)
        } else {
            Err(zc_io::error!(
                InvalidInput,
                "binary specifications of NBT cannot serialize unsized sequences"
            ))
        }
    }

    #[inline]
    fn before_element<W>(&mut self, _writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        Ok(())
    }

    #[inline]
    fn after_element<W>(&mut self, _writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        Ok(())
    }

    #[inline]
    fn end_seq<W>(&mut self, _writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        Ok(())
    }

    #[inline]
    fn begin_map<W>(&mut self, _writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        Ok(())
    }

    #[inline]
    fn before_key<W>(&mut self, writer: &mut W, hint: Kind) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        encode_id(writer, hint.to_id())
    }

    #[inline]
    fn emit_key<W>(&mut self, writer: &mut W, key: &str) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        self.emit_str(writer, key)
    }

    #[inline]
    fn after_key<W>(&mut self, _writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        Ok(())
    }

    #[inline]
    fn before_value<W>(&mut self, _writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        Ok(())
    }

    #[inline]
    fn after_value<W>(&mut self, _writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        Ok(())
    }

    #[inline]
    fn end_map<W>(&mut self, writer: &mut W) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        encode_id(writer, Id::End)
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        false
    }
}

fn encode_id<W>(writer: &mut W, id: Id) -> zc_io::Result<()>
where
    W: ?Sized + Write,
{
    writer.write_all(&[id.to_u8()])
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "varint")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "varint")))]
pub struct VarintEncoder;

#[cfg(feature = "varint")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "varint")))]
impl Encode for VarintEncoder {
    #[inline]
    fn encode_i16<W>(&mut self, writer: &mut W, value: i16) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    #[inline]
    fn encode_i32<W>(&mut self, writer: &mut W, value: i32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        mini_leb128::write_u32(writer, value.zigzag())?;
        Ok(())
    }

    #[inline]
    fn encode_i64<W>(&mut self, writer: &mut W, value: i64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        mini_leb128::write_u64(writer, value.zigzag())?;
        Ok(())
    }

    #[inline]
    fn encode_f32<W>(&mut self, writer: &mut W, value: f32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    #[inline]
    fn encode_f64<W>(&mut self, writer: &mut W, value: f64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    #[inline]
    fn encode_str_len<W>(&mut self, writer: &mut W, value: usize) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        let len = i32::try_from(value).map_err(|_|
            zc_io::error!(
                InvalidData,
                "the varint specification of NBT does not support strings greater than 2,147,483,647 bytes when encoded in MUTF-8"
            )
        )?;

        mini_leb128::write_i32(writer, len)?;
        Ok(())
    }

    #[inline]
    fn encode_seq_len<W>(&mut self, writer: &mut W, value: usize) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        let len = i32::try_from(value).map_err(|_| {
            zc_io::error!(
                InvalidData,
                "the varint specification of NBT does not support sequences greater than 2,147,483,647 elements"
            )
        })?;
        mini_leb128::write_u32(writer, len.zigzag())?;
        Ok(())
    }
}

#[cfg(feature = "le")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "le")))]
pub struct LeEncoder;

#[cfg(feature = "le")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "le")))]
impl Encode for LeEncoder {
    #[inline]
    fn encode_i16<W>(&mut self, writer: &mut W, value: i16) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    #[inline]
    fn encode_i32<W>(&mut self, writer: &mut W, value: i32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    #[inline]
    fn encode_i64<W>(&mut self, writer: &mut W, value: i64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    #[inline]
    fn encode_f32<W>(&mut self, writer: &mut W, value: f32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    #[inline]
    fn encode_f64<W>(&mut self, writer: &mut W, value: f64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    #[inline]
    fn encode_str_len<W>(&mut self, writer: &mut W, value: usize) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        let len = u16::try_from(value).map_err(|_|
            zc_io::error!(
                InvalidData,
                "the little-endian specification of NBT does not support strings greater than 65,535 bytes when encoded in MUTF-8"
            )
        )?;
        writer.write_all(&len.to_be_bytes())
    }

    #[inline]
    fn encode_seq_len<W>(&mut self, writer: &mut W, value: usize) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        let len = i32::try_from(value).map_err(|_| {
            zc_io::error!(
                InvalidData,
                "the little-endian specification of NBT does not support sequences greater than 2,147,483,647 elements"
            )
        })?;
        writer.write_all(&len.to_be_bytes())
    }
}

#[cfg(feature = "be")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "be")))]
pub struct BeEncoder;

#[cfg(feature = "be")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "be")))]
impl Encode for BeEncoder {
    #[inline]
    fn encode_i16<W>(&mut self, writer: &mut W, value: i16) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_be_bytes())
    }

    #[inline]
    fn encode_i32<W>(&mut self, writer: &mut W, value: i32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_be_bytes())
    }

    #[inline]
    fn encode_i64<W>(&mut self, writer: &mut W, value: i64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_be_bytes())
    }

    #[inline]
    fn encode_f32<W>(&mut self, writer: &mut W, value: f32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_be_bytes())
    }

    #[inline]
    fn encode_f64<W>(&mut self, writer: &mut W, value: f64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_be_bytes())
    }

    #[inline]
    fn encode_str_len<W>(&mut self, writer: &mut W, value: usize) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        let len = u16::try_from(value).map_err(|_|
            zc_io::error!(
                InvalidData,
                "the big-endian specification of NBT does not support strings greater than 65,535 bytes when encoded in MUTF-8"
            )
        )?;
        writer.write_all(&len.to_be_bytes())
    }

    #[inline]
    fn encode_seq_len<W>(&mut self, writer: &mut W, value: usize) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        let len = i32::try_from(value).map_err(|_| {
            zc_io::error!(
                InvalidData,
                "the big-endian specification of NBT does not support sequences greater than 2,147,483,647 elements"
            )
        })?;
        writer.write_all(&len.to_be_bytes())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg_attr(doc_cfg, doc(cfg(feature = "binary")))]
#[allow(clippy::missing_errors_doc)]
pub trait Encode {
    #[inline]
    fn encode_i8<W>(&mut self, writer: &mut W, value: i8) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_ne_bytes())
    }

    fn encode_i16<W>(&mut self, writer: &mut W, value: i16) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn encode_i32<W>(&mut self, writer: &mut W, value: i32) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn encode_i64<W>(&mut self, writer: &mut W, value: i64) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn encode_f32<W>(&mut self, writer: &mut W, value: f32) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn encode_f64<W>(&mut self, writer: &mut W, value: f64) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn encode_str_len<W>(&mut self, writer: &mut W, value: usize) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn encode_seq_len<W>(&mut self, writer: &mut W, value: usize) -> zc_io::Result<()>
    where
        W: ?Sized + Write;
}
