mod lexer;
use lexer::scanner::Scanner;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Expected 1 argument")
    }

    let mut scanner: Scanner = Scanner::new(&args[1]);
    scanner.scan();
    let scanner = scanner;

    scanner.print_tokens();
}
