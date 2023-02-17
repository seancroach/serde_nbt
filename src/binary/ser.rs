use super::mutf8;

use crate::{
    error::{Error, Path, Result},
    ser::{Emit, EmitMap, EmitSeq},
    value::Id,
    SeqKind,
};

use zc_io::{error, Write};

use zende::Zigzag;

pub struct NbtEmitter<E, W>
where
    E: Encode,
    W: Write,
{
    encoder: E,
    writer: W,
}

impl<E, W> NbtEmitter<E, W>
where
    E: Encode,
    W: Write,
{
    fn new(encoder: E, writer: W) -> Self {
        NbtEmitter { encoder, writer }
    }
}

impl<'a, E, W> Emit for &'a mut NbtEmitter<E, W>
where
    E: Encode,
    W: Write,
{
    type Output = ();

    type EmitSeq = NbtSeqEmitter;
    type EmitMap = NbtMapEmitter<'a, E, W>;

    fn emit_bool(&mut self, value: bool) -> zc_io::Result<Self::Output> {
        self.encoder
            .encode_i8(&mut self.writer, if value { 1 } else { 0 })
    }

    fn emit_i8(&mut self, value: i8) -> zc_io::Result<Self::Output> {
        self.encoder.encode_i8(&mut self.writer, value)
    }

    fn emit_i16(&mut self, value: i16) -> zc_io::Result<Self::Output> {
        self.encoder.encode_i16(&mut self.writer, value)
    }

    fn emit_i32(&mut self, value: i32) -> zc_io::Result<Self::Output> {
        self.encoder.encode_i32(&mut self.writer, value)
    }

    fn emit_i64(&mut self, value: i64) -> zc_io::Result<Self::Output> {
        self.encoder.encode_i64(&mut self.writer, value)
    }

    fn emit_f32(&mut self, value: f32) -> zc_io::Result<Self::Output> {
        self.encoder.encode_f32(&mut self.writer, value)
    }

    fn emit_f64(&mut self, value: f64) -> zc_io::Result<Self::Output> {
        self.encoder.encode_f64(&mut self.writer, value)
    }

    fn emit_str(&mut self, value: &str) -> zc_io::Result<Self::Output> {
        let data = mutf8::encode(value);
        self.encoder
            .encode_str_len(&mut self.writer, data.len())
            .and_then(|_| self.writer.write_all(&data))
    }

    fn emit_seq(&mut self, kind: SeqKind, len: Option<usize>) -> zc_io::Result<Self::EmitSeq> {
        let result = if let SeqKind::List(id) = kind {
            self.encoder.encode_id(&mut self.writer, id)
        } else {
            Ok(())
        };

        result.and_then(|_| {
            if let Some(len) = len {
                self.encoder.encode_seq_len(&mut self.writer, len)?;
                Ok(NbtSeqEmitter)
            } else {
                Err(zc_io::error!(
                    InvalidInput,
                    "NBT does not support unsized sequences"
                ))
            }
        })
    }

    fn emit_map(&mut self, _len: Option<usize>) -> zc_io::Result<Self::EmitMap> {
        Ok(NbtMapEmitter::new(self))
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

pub struct NbtSeqEmitter;

impl EmitSeq for NbtSeqEmitter {
    type Output = ();

    fn begin_element(&mut self) -> zc_io::Result<()> {
        Ok(())
    }

    fn handle_element(&mut self, value: Self::Output) -> zc_io::Result<()> {
        Ok(())
    }

    fn end_element(&mut self) -> zc_io::Result<()> {
        Ok(())
    }

    fn finish(self) -> zc_io::Result<Self::Output> {
        Ok(())
    }
}

pub struct NbtMapEmitter<'a, E, W>
where
    E: Encode,
    W: Write,
{
    emitter: &'a mut NbtEmitter<E, W>,
}

impl<'a, E, W> NbtMapEmitter<'a, E, W>
where
    E: Encode,
    W: Write,
{
    fn new(emitter: &'a mut NbtEmitter<E, W>) -> Self {
        NbtMapEmitter { emitter }
    }
}

impl<'a, E, W> EmitMap for NbtMapEmitter<'a, E, W>
where
    E: Encode,
    W: Write,
{
    type Output = ();

    fn begin_key(&mut self, hint: Id) -> zc_io::Result<()> {
        self.emitter
            .encoder
            .encode_id(&mut self.emitter.writer, hint)
    }

    fn emit_key(&mut self, key: &str) -> zc_io::Result<()> {
        self.emitter.emit_str(key)
    }

    fn end_key(&mut self) -> zc_io::Result<()> {
        Ok(())
    }

    fn begin_value(&mut self) -> zc_io::Result<()> {
        Ok(())
    }

    fn handle_value(&mut self, _value: Self::Output) -> zc_io::Result<()> {
        Ok(())
    }

    fn end_value(&mut self) -> zc_io::Result<()> {
        Ok(())
    }

    fn finish(self) -> zc_io::Result<Self::Output> {
        Ok(())
    }
}

pub struct JavaEncoder;

impl Encode for JavaEncoder {
    fn encode_id<W>(&mut self, writer: &mut W, value: Id) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&[value.to_u8()])
    }

    fn encode_i8<W>(&mut self, writer: &mut W, value: i8) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_be_bytes())
    }

    fn encode_i16<W>(&mut self, writer: &mut W, value: i16) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_be_bytes())
    }

    fn encode_i32<W>(&mut self, writer: &mut W, value: i32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_be_bytes())
    }

    fn encode_i64<W>(&mut self, writer: &mut W, value: i64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_be_bytes())
    }

    fn encode_f32<W>(&mut self, writer: &mut W, value: f32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_be_bytes())
    }

    fn encode_f64<W>(&mut self, writer: &mut W, value: f64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_be_bytes())
    }

    fn encode_str_len<W>(&mut self, writer: &mut W, value: usize) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        let len = u16::try_from(value).map_err(|_| {
            zc_io::error!(
                InvalidInput,
                "NBT does not support strings longer than 65,535 bytes"
            )
        })?;
        writer.write_all(&len.to_be_bytes())
    }

    fn encode_seq_len<W>(&mut self, writer: &mut W, value: usize) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        let len = i32::try_from(value).map_err(|_| {
            zc_io::error!(
                InvalidInput,
                "NBT does not support sequences longer than 2,147,483,647 elements"
            )
        })?;
        self.encode_i32(writer, len)
    }
}

pub struct BedrockDiskEncoder;

impl Encode for BedrockDiskEncoder {
    fn encode_id<W>(&mut self, writer: &mut W, value: Id) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&[value.to_u8()])
    }

    fn encode_i8<W>(&mut self, writer: &mut W, value: i8) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    fn encode_i16<W>(&mut self, writer: &mut W, value: i16) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    fn encode_i32<W>(&mut self, writer: &mut W, value: i32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    fn encode_i64<W>(&mut self, writer: &mut W, value: i64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    fn encode_f32<W>(&mut self, writer: &mut W, value: f32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    fn encode_f64<W>(&mut self, writer: &mut W, value: f64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    fn encode_str_len<W>(&mut self, writer: &mut W, value: usize) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        let len = u16::try_from(value).map_err(|_| {
            zc_io::error!(
                InvalidInput,
                "NBT does not support strings longer than 65,535 bytes"
            )
        })?;
        writer.write_all(&len.to_le_bytes())
    }

    fn encode_seq_len<W>(&mut self, writer: &mut W, value: usize) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        let len = i32::try_from(value).map_err(|_| {
            zc_io::error!(
                InvalidInput,
                "NBT does not support sequences longer than 2,147,483,647 elements"
            )
        })?;
        self.encode_i32(writer, len)
    }
}

pub struct BedrockNetworkEncoder;

impl Encode for BedrockNetworkEncoder {
    fn encode_id<W>(&mut self, writer: &mut W, value: Id) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&[value.to_u8()])
    }

    fn encode_i8<W>(&mut self, writer: &mut W, value: i8) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    fn encode_i16<W>(&mut self, writer: &mut W, value: i16) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    fn encode_i32<W>(&mut self, writer: &mut W, value: i32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        mini_leb128::write_u32(writer, value.zigzag())?;
        Ok(())
    }

    fn encode_i64<W>(&mut self, writer: &mut W, value: i64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        mini_leb128::write_u64(writer, value.zigzag())?;
        Ok(())
    }

    fn encode_f32<W>(&mut self, writer: &mut W, value: f32) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    fn encode_f64<W>(&mut self, writer: &mut W, value: f64) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        writer.write_all(&value.to_le_bytes())
    }

    fn encode_str_len<W>(&mut self, writer: &mut W, value: usize) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        let len = i32::try_from(value).map_err(|_| {
            zc_io::error!(
                InvalidInput,
                "NBT does not support strings longer than 2,147,483,647 bytes"
            )
        })?;
        mini_leb128::write_i32(writer, len)?;
        Ok(())
    }

    fn encode_seq_len<W>(&mut self, writer: &mut W, value: usize) -> zc_io::Result<()>
    where
        W: ?Sized + Write,
    {
        let len = i32::try_from(value).map_err(|_| {
            zc_io::error!(
                InvalidInput,
                "NBT does not support sequences longer than 2,147,483,647 elements"
            )
        })?;
        self.encode_i32(writer, len)
    }
}

pub trait Encode {
    fn encode_id<W>(&mut self, writer: &mut W, value: Id) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

    fn encode_i8<W>(&mut self, writer: &mut W, value: i8) -> zc_io::Result<()>
    where
        W: ?Sized + Write;

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
