# Rust awkward regex engine (RARE)

> [!NOTE]
> Work in progress.

- An overengineered regular expression matcher, written in the 2nd best low level
language out there.

## What does it do?

### TL;DR

- This engine uses the finite automaton method to match regular expressions. A
regular expression is compiled into a finite automaton, which is traversed when
matching a string.

### A little more words

- First, the program scans the regex string passed in and parse it into a infix
token list.
- Then, the infix token list goes through a postfix converter, which basically
is a Shunting Yard machine to convert the infix list into a postfix one.
  - Postfix list is simpler to parse because the parser only needs to worry
  about at most 2 states at a time.
- After that, the postfix list is passed to the parser, who converts the list
into a nondeterministic finite automaton (NFA).
  - The parser doesn't generate optimal NFAs though. The NFAs generated all
  have one single start state, and one single end state. This makes them very
  composable, but also generates quite a lot of empty states.
- Finally, given a string, the regex program traverses the resultant NFA.
If it lands at the final state, the string matches. Otherwise, while the string
isn't empty, it tries to match the same string, minus the first letter.
- If you use the CLI, this program highlights all parts of line that match the
regular expression. So, kinda like (a crippled version of) `grep`.
- The worst-case time complexity should be $O(mn)$.
  - In prior versions, it was actually $O(2^n)$. I benchmarked it by matching
  a pattern in the `clang`'s source code. It used to take 8 minutes. The version
  before 1.2.0 takes nearly 8 minutes to match. The newer ones take about 7 seconds.
    - Well, `grep` only takes 4 though. I mean, am I supposed to compete with `grep`?

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

- Here's a code example. This program reads from `stdin` and highlights the matching
parts in red:
  - Note, by "matching", I mean the longest strings that match the regex. You could
  say "b\*" matches every single character in a string of "a"s. But, the length of
  each match is, well, 0. So, it's not hightlighted in red.

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

    let in_handle = io::stdin().lock();
    for buf_result in in_handle.lines() {
        let buf = if let Ok(ret) = buf_result {
            ret
        } else {
            continue;
        };
        if let Some(match_substr) = rare.match_all(&buf) {
            let mut prev_last_idx = 0;
            for substr_range in match_substr.iter() {
                if let Some(out) = 
                    // a function I defined to help get the substring.
                    get_substring(&buf, substr_range.0, substr_range.1) {
                    print!("{}", &buf[prev_last_idx..substr_range.0]);
                    print!("\x1b[31m{out}\x1b[0m");
                    prev_last_idx = substr_range.1;
                }
                // println!("{}, {}", substr_range.0, substr_range.1);
            }
            println!("{}", &buf[prev_last_idx..]);
        }
    }

    Ok(())
}

```

And here is an example using the CLI:

![CLI example](https://github.com/user-attachments/assets/a6ff171a-1d0f-48f2-b20a-41ba716ac351)

## TODO

- Tidy up the code base. (Halfway there).
- Write usage docum
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

### History: this used to run in exponential time

- I think it used to run in $O(2^n)$ time complexity. This version is about
$O(mn)$ ($m$: length of string, $n$: length of regex).

- I am thinking of documenting the development process, which will better
describe what I mean by this.

### Performance consideration: how does it compare against grep?

- `grep` has its dark magic, in which, for longer strings, it actually runs faster.

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
