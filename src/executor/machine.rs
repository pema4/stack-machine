use std::io;

// use crate::common::*;
use super::{ExitCode, Registers, Stack};
use crate::{
    machine_code::read_op,
    models::{Op, Value},
};

pub struct Machine<I, O>
where
    I: io::BufRead,
    O: io::Write,
{
    registers: Registers,
    stack: Stack,
    input: I,
    output: O,
}

impl<I, O> Machine<I, O>
where
    I: io::BufRead,
    O: io::Write,
{
    pub fn new(input: I, output: O) -> Self {
        Machine {
            registers: Registers::default(),
            stack: Stack::default(),
            input: input,
            output: output,
        }
    }

    pub fn execute(&mut self, op: Op) -> Result<(), ExitCode> {
        use Op::*;
        let stack = &mut self.stack;

        match op {
            Add => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(a + b);
            }
            Sub => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(a - b);
            }
            Mul => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(a * b);
            }
            Div => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(a / b);
            }
            Mod => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(a % b);
            }
            Input => {
                let mut buf = String::new();
                print!("Enter number: ");
                let input = loop {
                    self.input
                        .read_line(&mut buf)
                        .map_err(|_| ExitCode::InputError)?;
                    if let Ok(x) = buf.parse() {
                        break x;
                    } else {
                        print!("Try again:")
                    }
                };
                stack.push(input);
            }
            Output => println!("{}", stack.pop()?),
            Halt => {
                write!(&mut self.output, "Program terminated successfully")
                    .map_err(|_| ExitCode::OutputError)?;
                return Err(ExitCode::Success);
            }
            PushValue(Value(v)) => stack.push(v),
            PushRegister(r) => stack.push(self.registers[r]),
            PopRegister(r) => self.registers[r] = stack.pop()?,
        };
        Ok(())
    }

    pub fn execute_program(&mut self, bytes: &[u8]) -> Result<(), String> {
        let mut idx = 0;
        while idx < bytes.len() {
            let (op, op_len) = read_op(&bytes[idx..])?;
            self.execute(op)?;
            idx += op_len;
        }
        Err("Program ended")?
    }
}

#[cfg(test)]
mod test {
    use std::io;

    use super::*;
    use crate::models::*;

    fn default_machine() -> Machine<impl io::BufRead, impl io::Write> {
        Machine {
            registers: Registers::default(),
            stack: Stack::default(),
            input: io::BufReader::new(io::stdin()),
            output: io::BufWriter::new(io::stdout()),
        }
    }

    fn execute(program: &[Op]) -> Option<i32> {
        let mut machine = default_machine();
        for &op in program.iter() {
            machine.execute(op).unwrap();
        }
        machine.stack.pop().ok()
    }

    #[test]
    fn two_plus_two() {
        let program = [Op::PushValue(2.into()), Op::PushValue(2.into()), Op::Add];
        assert_eq!(execute(&program[..]).unwrap(), 4);
    }

    #[test]
    fn divide_by_three() {
        let program = [Op::PushValue(9.into()), Op::PushValue(3.into()), Op::Div];
        assert_eq!(execute(&program[..]).unwrap(), 3);
    }

    #[test]
    fn swap() {
        let program = [
            Op::PushValue(1.into()),
            Op::PushValue(2.into()),
            // stack now is [1, 2]
            Op::PopRegister(Register::A),
            Op::PopRegister(Register::B),
            Op::PushRegister(Register::A),
            Op::PushRegister(Register::B),
            // stack now is [2, 1]
            Op::Sub,
            // result is 1
        ];
        assert_eq!(execute(&program[..]).unwrap(), 1);
    }
}
