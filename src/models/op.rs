use std::{fmt::Display, str::FromStr};

use super::{Register, Value};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Input,
    Output,
    Halt,
    PushValue(Value),
    PushRegister(Register),
    PopRegister(Register),
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Op::*;

        let s = s.to_uppercase();
        let mut words = s.trim().split_whitespace();

        match words.next().ok_or("Empty line, nothing to parse")? {
            "ADD" => Ok(Add),
            "SUB" => Ok(Sub),
            "MUL" => Ok(Mul),
            "DIV" => Ok(Div),
            "MOD" => Ok(Mod),
            "INPUT" => Ok(Input),
            "OUTPUT" => Ok(Output),
            "HALT" => Ok(Halt),
            "PUSH" => {
                let arg = words.next().ok_or("Argument not specified")?;
                if let Ok(value) = arg.parse() {
                    Ok(PushValue(value))
                } else if let Ok(register) = arg.parse() {
                    Ok(PushRegister(register))
                } else {
                    Err("Push must have an argument of Value or Register")?
                }
            }
            "POP" => {
                let register = words.next().ok_or("Register not specified")?;
                let register = register.parse()?;
                Ok(PopRegister(register))
            }
            x => Err(format!("Wrong assembly operation name: {}", x)),
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Op::*;
        let mut w = |x| write!(f, "{}", x);
        match self {
            Add => w("ADD"),
            Sub => w("SUB"),
            Mul => w("MUL"),
            Div => w("DIV"),
            Mod => w("MOD"),
            Input => w("INPUT"),
            Output => w("OUTPUT"),
            Halt => w("HALT"),
            PushValue(v) => w(&format!("PUSH {}", v)),
            PushRegister(r) => w(&format!("PUSH {}", r)),
            PopRegister(r) => w(&format!("POP {}", r)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_and_from_string() {
        let ops = [
            Op::Add,
            Op::Sub,
            Op::Mul,
            Op::Div,
            Op::Mod,
            Op::PushValue(Value::from(42)),
            Op::PushRegister(Register::A),
            Op::PopRegister(Register::C),
        ];

        for op in ops.iter() {
            let restored_op: Op = op.to_string().parse().unwrap();
            assert_eq!(*op, restored_op);
        }
    }
}
