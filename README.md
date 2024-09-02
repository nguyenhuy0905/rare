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
- (WIP) Finally, given a string, the regex program traverses the resultant NFA.
If it lands at the final state, the string matches.
