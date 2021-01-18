mod assembly;
mod error;
mod op;
mod op_code;
mod register;
mod value;

use std::io;

pub use error::*;
pub use op_code::OpCode;

pub trait Compile {
    type Error;

    fn compile(&self, output: &mut impl io::Write) -> Result<(), Self::Error>;
}

pub struct DecompileResult<T> {
    pub bytes_read: usize,
    pub value: T,
}

pub trait Decompile: Sized {
    type Error;

    fn decompile(bytes: &[u8]) -> Result<DecompileResult<Self>, Self::Error>;
}
