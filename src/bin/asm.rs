use std::env;
use std::error;
use std::fs;
use std::io::{self, prelude::*};
use std::process;

use thiserror::Error;

use stack_machine::{machine_code::Compile, models::Assembly};

struct Config<'a> {
    input: Option<&'a str>,
    output: &'a str,
}

#[derive(Error, Debug)]
enum UsageError {
    #[error("Not enough arguments, expected 1 or 2")]
    NotEnoughArguments,

    #[error("Too many arguments, expected 1 or 2")]
    TooManyArguments,
}

impl<'a> Config<'a> {
    /// Creates a new config from given command line arguments.
    /// 
    /// First argument is ignored, because it contains a path to the executable.
    fn new(args: &'a [String]) -> Result<Self, UsageError> {
        match &args[1..] {
            [output] => Ok(Config {
                input: None,
                output: output,
            }),
            [input, output] => Ok(Config {
                input: Some(input),
                output: output,
            }),
            [] => Err(UsageError::NotEnoughArguments),
            _ => Err(UsageError::TooManyArguments),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Usage error: {}", err);
        eprintln!("Usage: asm [path/to/input.sasm] path/to/output.s");
        process::exit(1);
    });

    if let Err(err) = try_main(&config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}

fn try_main(config: &Config) -> Result<(), Box<dyn error::Error>> {
    let input = if let Some(ref path) = config.input {
        fs::read_to_string(path)?
    } else {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf)?;
        String::from_utf8(buf)?
    };

    let assembly: Assembly = input.parse()?;
    let mut output = fs::File::create(&config.output)?;
    assembly.compile(&mut output)?;
    Ok(())
}
