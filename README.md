# Rust awkward regex engine

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
- If you use the CLI, this program simply checks which line has a match to the
regular expression, then prints the entire line. That's it. Not even the highlight
you get when you `grep`.

## What does it support currently?

- Grouping expressions with ().
- Dot (.).
- Kleene's star \*.
- Or boolean |.
- Plus (+).
- Question mark (?).

## How to use

- Here's a code example:

```rust

let regex = "\\.(c(xx|pp)|h(xx|pp))";
let mut parser = match Parser::new(regex) {
    Ok(p) => p,
    // It won't panic because I know how to write regex.
    Err(msg) => panic!("{msg}"),
};
let regex = parser.parse();
// regex.nfa.print_states();
assert!(regex.is_match("before the string cccccc.cxx and after"));
assert!(!regex.is_match(".hcxx"));
assert!(!regex.is_match(".hcxx"));
assert!(regex.is_match(".cpp"));

```
![CLI example](https://github.com/user-attachments/assets/22b5dc13-9d71-4690-b8f9-46f16790f38f)


## TODO

- Add the remaining notations: yay i got it done.

- More graceful error handling:
  - There's a lot of panicking. Need to chop down probably half of those.

- Develop some more notations:
  - Hat (^). This one is simpler: just don't creep up a character when the
  first attempt fails.
  - Dollar sign ($). A simple way is to just keep running even if there's already
  a match, if that match doesn't reach the end of the string.
  - Range match (\[a-z\]). Basically dot but more limited.
  - Limiter ({a,b}). Copy-and-pasting the NFA right before it, a times. Then add
  (b - a) beams (OR). Each of these either go empty or go to the copy-pasted NFA.
    - I wouldn't deal with the syntax sugar of {a,} or {,b}. At least, until I got
    everything working.

- UTF-8?
  - Not until I have everything above finished.
