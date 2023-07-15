mod compiler;
mod parser;
mod runtime;

use std::error::Error;
use std::io::Write;
use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => println!("Usage: letpl [script]"),
    }
}

fn run_file(path: &str) {
    let result = read_file_eval(path);
    print(result);
}

fn repl() -> ! {
    loop {
        print!("> ");
        let result = read_eval();
        print(result);
    }
}

fn read_file_eval(path: &str) -> Result<runtime::Value, Box<dyn Error>> {
    let src = fs::read_to_string(path)?;
    let value = eval(&src)?;
    Ok(value)
}

fn read_eval() -> Result<runtime::Value, Box<dyn Error>> {
    let src = read()?;
    let value = eval(&src)?;
    Ok(value)
}

fn read() -> Result<String, Box<dyn Error>> {
    // Must flush or the prompt never gets printed.
    io::stdout().flush()?;
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn eval(src: &str) -> Result<runtime::Value, Box<dyn Error>> {
    let program = parser::parse(src)?;
    let compiled_program = compiler::compile(&program)?;
    let value = runtime::run(&compiled_program)?;
    Ok(value)
}

fn print(result: Result<runtime::Value, Box<dyn Error>>) {
    match result {
        Ok(value) => println!("{}", value),
        Err(e) => eprintln!("error: {}", e),
    }
}
