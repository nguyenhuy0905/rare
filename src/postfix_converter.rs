use crate::lexer::{scanner::Scanner, token_type::TokenType};
use std::vec::Vec;

pub struct PostfixConverter<'a> {
    infix_token_stack: Vec<TokenType>,
    postfix_token_list: Vec<TokenType>,
    symbol_stack: Vec<TokenType>,
    last_sym: &'a TokenType,
    done: bool,
}

impl<'a> PostfixConverter<'a> {
    /// Constructs a postfix converter from the scanner provided.
    ///
    /// * `scanner`:
    pub fn from_scanner(mut scanner: Scanner) -> PostfixConverter {
        scanner.reverse_token_list();
        let mut ret = PostfixConverter {
            infix_token_stack: scanner.move_vec(),
            postfix_token_list: Vec::new(),
            symbol_stack: Vec::new(),
            last_sym: &TokenType::Empty,
            done: false,
        };

        ret.postfix_token_list.reserve(ret.infix_token_stack.len());
        ret
    }

    /// Converts the stored infix token list into a postfix one.
    ///
    /// After this function, call get_postfix_vec to retrieve the postfix vector.
    pub fn convert(mut self) -> Result<PostfixConverter<'a>, String> {
        while let Some(token) = self.infix_token_stack.pop() {
            if !token.is_symbol() {
                self.postfix_token_list.push(token);
            } else if let Err(estr) = self.push_symbol(token) {
                return Err(estr);
            }
        }

        while let Some(token) = self.symbol_stack.pop() {
            self.postfix_token_list.push(token);
        }

        self.done = true;
        Ok(self)
    }

    /// Returns the postfix vector stored inside this converter.
    /// After this call, the postfix converter is burnt to a crisp.
    ///
    /// Should only be called after convert. Otherwise, the return value isn't useful.
    pub(crate) fn move_postfix_vec(self) -> Vec<TokenType> {
        self.postfix_token_list
    }

    pub fn print_postfix_stack(&self) {
        for token in self.postfix_token_list.iter() {
            println!("{}", token);
        }
    }

    fn push_symbol(&mut self, token: TokenType) -> Result<(), String> {
        // TODO: generalize parts of this operation, using some sort of precedence mechanism
        match token {
            TokenType::LParen => {
                self.symbol_stack.push(token);
            }
            TokenType::RParen => {
                while let Some(pop_token) = self.symbol_stack.pop() {
                    if pop_token == TokenType::LParen {
                        return Ok(());
                    }
                    self.postfix_token_list.push(pop_token);
                }
                self.print_postfix_stack();
                return Err("Missing a (".to_owned());
            }
            TokenType::Concat => match self.symbol_stack.last() {
                Some(TokenType::Concat) => {
                    self.postfix_token_list.push(TokenType::Concat);
                }
                None | Some(_) => self.symbol_stack.push(token.clone()),
            },
            TokenType::Beam => match self.symbol_stack.last() {
                None | Some(TokenType::LParen) => {
                    self.symbol_stack.push(token);
                }
                Some(_) => {
                    self.postfix_token_list.push(
                        self.symbol_stack
                            .pop()
                            .expect("Trust me this can never go wrong"),
                    );
                    self.symbol_stack.push(token);
                }
            },
            _ => {
                self.postfix_token_list.push(token);
            }
        }

        Ok(())
    }
}

mod test;
