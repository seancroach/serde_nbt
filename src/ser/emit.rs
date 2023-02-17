use crate::{value::Id, SeqKind};

#[allow(clippy::missing_errors_doc)]
pub trait Emit: Sized {
    type Output;

    type EmitSeq: EmitSeq<Output = Self::Output>;
    type EmitMap: EmitMap<Output = Self::Output>;

    fn emit_bool(self, value: bool) -> zc_io::Result<Self::Output>;

    fn emit_i8(self, value: i8) -> zc_io::Result<Self::Output>;

    fn emit_i16(self, value: i16) -> zc_io::Result<Self::Output>;

    fn emit_i32(self, value: i32) -> zc_io::Result<Self::Output>;

    fn emit_i64(self, value: i64) -> zc_io::Result<Self::Output>;

    fn emit_f32(self, value: f32) -> zc_io::Result<Self::Output>;

    fn emit_f64(self, value: f64) -> zc_io::Result<Self::Output>;

    fn emit_str(self, value: &str) -> zc_io::Result<Self::Output>;

    fn emit_seq(self, kind: SeqKind, len: Option<usize>) -> zc_io::Result<Self::EmitSeq>;

    fn emit_map(self, len: Option<usize>) -> zc_io::Result<Self::EmitMap>;

    #[must_use]
    fn is_human_readable(&self) -> bool;
}

#[allow(clippy::missing_errors_doc)]
pub trait EmitSeq {
    type Output;

    fn begin_element(&mut self) -> zc_io::Result<()>;

    fn handle_element(&mut self, value: Self::Output) -> zc_io::Result<()>;

    fn end_element(&mut self) -> zc_io::Result<()>;

    fn finish(self) -> zc_io::Result<Self::Output>;
}

#[allow(clippy::missing_errors_doc)]
pub trait EmitMap {
    type Output;

    fn begin_key(&mut self, hint: Id) -> zc_io::Result<()>;

    fn emit_key(&mut self, key: &str) -> zc_io::Result<()>;

    fn end_key(&mut self) -> zc_io::Result<()>;

    fn begin_value(&mut self) -> zc_io::Result<()>;

    fn handle_value(&mut self, value: Self::Output) -> zc_io::Result<()>;

    fn end_value(&mut self) -> zc_io::Result<()>;

    fn finish(self) -> zc_io::Result<Self::Output>;
}
