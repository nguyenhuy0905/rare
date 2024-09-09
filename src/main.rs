use std::{
    io::{self, BufRead, Write},
    process::exit,
};

use rare::RARE;

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

    // lock version
    // let stdin = io::stdin();
    // while let Some(Ok(input)) = stdin.lock().lines().next() {
    //     if let Some(match_substr) = rare.match_all(&input) {
    //         let mut prev_last_idx = 0;
    //         for substr_range in match_substr.iter() {
    //             // println!("{}, {}", substr_range.0, substr_range.1);
    //             if let Some(out) = get_substring(&input, substr_range.0, substr_range.1) {
    //                 // print!("{out}");
    //                 print!("{}", &input[prev_last_idx..substr_range.0]);
    //                 print!("\x1b[31m{out}\x1b[0m");
    //                 prev_last_idx = substr_range.1;
    //             }
    //         }
    //         println!("\x1b[0m{}", &input[prev_last_idx..]);
    //     }
    // }

    // nonlock version
    // let in_handle = io::stdin().lock();
    // for buf_result in in_handle.lines() {
    //     let buf = if let Ok(ret) = buf_result {
    //         ret
    //     } else {
    //         continue;
    //     };
    //     if let Some(match_substr) = rare.match_all(&buf) {
    //         let mut prev_last_idx = 0;
    //         for substr_range in match_substr.iter() {
    //             if let Some(out) = get_substring(&buf, substr_range.0, substr_range.1) {
    //                 print!("{}", &buf[prev_last_idx..substr_range.0]);
    //                 print!("\x1b[31m{out}\x1b[0m");
    //                 prev_last_idx = substr_range.1;
    //             }
    //             // println!("{}, {}", substr_range.0, substr_range.1);
    //         }
    //         println!("{}", &buf[prev_last_idx..]);
    //     }
    // }

    // let in_handle = io::stdin().lock();
    // let mut out_lock = io::stdout().lock();
    // let mut buf_vec: Vec<(usize, usize)> = Vec::new();
    // for buf_result in in_handle.lines() {
    //     let buf = if let Ok(ret) = buf_result {
    //         ret
    //     } else {
    //         continue;
    //     };
    //     rare.write_match_all(&buf, &mut buf_vec);
    //     let mut prev_last_idx = 0;
    //     if buf_vec.is_empty() {
    //         continue;
    //     }
    //     for substr_range in buf_vec.drain(..) {
    //         if let Some(out) = get_substring(&buf, substr_range.0, substr_range.1) {
    //             // print!("{}", &buf[prev_last_idx..substr_range.0]);
    //             // print!("\x1b[31m{out}\x1b[0m");
    //             write!(out_lock, "{}\x1b[31m{out}\x1b[0m", &buf[prev_last_idx..substr_range.0])?;
    //             prev_last_idx = substr_range.1;
    //         }
    //         // println!("{}, {}", substr_range.0, substr_range.1);
    //     }
    //     println!("{}", &buf[prev_last_idx..]);
    // }

    // is match or not only. No highlight.
    // scales way better. Although still smoked by grep.
    let in_handle = io::stdin().lock();
    for buf_result in in_handle.lines() {
        let buf = if let Ok(ret) = buf_result {
            ret
        } else {
            continue;
        };
        if rare.is_match(&buf) {
            println!("{buf}");
        }
    }

    Ok(())
}

#[allow(dead_code)]
#[inline]
fn get_substring(input: &str, beg: usize, end: usize) -> Option<&str> {
    if beg >= input.len() {
        None
    } else if end >= input.len() {
        Some(&input[beg..])
    } else {
        Some(&input[beg..end])
    }
}
