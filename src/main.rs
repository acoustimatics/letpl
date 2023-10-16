mod compiler;
mod name_analysis;
mod parser;
mod runtime;
mod symbol_table;
mod type_checking;
mod types;

use std::error::Error;
use std::io::Write;
use std::{env, fs, io};

use runtime::Value;
use types::LetType;

type EvalResult = Result<(Value, LetType), Box<dyn Error>>;

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

fn read_file_eval(path: &str) -> EvalResult {
    let src = fs::read_to_string(path)?;
    let t = eval(&src)?;
    Ok(t)
}

fn read_eval() -> EvalResult {
    let src = read()?;
    let t = eval(&src)?;
    Ok(t)
}

fn read() -> Result<String, Box<dyn Error>> {
    // Must flush or the prompt never gets printed.
    io::stdout().flush()?;
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn eval(src: &str) -> EvalResult {
    let program = parser::parse(src)?;
    let program_type = type_checking::let_type_of(&program)?;
    let nameless_program = name_analysis::resolve_names(&program)?;
    let compiled_program = compiler::compile(&nameless_program)?;
    let value = runtime::run(&compiled_program)?;
    Ok((value, program_type))
}

fn print(result: EvalResult) {
    match result {
        Ok((value, program_type)) => {
            println!("{}", value);
            println!("{}", program_type);
        }
        Err(e) => eprintln!("error: {}", e),
    }
}
