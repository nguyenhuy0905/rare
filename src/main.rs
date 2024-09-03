use std::{io::{self, BufRead}, process::exit};

use parser::Parser;

mod lexer;
mod postfix_converter;
mod parser;
mod regex;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Expected 1 argument")
    }

    let mut parser = match Parser::new(&args[1]) {
        Ok(p) => p,
        Err(msg) => {
            eprintln!("Error: {msg}");
            exit(1);
        }
    };

    let regex = parser.parse();

    let stdin = io::stdin();
    while let Some(Ok(input)) = stdin.lock().lines().next() {
        if regex.is_match(&input) {
            println!("{input}");
        }
    }

    Ok(())
}
