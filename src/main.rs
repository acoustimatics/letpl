mod chunk;
mod compiler;
mod environment;
mod op;
mod parser;
mod scanner;
mod syntax;
mod token;
mod value;
mod vm;

use crate::compiler::compile;
use crate::value::Value;
use crate::vm::run;
use std::error::Error;
use std::io;
use std::io::Write;

fn main() {
    repl();
}

fn repl() -> ! {
    loop {
        print!("> ");
        let result = read_eval();
        print(result);
    }
}

fn read_eval() -> Result<Value, Box<dyn Error>> {
    let src = read()?;
    let chunk = compile(&src)?;
    let value = run(&chunk)?;
    Ok(value)
}

fn read() -> Result<String, Box<dyn Error>> {
    // Must flush or the prompt never gets printed.
    io::stdout().flush()?;
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn print(result: Result<Value, Box<dyn Error>>) {
    match result {
        Ok(value) => println!("{}", value),
        Err(e) => eprintln!("error: {}", e),
    }
}
