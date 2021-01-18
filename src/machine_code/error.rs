use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Can't write compiled code: {0}")]
pub struct OutputError(#[from] io::Error);

#[derive(Error, Debug)]
#[error("Can't read compiled code: {0}")]
pub struct InputError(#[from] io::Error);

#[derive(Error, Debug)]
#[error("Unexpected end of file when reading {name}")]
pub struct EndOfInput {
    pub name: &'static str,
}

#[derive(Error, Debug)]
pub enum RegisterDecompileError {
    #[error(transparent)]
    EndOfInput(#[from] EndOfInput),

    #[error("Incorrect register byte: {register}")]
    WrongRegister { register: u8 },
}

#[derive(Error, Debug)]
#[error("Incorrect op code byte: {op_code}")]
pub struct WrongOpCode {
    pub op_code: u8,
}

#[derive(Error, Debug)]
pub enum OpDecompileError {
    #[error(transparent)]
    WrongOpCode(#[from] WrongOpCode),

    #[error(transparent)]
    RegisterDecompileError(#[from] RegisterDecompileError),

    #[error(transparent)]
    EndOfInput(#[from] EndOfInput),
}

#[derive(Error, Debug)]
pub enum AssemblyDecompileError {
    #[error(transparent)]
    OpDecompileError(#[from] OpDecompileError),
}
