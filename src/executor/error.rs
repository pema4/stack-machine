use thiserror::Error;

use crate::machine_code::OpDecompileError;

#[derive(Error, Debug)]
#[error("Can't read from the input stream: {inner}")]
pub struct InputError {
    #[from]
    inner: std::io::Error,
}

#[derive(Error, Debug)]
#[error("Can't print to the output stream: {inner}")]
pub struct OutputError {
    #[from]
    inner: std::io::Error,
}

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Attemted to pop from empty stack")]
    StackUnderflow,

    #[error("{inner}")]
    InputError {
        #[from]
        inner: InputError,
    },

    #[error("{inner}")]
    OutputError {
        #[from]
        inner: OutputError,
    },

    #[error("{inner}")]
    OpReadError {
        #[from]
        inner: OpDecompileError,
    },
}
