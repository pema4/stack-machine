use crate::models::Register;

use super::{Compile, Decompile, DecompileResult, EndOfInput, OutputError, RegisterDecompileError};

impl Decompile for Register {
    type Error = RegisterDecompileError;

    fn decompile(bytes: &[u8]) -> Result<super::DecompileResult<Self>, Self::Error> {
        use Register::*;

        let register = match &bytes[..] {
            [] => Err(EndOfInput { name: "Register" })?,
            [0, ..] => A,
            [1, ..] => B,
            [2, ..] => C,
            [3, ..] => D,
            [x, ..] => Err(RegisterDecompileError::WrongRegister { register: *x })?,
        };

        Ok(DecompileResult {
            value: register,
            bytes_read: 1,
        })
    }
}

impl Compile for Register {
    type Error = OutputError;

    fn compile(&self, output: &mut impl std::io::Write) -> Result<(), Self::Error> {
        output.write(&[*self as u8])?;
        Ok(())
    }
}
