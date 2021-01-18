use std::{fmt::Display, str::FromStr};

use super::{
    ArgumentParseError,
    OpParseError::{self, *},
    Register, Value,
};

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
    type Err = OpParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Op::*;

        let s = s.to_uppercase();
        let words = s.trim().split_whitespace().collect::<Vec<&str>>();

        match (words[0], &words[1..]) {
            ("ADD", []) => Ok(Add),
            ("SUB", []) => Ok(Sub),
            ("MUL", []) => Ok(Mul),
            ("DIV", []) => Ok(Div),
            ("MOD", []) => Ok(Mod),
            ("INPUT", []) => Ok(Input),
            ("OUTPUT", []) => Ok(Output),
            ("HALT", []) => Ok(Halt),
            ("PUSH", [arg]) => parse_push(arg),
            ("POP", [register]) => match register.parse() {
                Ok(register) => Ok(PopRegister(register)),
                Err(err) => Err(WrongArguments {
                    op: "POP",
                    errors: vec![(0, err)],
                }),
            },
            (op, args) => Err(WrongOp {
                op: op.to_owned(),
                num_args: args.len(),
            }),
        }
    }
}

fn parse_push<'a>(arg: &'a str) -> Result<Op, OpParseError> {
    if let Ok(value) = arg.parse() {
        Ok(Op::PushValue(value))
    } else if let Ok(register) = arg.parse() {
        Ok(Op::PushRegister(register))
    } else {
        Err(WrongArguments {
            op: "PUSH",
            errors: vec![(0, ArgumentParseError::WrongRegisterOrValue(arg.to_owned()))],
        })?
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

    #[test]
    fn wrong_op_name() {
        assert_eq!(
            Op::from_str("PUSH 1 2 3").unwrap_err(),
            WrongOp {
                op: "PUSH".to_owned(),
                num_args: 3,
            }
        );

        assert_eq!(
            Op::from_str("HELLO WORLD THERE").unwrap_err(),
            WrongOp {
                op: "HELLO".to_owned(),
                num_args: 2,
            }
        );

        assert_eq!(
            Op::from_str("ADD 1").unwrap_err(),
            WrongOp {
                op: "ADD".to_owned(),
                num_args: 1,
            }
        );
    }

    #[test]
    fn wrong_arguments_number() {
        assert_eq!(
            Op::from_str("ADD 1").unwrap_err(),
            OpParseError::WrongOp {
                op: "ADD".into(),
                num_args: 1,
            }
        );

        assert_eq!(
            Op::from_str("PUSH 1 2").unwrap_err(),
            OpParseError::WrongOp {
                op: "PUSH".into(),
                num_args: 2,
            }
        )
    }

    #[test]
    fn failure_wrong_argument() {
        assert_eq!(
            Op::from_str("PUSH hello").unwrap_err(),
            OpParseError::WrongArguments {
                op: "PUSH",
                errors: vec![(0, ArgumentParseError::WrongRegisterOrValue("HELLO".into()))],
            }
        );

        assert_eq!(
            Op::from_str("POP x").unwrap_err(),
            OpParseError::WrongArguments {
                op: "POP",
                errors: vec![(0, ArgumentParseError::WrongRegister("X".into()))],
            }
        );
    }
}
