mod chunk;
mod compiler;
mod environment;
mod op;
mod parser;
mod procedure;
mod scanner;
mod syntax;
mod token;
mod value;
mod vm;

use crate::value::Value;
use std::error::Error;
use std::io::Write;
use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: letpl [script]");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        repl();
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

fn read_file_eval(path: &str) -> Result<Value, Box<dyn Error>> {
    let src = fs::read_to_string(path)?;
    let value = eval(&src)?;
    Ok(value)
}

fn read_eval() -> Result<Value, Box<dyn Error>> {
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

fn eval(src: &str) -> Result<Value, Box<dyn Error>> {
    let chunk = compiler::compile(src)?;
    let value = vm::run(&chunk)?;
    Ok(value)
}

fn print(result: Result<Value, Box<dyn Error>>) {
    match result {
        Ok(value) => println!("{}", value),
        Err(e) => eprintln!("error: {}", e),
    }
}
