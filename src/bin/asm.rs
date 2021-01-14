use stack_machine::models::Assembly;
use std::{
    env, error, fmt, fs,
    io::{self, prelude::*},
};

#[derive(Debug)]
enum AsmError {
    UsageError,
}

impl fmt::Display for AsmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsmError::UsageError => write!(f, "Usage: asm [path/to/input.sasm] path/to/output.s"),
        }
    }
}

impl error::Error for AsmError {}

fn inner_main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    let (input, output) = match &args[1..] {
        [output] => (None, output),
        [input, output] => (Some(input), output),
        _ => Err(AsmError::UsageError)?,
    };

    let input = if let Some(path) = input {
        fs::read_to_string(path)?
    } else {
        let mut result = String::new();
        io::stdin().read_to_string(&mut result)?;
        result
    };

    let assembly: Assembly = input.parse()?;
    let machine_code: Vec<u8> = assembly.into();
    fs::write(output, machine_code)?;
    Ok(())
}

fn main() {
    if let Err(err) = inner_main() {
        println!("{}", err);
    }
}
