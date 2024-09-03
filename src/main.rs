use std::{io::{self, Read}, process::exit};

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

    let mut stdin = io::stdin();
    let mut input = String::new();

    stdin.read_to_string(&mut input)?;

    let mut parser = match Parser::new(&args[1]) {
        Err(msg) => {
            println!("Invalid regex: {msg}");
            exit(1);
        },
        Ok(ret) => ret,
    };

    let regex = parser.parse();
    if !regex.is_match(&input) {
        println!("Didn't match");
        exit(1);
    }
    
    println!("Match");

    Ok(())
}
