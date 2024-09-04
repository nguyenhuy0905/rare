use crate::lexer::{scanner::Scanner, token_type::TokenType};
use std::vec::Vec;

/// Converts an infix token stack into a postfix one. This struct assumes the infix token array is
/// provided by struct `Scanner`.
///
/// * `infix_token_stack`: basically the infix token list from `Scanner` but reversed.
/// * `postfix_token_list`: the resultant token list. Remains invalid until
///                         `PostfixConverter::convert` is called.
/// * `symbol_stack`: the symbol stack, used for temporarily holding symbols.
/// * `done`: whether the conversion is finished.
pub(crate) struct PostfixConverter {
    infix_token_stack: Vec<TokenType>,
    postfix_token_list: Vec<TokenType>,
    symbol_stack: Vec<TokenType>,
    done: bool,
}

impl PostfixConverter {
    /// Constructs a postfix converter from the scanner provided.
    ///
    /// * `scanner`:
    pub fn from_scanner(mut scanner: Scanner) -> PostfixConverter {
        scanner.reverse_token_list();
        let mut ret = PostfixConverter {
            infix_token_stack: scanner.move_vec(),
            postfix_token_list: Vec::new(),
            symbol_stack: Vec::new(),
            done: false,
        };

        ret.postfix_token_list.reserve(ret.infix_token_stack.len());
        ret
    }

    /// Converts the stored infix token list into a postfix one.
    ///
    /// After this function, call get_postfix_vec to retrieve the postfix vector.
    pub fn convert(mut self) -> Result<PostfixConverter, String> {
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
    /// Should only be called after `convert`. Otherwise, the return value isn't useful.
    /// * Return: the converted postfix token list.
    pub fn move_postfix_vec(self) -> Vec<TokenType> {
        self.postfix_token_list
    }

    /// Prints the current postfix stack. Only useful when debugging.
    pub fn print_postfix_stack(&self) {
        for token in self.postfix_token_list.iter() {
            println!("{}", token);
        }
    }

    /// Handles the token passed in. This symbol is assumed to be the symbol just `pop`ped from
    /// this converter's infix stack.
    /// Read the documentation on `TokenType::precedence` and `TokenType::is_symbol` for the
    /// general mechanism.
    ///
    /// * `token`: the token passed in.
    fn push_symbol(&mut self, token: TokenType) -> Result<(), String> {
        // TODO: generalize parts of this operation, using some sort of precedence mechanism
        
        // TL;DR:
        //
        // LParen has lowest precedence, hence is pushed right away onto the symbol stack.
        // RParen will not push itself onto the symbol stack, but rather, it's a signal that every
        // symbol popped up until the first left parentheses should be pushed onto the postfix token
        // list. Then, that left parentheses is also removed from the stack.
        // All the other tokens work as described in `TokenType::precedence`. Following this logic,
        // characters and quantifiers get pushed straight to the postfix token list.
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
                None | Some(_) => self.symbol_stack.push(token),
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
