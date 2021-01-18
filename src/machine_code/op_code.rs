use std::convert::TryFrom;

use crate::models::Op;

use super::WrongOpCode;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpCode {
    Add = 0,
    Sub = 1,
    Mul = 2,
    Div = 3,
    Mod = 4,
    Input = 5,
    Output = 6,
    Halt = 7,
    PushValue = 8,
    PushRegister = 9,
    PopRegister = 10,
}

impl OpCode {
    pub const fn op_len(&self) -> usize {
        use OpCode::*;

        match self {
            PushValue => 5,
            PushRegister | PopRegister => 2,
            _ => 1,
        }
    }
}

impl From<&Op> for OpCode {
    fn from(op: &Op) -> Self {
        use OpCode::*;

        match op {
            Op::Add => Add,
            Op::Sub => Sub,
            Op::Mul => Mul,
            Op::Div => Div,
            Op::Mod => Mod,
            Op::Input => Input,
            Op::Output => Output,
            Op::Halt => Halt,
            Op::PushValue(_) => PushValue,
            Op::PushRegister(_) => PushRegister,
            Op::PopRegister(_) => PopRegister,
        }
    }
}

// Conversions for OpCode

impl From<OpCode> for u8 {
    fn from(op_code: OpCode) -> u8 {
        op_code as u8
    }
}

impl TryFrom<u8> for OpCode {
    type Error = WrongOpCode;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        use OpCode::*;

        match byte {
            x if x == Add.into() => Ok(Add),
            x if x == Sub.into() => Ok(Sub),
            x if x == Mul.into() => Ok(Mul),
            x if x == Div.into() => Ok(Div),
            x if x == Mod.into() => Ok(Mod),
            x if x == Input.into() => Ok(Input),
            x if x == Output.into() => Ok(Output),
            x if x == Halt.into() => Ok(Halt),
            x if x == PushValue.into() => Ok(PushValue),
            x if x == PushRegister.into() => Ok(PushRegister),
            x if x == PopRegister.into() => Ok(PopRegister),
            x => Err(WrongOpCode { op_code: x }),
        }
    }
}
