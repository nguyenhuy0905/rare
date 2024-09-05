#[cfg(test)]
use crate::postfix_converter::*;

#[test]
fn simple_postfix_test() {
    let mut test_scanner = Scanner::new("ab*|bc*");
    test_scanner.scan();
    test_scanner.print_tokens();
    println!();

    let test_pfix_conv = PostfixConverter::from_scanner(test_scanner);
    let test_pfix_conv = test_pfix_conv.convert().expect("It didn't work");
    test_pfix_conv.print_postfix_stack();
}

#[test]
fn simple_postfix_test_2() {
    let mut test_scanner = Scanner::new("\\.c(c|h)");
    test_scanner.scan();
    test_scanner.print_tokens();
    println!();

    let test_pfix_conv = PostfixConverter::from_scanner(test_scanner);
    let test_pfix_conv = test_pfix_conv.convert().expect("It didn't work");
    test_pfix_conv.print_postfix_stack();
}

#[test]
fn complex_postfix_test() {
    let mut test_scanner = Scanner::new("z(abc?|c)+\\.?.*");
    test_scanner.scan();

    let test_pfix_conv = PostfixConverter::from_scanner(test_scanner);
    let test_pfix_conv = test_pfix_conv.convert().expect("It didn't work");
    test_pfix_conv.print_postfix_stack();

    let test_vec: Vec<TokenType> = vec![
        TokenType::Character('z'),
        TokenType::Character('a'),
        TokenType::Character('b'),
        TokenType::Concat, // Concat a and b
        TokenType::Character('c'),
        TokenType::QuestionMark,
        TokenType::Concat, // Concat ab and c?
        TokenType::Character('c'),
        TokenType::Beam,   // either abc? or c
        TokenType::Plus,   // one or more of (abc?|c)
        TokenType::Concat, // concat z with (abc?|c)+
        TokenType::Character('.'),
        TokenType::QuestionMark,
        TokenType::Concat, // concat z(abc?|c)+ with \.?
        TokenType::Dot,
        TokenType::Star,
        TokenType::Concat, // concat z(abc?|c)+\.? with .*
    ];

    assert!(
        test_pfix_conv
            .postfix_token_list
            .iter()
            .zip(test_vec.iter())
            .filter(|pair| pair.0.token_type == *pair.1)
            .count()
            == test_vec.len()
    )
}
