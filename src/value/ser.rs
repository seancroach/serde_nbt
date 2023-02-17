//! TODO

use super::{Byte, Compound, Id, List, Value};

use crate::{
    error::Result,
    ser::{Emit, EmitMap, EmitSeq, Serializer},
    SeqKind,
};

use ahash::RandomState;
use serde::Serialize;

pub fn to_value<T>(value: &T) -> Result<Value>
where
    T: ?Sized + Serialize,
{
    let mut emitter = ValueEmitter::new(true);
    let mut serializer = Serializer::new(emitter);
    let value = value.serialize(&mut serializer)?;
    Ok(value)
}

pub struct ValueEmitter {
    is_human_readable: bool,
}

impl ValueEmitter {
    pub fn new(is_human_readable: bool) -> Self {
        ValueEmitter { is_human_readable }
    }
}

impl Emit for ValueEmitter {
    type Output = Value;

    type EmitSeq = ValueSeqEmitter;
    type EmitMap = ValueMapEmitter;

    fn emit_bool(self, value: bool) -> zc_io::Result<Self::Output> {
        Ok(Value::Byte(Byte::Bool(value)))
    }

    fn emit_i8(self, value: i8) -> zc_io::Result<Self::Output> {
        Ok(Value::Byte(Byte::I8(value)))
    }

    fn emit_i16(self, value: i16) -> zc_io::Result<Self::Output> {
        Ok(Value::Short(value))
    }

    fn emit_i32(self, value: i32) -> zc_io::Result<Self::Output> {
        Ok(Value::Int(value))
    }

    fn emit_i64(self, value: i64) -> zc_io::Result<Self::Output> {
        Ok(Value::Long(value))
    }

    fn emit_f32(self, value: f32) -> zc_io::Result<Self::Output> {
        Ok(Value::Float(value))
    }

    fn emit_f64(self, value: f64) -> zc_io::Result<Self::Output> {
        Ok(Value::Double(value))
    }

    fn emit_str(self, value: &str) -> zc_io::Result<Self::Output> {
        Ok(Value::String(value.to_string()))
    }

    fn emit_seq(self, kind: SeqKind, len: Option<usize>) -> zc_io::Result<Self::EmitSeq> {
        let list = List::with_capacity_and_id(len.unwrap_or_default(), kind.element_id());
        Ok(ValueSeqEmitter::new(list, self.is_human_readable))
    }

    fn emit_map(self, len: Option<usize>) -> zc_io::Result<Self::EmitMap> {
        let compound =
            Compound::with_capacity_and_hasher(len.unwrap_or_default(), RandomState::new());
        Ok(ValueMapEmitter::new(compound))
    }

    fn is_human_readable(&self) -> bool {
        self.is_human_readable
    }
}

pub struct ValueSeqEmitter {
    is_human_readable: bool,
    list: List,
}

impl ValueSeqEmitter {
    fn new(list: List, is_human_readable: bool) -> Self {
        ValueSeqEmitter {
            list,
            is_human_readable,
        }
    }
}

impl EmitSeq for ValueSeqEmitter {
    type Output = Value;

    fn begin_element(&mut self) -> zc_io::Result<()> {
        Ok(())
    }

    fn handle_element(&mut self, value: Self::Output) -> zc_io::Result<()> {
        self.list.push(value).unwrap();
        Ok(())
    }

    fn end_element(&mut self) -> zc_io::Result<()> {
        Ok(())
    }

    fn finish(self) -> zc_io::Result<Self::Output> {
        Ok(Value::List(self.list))
    }
}

pub struct ValueMapEmitter {
    compound: Compound,
    key: Option<String>,
}

impl ValueMapEmitter {
    fn new(compound: Compound) -> Self {
        ValueMapEmitter {
            compound,
            key: None,
        }
    }
}

impl EmitMap for ValueMapEmitter {
    type Output = Value;

    fn begin_key(&mut self, _hint: Id) -> zc_io::Result<()> {
        Ok(())
    }

    fn emit_key(&mut self, key: &str) -> zc_io::Result<()> {
        self.key = Some(key.to_string());
        Ok(())
    }

    fn end_key(&mut self) -> zc_io::Result<()> {
        Ok(())
    }

    fn begin_value(&mut self) -> zc_io::Result<()> {
        Ok(())
    }

    fn handle_value(&mut self, value: Self::Output) -> zc_io::Result<()> {
        self.compound.insert(self.key.take().unwrap(), value);
        Ok(())
    }

    fn end_value(&mut self) -> zc_io::Result<()> {
        Ok(())
    }

    fn finish(self) -> zc_io::Result<Self::Output> {
        Ok(Value::Compound(self.compound))
    }
}
