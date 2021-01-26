use std::env;
use std::error;
use std::fs;
use std::io::{self, prelude::*};
use std::path;
use std::process;

use thiserror::Error;

use stack_machine::{executor::Machine, machine_code::{Compile, Decompile}, models::Assembly};

enum Config<'a> {
    Execute {
        input: Option<&'a path::Path>,
    },
    Compile {
        input: Option<&'a path::Path>,
        output: &'a path::Path,
    },
    Decompile {
        input: &'a path::Path,
        output: Option<&'a path::Path>,
    },
    Help,
}

#[derive(Error, Debug)]
#[error("{0}")]
struct UsageError(String);

impl<'a> Config<'a> {
    /// Creates a new config from given command line arguments.
    ///
    /// First argument is ignored, because it contains a path to the executable.
    fn new(args: &'a [String]) -> Result<Self, UsageError> {
        let flag = args.get(1).ok_or(UsageError("Flag not specified".into()))?;
        match flag.as_str() {
            "-c" | "--compile" => match &args[2..] {
                [output] => Ok(Config::Compile {
                    input: None,
                    output: path::Path::new(output),
                }),
                [input, output] => Ok(Config::Compile {
                    input: Some(path::Path::new(input)),
                    output: path::Path::new(output),
                }),
                x => {
                    let msg = format!("Expected 1 or 2 arguments after -c flag, got {}", x.len());
                    Err(UsageError(msg))
                }
            },
            "-d" | "--decompile" => match &args[2..] {
                [input] => Ok(Config::Decompile {
                    input: path::Path::new(input),
                    output: None,
                }),
                [input, output] => Ok(Config::Decompile {
                    input: path::Path::new(input),
                    output: Some(path::Path::new(output)),
                }),
                x => {
                    let msg = format!("Expected 1 or 2 arguments after -d flag, got {}", x.len());
                    Err(UsageError(msg))
                }
            },
            "-x" => match &args[2..] {
                [] => Ok(Config::Execute { input: None }),
                [input] => Ok(Config::Execute {
                    input: Some(path::Path::new(input)),
                }),
                x => {
                    let msg = format!("Expected 0 or 1 argument after -x flag, got {}", x.len());
                    Err(UsageError(msg))
                }
            },
            "-h" | "--help" => {
                if args.len() == 2 {
                    Ok(Config::Help)
                } else {
                    let msg = format!("Expected 0 arguments after -h flag, got {}", args.len() - 2);
                    Err(UsageError(msg))
                }
            }
            x => {
                let msg = format!(
                    "Expected one of '-x', '-d', '-h' or '-c' flags, got {} argument{}",
                    x.len(),
                    if x.len() == 1 { "" } else { "s" },
                );
                Err(UsageError(msg))
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Usage error: {}", err);
        eprintln!("Usage: {}", USAGE);
        process::exit(2);
    });

    if let Err(err) = try_main(&config) {
        eprintln!("Application error:\n{}", err);
        process::exit(1);
    }
}

type MyResult = Result<(), Box<dyn error::Error>>;

fn try_main(config: &Config) -> MyResult {
    match config {
        Config::Help => help(),
        Config::Execute { input } => execute(input)?,
        Config::Compile { input, output } => compile(input, output)?,
        Config::Decompile { input, output } => decompile(input, output)?,
    };
    Ok(())
}

fn help() -> () {
    println!("Usage:\n{}", USAGE);
    println!("Examples:\n{}", EXAMPLES);
}

fn execute(input: &Option<&path::Path>) -> MyResult {
    let machine_code = if let Some(path) = input {
        fs::read(path)?
    } else {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf)?;
        let assembly: Assembly = String::from_utf8(buf)?.parse()?;
        let mut machine_code = Vec::new();
        assembly.compile(&mut machine_code)?;
        machine_code
    };

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut machine = Machine::new(stdin.lock(), stdout.lock());
    machine.execute_program(&machine_code[..])?;
    Ok(())
}

fn compile(input: &Option<&path::Path>, output: &&path::Path) -> MyResult {
    let input = if let Some(ref path) = input {
        fs::read_to_string(path)?
    } else {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf)?;
        String::from_utf8(buf)?
    };

    let assembly: Assembly = input.parse()?;
    let mut output = fs::File::create(output)?;
    assembly.compile(&mut output)?;
    Ok(())
}

fn decompile(
    input: &&path::Path,
    output: &Option<&path::Path>,
) -> MyResult {
    let machine_code = fs::read(input)?;
    let assembly = Assembly::decompile(machine_code.as_slice())?.value;
    if let Some(path) = output {
        fs::write(path, assembly.to_string().as_bytes())?;
    } else {
        println!("{}", assembly);
    }
    Ok(())
}

const USAGE: &'static str = "\
smachine -c [path/to/input.sasm] path/to/output/s
smachine -d [path/to/input.s] path/to/output.sasm
smachine -x [path/to/input.s]
";

const EXAMPLES: &'static str = "\
Execute compiled binary 'a.s':
smachine -x a.s

Compile 'a.sasm' and execute:
smachine -c a.sasm a.s && smachine -x a.s
Can do 'smachine -x < a.sasm' in Bash (or other shells)

Compile a.sasm and write resulting binary to 'a.s':
smachine -c a.sasm a.s
Also try 'cat a.sasm | smachine -c a.s' in Bash (or other shells)

Decompile b.s and write resulting assembly to b.sasm
smachine -d b.s b.sasm
Can write to STDOUT instead: `smachine -d b.s | cat > b.sasm'
";
