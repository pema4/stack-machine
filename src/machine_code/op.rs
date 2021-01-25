use std::convert::TryFrom;

use crate::models::{Op, Register, Value};

use super::*;

impl Compile for Op {
    type Error = OutputError;

    fn compile(&self, output: &mut impl std::io::Write) -> Result<(), Self::Error> {
        let op_code: u8 = OpCode::from(self).into();
        output.write(&[op_code])?;

        match self {
            Op::PushValue(v) => v.compile(output)?,
            Op::PushRegister(r) | Op::PopRegister(r) => r.compile(output)?,
            _ => (),
        };
        Ok(())
    }
}

impl Decompile for Op {
    type Error = OpDecompileError;

    fn decompile(bytes: &[u8]) -> Result<DecompileResult<Self>, Self::Error> {
        use OpCode::*;

        let op_code = OpCode::try_from(bytes[0])?;

        let op = match (op_code, &bytes[1..]) {
            (Add, _) => Op::Add,
            (Sub, _) => Op::Sub,
            (Mul, _) => Op::Mul,
            (Div, _) => Op::Div,
            (Mod, _) => Op::Mod,
            (Input, _) => Op::Input,
            (Output, _) => Op::Output,
            (Halt, _) => Op::Halt,
            (PushValue, bytes) => {
                let v = Value::decompile(bytes)?;
                Op::PushValue(v.value)
            }
            (PushRegister, bytes) => {
                let reg = Register::decompile(bytes)?;
                Op::PushRegister(reg.value)
            }
            (PopRegister, bytes) => {
                let reg = Register::decompile(bytes)?;
                Op::PopRegister(reg.value)
            }
        };

        Ok(DecompileResult {
            value: op,
            bytes_read: OpCode::from(&op).op_len(),
        })
    }
}
