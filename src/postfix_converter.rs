use crate::lexer::{scanner::Scanner, token_type::{Token, TokenType}};
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
    infix_token_stack: Vec<Token>,
    postfix_token_list: Vec<Token>,
    symbol_stack: Vec<Token>,
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
        while let Some(tok) = self.infix_token_stack.pop() {
            if !tok.token_type.is_symbol() {
                self.postfix_token_list.push(tok);
            } else if let Err(estr) = self.push_symbol(tok) {
                return Err(estr);
            }
        }

        while let Some(tok) = self.symbol_stack.pop() {
            if tok.token_type == TokenType::LParen {
                return Err(format!("Character ( at {0}: extra (", tok.pos + 1));
            }
            self.postfix_token_list.push(tok);
        }

        self.done = true;
        Ok(self)
    }

    /// Returns the postfix vector stored inside this converter.
    /// After this call, the postfix converter is burnt to a crisp.
    ///
    /// Should only be called after `convert`. Otherwise, the return value isn't useful.
    /// * Return: the converted postfix token list.
    pub fn move_postfix_vec(self) -> Vec<Token> {
        self.postfix_token_list
    }

    #[allow(dead_code)]
    /// Prints the current postfix stack. Only useful when debugging.
    pub fn print_postfix_stack(&self) {
        for tok in self.postfix_token_list.iter() {
            println!("{}", tok.token_type);
        }
    }

    /// Handles the token passed in. This symbol is assumed to be the symbol just `pop`ped from
    /// this converter's infix stack.
    /// Read the documentation on `TokenType::precedence` and `TokenType::is_symbol` for the
    /// general mechanism.
    ///
    /// * `token`: the token passed in.
    fn push_symbol(&mut self, tok: Token) -> Result<(), String> {
        // TODO: generalize parts of this operation, using some sort of precedence mechanism
        
        // TL;DR:
        //
        // LParen has lowest precedence, hence is pushed right away onto the symbol stack.
        // RParen will not push itself onto the symbol stack, but rather, it's a signal that every
        // symbol popped up until the first left parentheses should be pushed onto the postfix token
        // list. Then, that left parentheses is also removed from the stack.
        // All the other tokens work as described in `TokenType::precedence`. Following this logic,
        // characters and quantifiers get pushed straight to the postfix token list.
        match tok.token_type {
            TokenType::LParen => {
                self.symbol_stack.push(tok);
            }
            TokenType::RParen => {
                while let Some(pop_tok) = self.symbol_stack.pop() {
                    if pop_tok.token_type == TokenType::LParen {
                        return Ok(());
                    }
                    self.postfix_token_list.push(pop_tok);
                }
                return Err(format!("Character ) at {0}: missing a (", tok.pos + 1));
            }
            TokenType::Concat => match self.symbol_stack.last() {
                Some(Token{token_type: TokenType::Concat, ..}) => {
                    // position doesn't really matter for this token.
                    self.postfix_token_list.push(Token::new(tok.pos, TokenType::Concat));
                }
                None | Some(_) => self.symbol_stack.push(tok),
            },
            TokenType::Beam => match self.symbol_stack.last() {
                None | Some(Token{token_type: TokenType::LParen, ..}) => {
                    self.symbol_stack.push(tok);
                }
                Some(_) => {
                    self.postfix_token_list.push(
                        self.symbol_stack
                            .pop()
                            .expect("Trust me this can never go wrong"),
                    );
                    self.symbol_stack.push(tok);
                }
            },
            _ => {
                self.postfix_token_list.push(tok);
            }
        }

        Ok(())
    }
}

mod test;
