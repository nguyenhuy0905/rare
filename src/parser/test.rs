#[cfg(test)]
use crate::parser::Parser;

#[test]
fn simple_parse_test() {
    let rstr = "(ab|b*)*|c";
    let mut parser = match Parser::new(rstr) {
        Err(msg) => {
            panic!("{msg}");
        }
        Ok(ret) => ret,
    };

    let regex_handle = match parser.parse() {
        Ok(r) => r,
        Err(msg) => panic!("{msg}"),
    };
    println!("Regex string: {rstr}");
    regex_handle.nfa.print_states();
}
