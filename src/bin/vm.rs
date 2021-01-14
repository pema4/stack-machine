use std::{env, fmt};
use std::io;
use std::fs;
use std::error;
use stack_machine::executor::Machine;

#[derive(Debug)]
enum VmError {
    UsageError,
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::UsageError => write!(f, "Usage: vm path/to/program.s")
        }
    }
}

impl error::Error for VmError {}

fn inner_main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    match &args[1..] {
        [program] => {
            let program = fs::read(program)?;
            let input = io::stdin();
            let output = io::stdout();
            let mut machine = Machine::new(
                input.lock(),
                output.lock(),
            );
            machine.execute_program(&program[..])?;
        },
        _ => Err(VmError::UsageError)?,
    };
    Ok(())
}

fn main() {
    if let Err(err) = inner_main() {
        println!("{}", err);
    }
}