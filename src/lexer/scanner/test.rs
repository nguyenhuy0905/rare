#[cfg(test)]
use super::*;
#[test]
fn simple_scan_test() {
    // println!("{}", TokenType::Character('c').discriminant());
    let mut test_scanner = Scanner::new("(abc)\\..*");
    test_scanner.scan();
    // I can test private methods?
    let res_vec: &Vec<TokenType> = &test_scanner.token_list;
    let outputs: [TokenType; 12] = [
        TokenType::LParen,
        TokenType::Character('a'),
        TokenType::Concat,
        TokenType::Character('b'),
        TokenType::Concat,
        TokenType::Character('c'),
        TokenType::RParen,
        TokenType::Concat,
        TokenType::Character('.'),
        TokenType::Concat,
        TokenType::Dot,
        TokenType::Star,
    ];

    let mut idx = 0;

    while idx < res_vec.len() {
        assert_eq!(res_vec[idx], outputs[idx]);
        idx += 1;
    }

    for out in outputs.iter() {
        println!("{}", out);
    }
}
