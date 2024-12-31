mod environment;
mod expr;
mod interpreter;
mod lexer;
mod parser;
mod stmt;

use interpreter::*;
use lexer::*;
use parser::*;
use std::env::args;
use std::fs::read_to_string;
use std::io::{stdin, stdout, Write};
use std::process::exit;

fn run_file(path: &String) -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    let file_content =
        read_to_string(path).map_err(|err| format!("Failed to read file to string: {}", err))?;
    run(&mut interpreter, file_content)
}

fn run_prompt() -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    loop {
        print!("> ");
        stdout()
            .flush()
            .map_err(|err| format!("Flush error <lol> : {}", err))?;

        let mut prompt = String::new();
        stdin()
            .read_line(&mut prompt)
            .map_err(|err| format!("Failed to read line: {}", err))?;

        let prompt = prompt.trim();

        match run(&mut interpreter, prompt.to_string()) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}

fn run(interpreter: &mut Interpreter, source: String) -> Result<(), String> {
    let mut scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;
    interpreter.interpret(statements)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = args().collect();
    let run_result = match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: kadom [script]");
            exit(64);
        }
    };

    match run_result {
        Ok(_) => (),
        Err(msg) => {
            println!("{}", msg);
            exit(1);
        }
    }
}
