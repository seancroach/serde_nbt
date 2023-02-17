mod emit;
mod framework;

pub use self::{
    emit::{Emit, EmitMap, EmitSeq},
    framework::Serializer,
};
