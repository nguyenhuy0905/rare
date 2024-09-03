#[cfg(test)]
use crate::parser::Parser;
use super::*;

#[test]
fn test_regex_matcher_simple() {
    let regex = "ab*";
    println!("Regex: {regex}");

    let mut parser = match Parser::new(regex) {
        Ok(p) => p,
        Err(msg) => panic!("{msg}"),   
    };

    let ret_ex = parser.parse();
    ret_ex.nfa.print_states();
    // assert!(ret_ex.is_match("ab"));
}
