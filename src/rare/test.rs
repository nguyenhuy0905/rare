#[cfg(test)]
use crate::parser::Parser;

#[test]
fn test_regex_matcher_simple() {
    let regex = "ab+|c+";
    println!("Regex: {regex}");

    let mut parser = match Parser::new(regex) {
        Ok(p) => p,
        Err(msg) => panic!("{msg}"),
    };

    let ret_ex = match parser.parse() {
        Ok(r) => r,
        Err(msg) => panic!("{msg}"),
    };
    ret_ex.nfa.print_states();
    assert!(ret_ex.is_match("abbbbbb"));
    assert!(ret_ex.is_match("bccccccccc"));
    // because 'a' still matches, what comes after doesn't matter.
    assert!(!ret_ex.is_match("adbbbbb"));
    // note: since part of the string still matches the entire regex.
    assert!(ret_ex.is_match("abc"));
}

#[test]
fn test_regex_matcher_less_simple() {
    // matcher for the bestest low level language
    let regex = "\\.(c(xx|pp)|h(xx|pp))+";
    let mut parser = match Parser::new(regex) {
        Ok(p) => p,
        Err(msg) => panic!("{msg}"),
    };
    let regex = match parser.parse() {
        Ok(r) => r,
        Err(msg) => panic!("{msg}"),
    };
    regex.nfa.print_states();
    assert!(regex.is_match("whatever before isn't important cccccc.cxx and whatever behind here also isn't really important"));
    assert!(regex.is_match(".hcxx.cpp"));
    assert!(!regex.is_match(".hcxx"));
    assert!(regex.is_match(".cpp"));
}

#[test]
fn matcher_stress_test() {
    let regex = "a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa";
    let match_str = "aaaaaaaaaaaaaaaaaaaa";

    let mut parser = match Parser::new(regex) {
        Ok(p) => p,
        Err(msg) => panic!("{msg}"),
    };
    let regex = match parser.parse() {
        Ok(r) => r,
        Err(msg) => panic!("{msg}"),
    };
    regex.nfa.print_states();
    assert!(regex.is_match(match_str));
    if let Some(pairs) = regex.match_all(match_str) {
        for pair in pairs.iter() {
            println!("({}, {})", pair.0, pair.1);
        }
    } else {
        println!("No match!!!!");
        panic!();
    }
}

#[test]
fn matcher_sus_test() {
    let regex: &str = "^(Never)+";
    let match_str: &str = "Neverrrrrrr";

    let mut parser = match Parser::new(regex) {
        Ok(p) => p,
        Err(msg) => panic!("{msg}"),
    };
    let regex = match parser.parse() {
        Ok(r) => r,
        Err(msg) => panic!("{msg}"),
    };
    regex.nfa.print_states();
    // assert!(regex.is_match(match_str));
    if let Some(pairs) = regex.match_all(match_str) {
        for pair in pairs.iter() {
            println!("({}, {})", pair.0, pair.1);
        }
    } else {
        println!("No match!!!!");
        panic!();
    }
}
