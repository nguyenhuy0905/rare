use super::token_type::{Token, TokenType};
/// A scanner converts a raw string into an infix list of tokens. At the moment, the scanner, and
/// hence all other components of this regular expression engine, only works on ASCII characters.
///
/// * `token_list`: the resultant infix token list. Only valid after `Scanner::scan` is called.
/// * `input`: a reference to the input string. Since this is only a reference, the lifetime of the
///            input string must be at least that of the Scanner.
/// * `curr_scan_fn`: the function called by the scanner for each character it reads.
/// * `concat_next`: whether a concatenation notation may be inserted when scanning the next
///                  character.
pub(crate) struct Scanner<'a> {
    token_list: Vec<Token>,
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
    /// The scanner is expected to be consumed by the postfix converter after this step.
    pub fn scan(&mut self) -> Result<(), String> {
        for (idx, input_char) in self.input.chars().enumerate() {
            let ret_token = (self.curr_scan_fn)(self, input_char);
            let mut need_concat = ret_token.need_concat_next();
            match ret_token {
                TokenType::Escape => {
                    // escape isn't really a character, so gracefully set need_concat_next back to
                    // that of the last character.
                    need_concat = self.concat_next;
                }
                // TokenType::Dollar will be evaluated by the postfix converter.
                TokenType::Character(_)
                | TokenType::Dot
                | TokenType::LParen
                | TokenType::Dollar => self.push_need_concat(idx, ret_token),
                TokenType::Hat => self.handle_push_hat(idx)?,
                TokenType::QuestionMark | TokenType::Star | TokenType::Plus => {
                    self.handle_push_single_quantifier(idx, ret_token)?
                }
                _ => self.token_list.push(Token::new(idx, ret_token)),
            }
            self.concat_next = need_concat;
        }

        Ok(())
    }

    /// Scans the input character.
    /// If the input character is escape ('\\'), the next time the scanner calls its scan function,
    /// it calls `scan_escape`.
    ///
    /// * `input_char`: the input character.
    /// * Return: the token type detected.
    fn scan_char(&mut self, input_char: char) -> TokenType {
        match input_char {
            '.' => TokenType::Dot,
            '*' => TokenType::Star,
            '|' => TokenType::Beam,
            '+' => TokenType::Plus,
            '^' => TokenType::Hat,
            '$' => TokenType::Dollar,
            '?' => TokenType::QuestionMark,
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            '\\' => {
                self.curr_scan_fn = Scanner::scan_escape;
                TokenType::Escape
            }
            _ => TokenType::Character(input_char),
            // the user cannot input the concat token.
        }
    }

    /// The scan function called if the last character scanned is an escape.
    /// After this function is called, the scanner's next scan function is `scan_char`.
    ///
    /// * `input_char`:
    fn scan_escape(&mut self, input_char: char) -> TokenType {
        self.curr_scan_fn = Scanner::scan_char;
        TokenType::Character(input_char)
    }

    /// Reverses the token list, or in other words, convert the token list held by this scanner
    /// into a stack.
    pub(crate) fn reverse_token_list(&mut self) {
        self.token_list.reverse();
    }

    /// Takes the token list from this scanner and incinerate the scanner.
    pub fn move_vec(self) -> Vec<Token> {
        self.token_list
    }

    #[allow(dead_code)]
    /// Prints the entire token list of this scanner. Only useful for debugging.
    pub fn print_tokens(&self) {
        for tok in self.token_list.iter() {
            println!("{}", tok.token_type);
        }
    }

    /// Pushes the characters/symbols who may need concatenation with the preceding symbol.
    ///
    /// * `idx`: the string index.
    /// * `token`: the token type to push on.
    fn push_need_concat(&mut self, idx: usize, token: TokenType) {
        if self.concat_next {
            self.token_list.push(Token::new(idx, TokenType::Concat));
        }
        self.token_list.push(Token::new(idx, token));
    }

    /// Handles pushing hat (^) token onto the stack.
    ///
    /// * `idx`:
    fn handle_push_hat(&mut self, idx: usize) -> Result<(), String> {
        match self.token_list.last() {
            None => self.push_need_concat(idx, TokenType::Hat),
            Some(tok) => {
                if !(tok.token_type == TokenType::LParen || tok.token_type == TokenType::Beam) {
                    return Err(format!(
                        "Character {0} at {1}: {0} not at beginning of statement",
                        TokenType::Hat,
                        idx
                    ));
                }
                self.push_need_concat(idx, TokenType::Hat);
            }
        }
        Ok(())
    }

    /// Handles pushing star (*), plus (+) and question mark (?) onto the stack.
    /// These are single-character symbols, hence the name.
    ///
    /// * `idx`: the string index
    /// * `token`: the token type
    fn handle_push_single_quantifier(
        &mut self,
        idx: usize,
        token: TokenType,
    ) -> Result<(), String> {
        match self.token_list.last() {
            None => {
                return Err(format!(
                    "Character {0} at {1}: {0} at beginning of statement",
                    token, idx
                ))
            }

            Some(tok) => {
                if tok.token_type == TokenType::LParen || tok.token_type == TokenType::Beam {
                    return Err(format!(
                        "Character {0} at {1}: {0} at beginning of statement",
                        token, idx
                    ));
                }
                self.token_list.push(Token::new(idx, token));
            }
        }
        Ok(())
    }
}

mod test;
