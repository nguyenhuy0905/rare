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

    let regex_handle = parser.parse();
    println!("Regex string: {rstr}");
    regex_handle.nfa.print_states();
}
