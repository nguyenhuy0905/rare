use super::token_type::TokenType;
/// Encapsulates scanner elements. The struct should be mutable, otherwise it's useless.
///
/// The scanner doesn't own the string. Hence, the input string's lifetime must be at least
/// the lifetime of the scanner.
pub struct Scanner<'a> {
    token_list: Vec<TokenType>,
    input: &'a str,
    curr_scan_fn: fn(&mut Scanner<'a>, char) -> TokenType,
    concat_next: bool,
}

impl<'a> Scanner<'a> {
    /// Creates a new Scanner.
    ///
    /// The variable containing this scanner should be mutable.
    /// * `input_str`: The string to be scanned.
    pub fn new(input_str: &'a str) -> Scanner<'a> {
        Scanner {
            token_list: Vec::new(),
            input: input_str,
            curr_scan_fn: Scanner::scan_char,
            concat_next: false,
        }
    }

    /// Scans the input string that the scanner holds.
    ///
    /// After this function is called, the scanner should be made immutable.
    pub fn scan(&mut self) {
        for input_char in self.input.chars() {
            let ret_token = (self.curr_scan_fn)(self, input_char);
            let need_concat = ret_token.need_concat_next();
            match ret_token {
                TokenType::Escape => {}
                TokenType::Character(_) | TokenType::Dot | TokenType::LParen => {
                    if self.concat_next {
                        self.token_list.push(TokenType::Concat);
                    }
                    self.token_list.push(ret_token);
                }
                _ => self.token_list.push(ret_token),
            }
            self.concat_next = need_concat;
        }
    }

    pub(in crate::lexer) fn scan_char(&mut self, input_char: char) -> TokenType {
        match input_char {
            '.' => TokenType::Dot,
            '*' => TokenType::Star,
            '|' => TokenType::Beam,
            '+' => TokenType::Plus,
            '?' => TokenType::QuestionMark,
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            '\\' => {
                self.curr_scan_fn = Scanner::scan_escape;
                TokenType::Escape
            }
            _ => TokenType::Character(input_char),
        }
    }

    pub(in crate::lexer) fn scan_escape(&mut self, input_char: char) -> TokenType {
        self.curr_scan_fn = Scanner::scan_char;
        TokenType::Character(input_char)
    }

    /// Reverses the token list, useful for converting into a postfix representation.
    pub(in crate) fn reverse_token_list(&mut self) {
        self.token_list.reverse();
    }

    pub fn get_vec(&self) -> &Vec<TokenType> {
        &self.token_list
    }
    
    /// Takes the token list from this scanner and incinerate the scanner.
    pub fn move_vec(self) -> Vec<TokenType> {
        self.token_list
    }

    pub(in crate) fn get_mut_vec(&mut self) -> &mut Vec<TokenType> {
        &mut self.token_list
    }

    /// Prints the entire token list of this scanner
    pub fn print_tokens(&self) {
        for tok in self.token_list.iter() {
            println!("{}", tok);
        }
    }
}

mod test;
