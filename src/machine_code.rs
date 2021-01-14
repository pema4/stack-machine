use std::convert::{TryFrom, TryInto};

use crate::models::{Assembly, Op, Register, Statement, Value};

pub fn read_op(bytes: &[u8]) -> Result<(Op, usize), String> {
    let op_code = OpCode::try_from(bytes[0])?;
    let op_len = op_code.op_len();
    let op = Op::try_from(&bytes[0..op_len])?;
    Ok((op, op_len))
}

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

impl From<Op> for OpCode {
    fn from(op: Op) -> Self {
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
    type Error = String;

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
            x => Err(format!("Wrong op code byte: {}", x)),
        }
    }
}

// Conversions for Register

impl TryFrom<u8> for Register {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Register::*;

        match value {
            x if x == A.into() => Ok(A),
            x if x == B.into() => Ok(B),
            x if x == C.into() => Ok(C),
            x if x == D.into() => Ok(D),
            _ => Err(format!("Byte {} is not a valid register", value)),
        }
    }
}

impl From<Register> for u8 {
    fn from(value: Register) -> Self {
        value as u8
    }
}

// Conversions for Value

impl TryFrom<&[u8]> for Value {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if let Ok(bytes) = bytes.try_into() {
            Ok(Value(i32::from_be_bytes(bytes)))
        } else {
            Err("")
        }
    }
}

impl From<Value> for Vec<u8> {
    fn from(value: Value) -> Self {
        value.0.to_be_bytes().into()
    }
}

// Conversions for Op

impl From<Op> for Vec<u8> {
    fn from(op: Op) -> Self {
        let op_code: u8 = OpCode::from(op).into();
        match op {
            Op::PushValue(v) => {
                let mut res = vec![op_code];
                res.append(&mut v.into());
                res
            },
            Op::PushRegister(r) => vec![op_code, r.into()],
            Op::PopRegister(r) => vec![op_code, r.into()],
            _ => vec![op_code],
        }
    }
}

impl TryFrom<&[u8]> for Op {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        use Op::*;

        if bytes == &[] {
            return Err("Op must be represented by at least one byte".into());
        }

        match (bytes[0], &bytes[1..]) {
            (x, []) if x == OpCode::Add.into() => Ok(Add),
            (x, []) if x == OpCode::Sub.into() => Ok(Sub),
            (x, []) if x == OpCode::Mul.into() => Ok(Mul),
            (x, []) if x == OpCode::Div.into() => Ok(Div),
            (x, []) if x == OpCode::Mod.into() => Ok(Mod),
            (x, []) if x == OpCode::Input.into() => Ok(Input),
            (x, []) if x == OpCode::Output.into() => Ok(Output),
            (x, []) if x == OpCode::Halt.into() => Ok(Halt),
            (x, bytes) if x == OpCode::PushValue.into() => {
                let v = bytes.try_into()?;
                Ok(PushValue(v))
            }
            (x, &[byte]) if x == OpCode::PushRegister.into() => {
                let reg = byte.try_into()?;
                Ok(PushRegister(reg))
            }
            (x, &[byte]) if x == OpCode::PopRegister.into() => {
                let reg = byte.try_into()?;
                Ok(PopRegister(reg))
            }
            (x, _) => Err(format!("Unknown OP code: {}", x)),
        }
    }
}

// Conversions for a whole Program.

impl TryFrom<&[u8]> for Assembly {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let mut idx = 0;
        let mut result = Vec::new();

        while idx < bytes.len() {
            let (op, op_len) = read_op(&bytes[idx..])?;
            result.push(Statement::Op(op));
            idx += op_len;
        }

        if idx != bytes.len() {
            let bytes_left = bytes.len() - idx;
            Err(format!(
                "Can't read whole program, {} bytes left",
                bytes_left
            ))
        } else {
            Assembly::try_from(result)
        }
    }
}

impl From<Assembly> for Vec<u8> {
    fn from(assembly: Assembly) -> Self {
        let mut result = Vec::new();
        for statement in assembly.statements() {
            match statement {
                Statement::Op(op) => result.append(&mut (*op).into()),
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_bytes() {
        let value = Value::from(42);
        let bytes = Vec::from(value);
        let value_again = Value::try_from(&bytes[..]);
        assert_eq!(value, value_again.unwrap())
    }

    #[test]
    fn from_wrong_sized_bytes() {
        let bytes: &[u8] = &[0; 5];
        assert!(Value::try_from(bytes).is_err());
    }
}
