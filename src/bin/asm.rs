use fs::File;
use stack_machine::{machine_code::Compile, models::Assembly};
use std::{
    env, error, fs,
    io::{self, prelude::*},
};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    match &args[1..] {
        [output] => run(None, output)?,
        [input, output] => run(Some(input), output)?,
        _ => usage(),
    };
    Ok(())
}

fn run(input: Option<&str>, output: &str) -> Result<(), Box<dyn error::Error>> {
    let input = if let Some(path) = input {
        fs::read_to_string(path)?
    } else {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf)?;
        String::from_utf8(buf)?
    };

    let assembly: Assembly = input.parse()?;
    let mut output = File::create(output)?;
    assembly.compile(&mut output)?;
    Ok(())
}

fn usage() {
    println!("Usage: asm [path/to/input.sasm] path/to/output.s");
}
