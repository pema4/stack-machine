use std::{convert::TryInto, io};

use super::{ExecutionError, InputError, OutputError, Registers, Stack};
use crate::{
    machine_code::{Decompile, OpCode},
    models::{Op, Value},
};

/// Result of successful operation execution.
///
/// Basically it tells what to do next.
pub enum NextOperation {
    /// Program has not terminated jet, execute operation specified offset.
    Offset(isize),

    /// Program terminated, don't do anything.
    None,
}

/// Return value of `Machine::execute` method
type ExecutionResult = Result<NextOperation, ExecutionError>;

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
    /// Returns Machine instance with specified input and output streams.
    pub fn new(input: I, output: O) -> Self {
        Machine {
            registers: Registers::default(),
            stack: Stack::default(),
            input: input,
            output: output,
        }
    }

    /// Executes compiled program.
    pub fn execute_program(&mut self, bytes: &[u8]) -> Result<(), ExecutionError> {
        let mut idx = 0;
        while idx < bytes.len() {
            let op = Op::decompile(&bytes[idx..])?;
            self.execute(op.value)?;
            idx += op.bytes_read;
        }
        Ok(())
    }

    pub fn execute(&mut self, op: Op) -> ExecutionResult {
        use Op::*;

        match op {
            Add => self.binary_fn(|a, b| a + b)?,
            Sub => self.binary_fn(|a, b| a - b)?,
            Mul => self.binary_fn(|a, b| a * b)?,
            Div => self.binary_fn(|a, b| a / b)?,
            Mod => self.binary_fn(|a, b| a % b)?,
            Input => self.input()?,
            Output => self.output()?,
            Halt => return Ok(NextOperation::None),
            PushValue(Value(v)) => self.stack.push(v),
            PushRegister(r) => self.stack.push(self.registers[r]),
            PopRegister(r) => self.registers[r] = self.stack.pop()?,
        };

        let offset = OpCode::from(&op).op_len().try_into().unwrap();
        Ok(NextOperation::Offset(offset))
    }

    fn binary_fn<F>(&mut self, f: F) -> Result<(), ExecutionError>
    where
        F: FnOnce(i32, i32) -> i32,
    {
        let b = self.stack.pop()?;
        let a = self.stack.pop()?;
        let result = f(a, b);
        self.stack.push(result);
        Ok(())
    }

    fn input(&mut self) -> Result<(), ExecutionError> {
        let mut buf = String::new();
        print!("Enter number: ");
        let input = loop {
            self.input.read_line(&mut buf).map_err(InputError::from)?;
            if let Ok(x) = buf.parse() {
                break x;
            } else {
                print!("Try again: ");
            }
        };
        self.stack.push(input);
        Ok(())
    }

    fn output(&mut self) -> Result<(), ExecutionError> {
        let value = self.stack.pop()?;
        writeln!(self.output, "{}", value).map_err(OutputError::from)?;
        Ok(())
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
