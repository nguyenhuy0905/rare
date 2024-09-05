# Rust awkward regex engine (RARE)

> [!NOTE]
> Work in progress.

- An overengineered regular expression matcher, written in the 2nd best low level
language out there.

## What does it do?

- First, the program scans the regex string passed in and parse it into a infix
token list.
- Then, the infix token list goes through a postfix converter, which basically
is a Shunting Yard machine to convert the infix list into a postfix one.
  - Postfix list is simpler to parse because the parser only needs to worry
  about at most 2 states at a time.
- After that, the postfix list is passed to the parser, who converts the list
into a nondeterministic finite automaton (NFA).
- Finally, given a string, the regex program traverses the resultant NFA.
If it lands at the final state, the string matches. Otherwise, while the string
isn't empty, it tries to match the same string, minus the first letter.
- If you use the CLI, this program highlights all parts of line that match the
regular expression.
- The worst-case time complexity should be $O(mn)$

## What does it support currently?

- Grouping expressions with ().
- Dot (.).
- Kleene's star \*.
- Or boolean |.
- Plus (+).
- Question mark (?).
- Hat (^).
- Dollar sign ($).

## How to use

- Here's a code example:

```rust
use std::{
    io::{self, BufRead},
    process::exit,
};

use rare::RARE;

// taken straight from this project's main function.
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

```

And here is an example using the CLI:

![CLI example](https://github.com/user-attachments/assets/a6ff171a-1d0f-48f2-b20a-41ba716ac351)

## TODO

- Tidy up the code base. (Halfway there).
- Write usage documentation.
- ~~Write a professional-looking blog post about this project~~.
- Develop some more notations:
  - Range match (\[a-z\]). Basically dot but more limited.
  - Limiter ({a,b}). Copy-and-pasting the NFA right before it, a times. Then add
  (b - a) beams (OR). Each of these either go empty or go to the copy-pasted NFA.
    - I wouldn't deal with the syntax sugar of {a,} or {,b}. At least, until I got
    everything working.

- UTF-8?
  - Not until I have everything above finished.

- ~~Rename to Rust Awesome Pattern Engine~~.

## Misc

### Performance consideration: how does it compare against grep?

- For short regex and kind-of short string like this, it is about as fast.
- However, `grep` has its dark magic, in which, for longer strings, it actually
runs faster.
- So, for long inputs, I don't expect this program to come any close to `grep`.
- These are some moments when it actually beats out grep:

![I did it boys](https://github.com/user-attachments/assets/1a2e2d4b-517f-4d56-91a3-03f557966ddf)
![And AGAIN!!!](https://github.com/user-attachments/assets/6ce017b6-2b04-4dc9-a8aa-3b4f8422f56f)


### Performance consideration: linked list vs vector

- Note: this isn't really benchmarked so it's the author yappin'.
- This program uses a LOT of stacks for parsing a regular expression. However,
it is assumed that almost every regular expression passed in is short. Given
that, the Big-O gain of a linked list for stack operations isn't really worth
it, especially when considering its tradeoff in cache locality.
- The only collection returned that uses a linked list is that of `match_all`,
since, if given a long string and a short regex (worst case, ".\*"), the
returned collection can be really long.
  - "But I need the random access!!!"
  - Convert it into a linked list then. Even when considering the conversion, that
  may still be more efficient than resizing too many times.

### What does it have over grep?

- Nothing. Maybe it's written in Rust?

### What does it have over Rust's Regex crate?

- Maybe something? I have never used that crate so I dunno.
