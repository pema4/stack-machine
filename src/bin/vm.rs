use stack_machine::executor::Machine;
use std::env;
use std::error;
use std::fs;
use std::io;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    match &args[1..] {
        [program] => run(program)?,
        _ => usage(),
    };
    Ok(())
}

fn run(program: &str) -> Result<(), Box<dyn error::Error>> {
    let program = fs::read(program)?;
    let input = io::stdin();
    let output = io::stdout();
    Machine::new(input.lock(), output.lock()).execute_program(&program[..])?;
    println!("Program terminated successfully!");
    Ok(())
}

fn usage() {
    println!("Usage: vm path/to/program.s");
}
