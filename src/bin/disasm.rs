use std::convert::TryFrom;
use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;

use stack_machine::models::Assembly;

#[derive(Debug)]
enum DisasmError {
    IOError(io::Error),
    StringError(String),
    UsageError,
}

impl fmt::Display for DisasmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DisasmError::IOError(x) => x.fmt(f),
            DisasmError::StringError(x) => x.fmt(f),
            DisasmError::UsageError => {
                write!(f, "Usage: disasm path/to/input.s [path/to/output.sasm]")
            }
        }
    }
}

impl error::Error for DisasmError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DisasmError::IOError(x) => Some(x),
            _ => None,
        }
    }
}

impl From<io::Error> for DisasmError {
    fn from(err: io::Error) -> Self {
        DisasmError::IOError(err)
    }
}

impl From<String> for DisasmError {
    fn from(msg: String) -> Self {
        DisasmError::StringError(msg)
    }
}

fn main_inner() -> Result<(), DisasmError> {
    let args: Vec<String> = env::args().collect();
    let (input, output) = match &args[1..] {
        [input] => (input, None),
        [input, output] => (input, Some(output)),
        _ => Err(DisasmError::UsageError)?
    };

    let machine_code = fs::read(input)?;
    let assembly = Assembly::try_from(machine_code.as_slice())?;
    let assembly = assembly.to_string();

    if let Some(path) = output {
        fs::write(path, assembly)?;
    } else {
        io::stdout().write_all(assembly.as_bytes())?;
    };
    Ok(())
}

fn main() {
    if let Err(err) = main_inner() {
        println!("{}", err);
    };
}
