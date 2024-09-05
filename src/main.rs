use std::{
    io::{self, BufRead},
    process::exit,
};

use rare::rare::RARE;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Expected 1 argument")
    }

    let rare = match RARE::new(&args[1]) {
        Ok(r) => r,
        Err(msg) => {
            println!("{msg}");
            exit(1);
        }
    };

    let stdin = io::stdin();
    while let Some(Ok(input)) = stdin.lock().lines().next() {
        if let Some(match_substr) = rare.match_all_index(&input) {
            let mut prev_last_idx = 0;
            for substr_range in &match_substr {
                print!(
                    "{0}\x1b[31m\x1b[1m{1}\x1b[0m",
                    &input[prev_last_idx..substr_range.0],
                    &input[substr_range.0..substr_range.1]
                );
                prev_last_idx = substr_range.1;
                // println!("{0}, {1}", substr_range.0, substr_range.1);
            }
            print!("{0}", &input[prev_last_idx..]);
            println!();
        }
    }

    Ok(())
}
