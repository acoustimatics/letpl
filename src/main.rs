mod environment;
mod interpreter;
mod parser;
mod scanner;
mod syntax;
mod token;
mod value;

use std::io;
use std::io::Write;

fn main() {
    repl();
}

fn repl() {
    loop {
        print!("> ");
        // Must flush or the prompt never gets printed.
        if let Err(e) = io::stdout().flush() {
            eprintln!("{}", e);
            continue;
        }
        let mut buffer = String::new();
        if let Err(e) = io::stdin().read_line(&mut buffer) {
            eprintln!("{}", e);
            continue;
        }
        let result = interpreter::run(&buffer);
        match result {
            Ok(val) => println!("{}", val),
            Err(e) => eprintln!("error: {}", e),
        }
    }
}
