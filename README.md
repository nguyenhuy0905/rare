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

And here is an example using the CLI:

![CLI example](https://github.com/user-attachments/assets/200f1135-8c94-4964-8846-65d7e4bd9862)

## TODO

- Tidy up the code base.
- Develop some more notations:
  - Range match (\[a-z\]). Basically dot but more limited.
  - Limiter ({a,b}). Copy-and-pasting the NFA right before it, a times. Then add
  (b - a) beams (OR). Each of these either go empty or go to the copy-pasted NFA.
    - I wouldn't deal with the syntax sugar of {a,} or {,b}. At least, until I got
    everything working.

- UTF-8?
  - Not until I have everything above finished.

## Misc

### Performance consideration: how does it compare against grep?

- For short regex and kind-of short string like this, it is about as fast.
- However, `grep` has its dark magic, in which, for longer strings, it actually
runs faster.
- So, for long inputs, I don't expect this program to come any close to `grep`.
- This is a rare moment where it actually beats out grep:

![I did it boys](https://github.com/user-attachments/assets/1a2e2d4b-517f-4d56-91a3-03f557966ddf)

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
