mod lexer;

use lexer::*;
use std::env::args;
use std::fs::read_to_string;
use std::io::{stdin, stdout, Write};
use std::process::exit;

fn run_file(path: &String) {
    let file_content = read_to_string(path).expect("Oh no");
    let _ = run(file_content);
}

fn run_prompt() {
    loop {
        print!("> ");
        stdout().flush().expect("Oh no");

        let mut prompt = String::new();
        stdin().read_line(&mut prompt).expect("Oh no");

        let prompt = prompt.trim();

        let _ = run(prompt.to_string());
    }
}

fn run(source: String) -> Result<(), ()> {
    let mut scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens().map_err(|e| println!("{}", e))?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: kadom [script]");
            exit(64);
        }
    }
}
