use std::env;
use std::error;
use std::fs;

use stack_machine::{machine_code::Decompile, models::Assembly};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    match &args[1..] {
        [input] => run(input, None)?,
        [input, output] => run(input, Some(output))?,
        _ => usage(),
    }
    Ok(())
}

fn usage() {
    println!("Usage: disasm path/to/input.s [path/to/output.sasm]");
}

fn run(input: &str, output: Option<&str>) -> Result<(), Box<dyn error::Error>> {
    let machine_code = fs::read(input)?;
    let assembly = Assembly::decompile(machine_code.as_slice())?.value;
    if let Some(path) = output {
        fs::write(path, assembly.to_string().as_bytes())?;
    } else {
        println!("{}", assembly);
    }
    Ok(())
}
