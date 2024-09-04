use crate::regex::Regex;

pub(crate) mod nfa;
mod state;

use nfa::Nfa;
use state::State;

use crate::{
    lexer::{scanner::Scanner, token_type::TokenType},
    postfix_converter::PostfixConverter,
};

pub struct Parser {
    postfix_stack: Vec<TokenType>,
    nfa_stack: Vec<Nfa>,
}
impl Parser {
    pub fn new(regex: &str) -> Result<Self, String> {
        let mut pfix_stack = {
            let mut scanner = Scanner::new(regex);
            scanner.scan();
            // i should redesign the convert method.
            let conv = PostfixConverter::from_scanner(scanner);
            match conv.convert() {
                Ok(conv) => conv.move_postfix_vec(),
                Err(msg) => return Err(msg),
            }
        };

        pfix_stack.reverse();

        Ok(Self {
            postfix_stack: pfix_stack,
            nfa_stack: vec![Nfa::new(TokenType::Empty)],
        })
    }

    pub fn parse(&mut self) -> Regex {
        while let Some(token) = self.postfix_stack.pop() {
            if token.is_symbol() {
                let _ = self.handle_symbol(token);
            } else {
                self.nfa_stack.push(Nfa::new(token));
            }
        }

        // if something throws here, I must handle these cases.
        let last_state = self.nfa_stack.pop().unwrap();
        if let Some(mut ret) = self.nfa_stack.pop() {
            ret.merge(last_state);
            return Regex::from_nfa(ret);
        }
        Regex::from_nfa(last_state)
    }

    fn handle_symbol(&mut self, input: TokenType) -> Result<(), String> {
        debug_assert!(input.is_symbol());

        match input {
            TokenType::Concat => self.handle_concat(),
            TokenType::Beam => self.handle_beam(),
            TokenType::Star => self.handle_star(),
            _ => todo!(),
        }
    }

    fn handle_concat(&mut self) -> Result<(), String> {
        let second_nfa = match self.nfa_stack.pop() {
            None => return Err(String::from("Error, no value to concatenate")),
            Some(ret) => ret,
        };
        let mut first_nfa = match self.nfa_stack.pop() {
            None => return Err(String::from("Error, insufficient value to concatenate")),
            Some(ret) => ret,
        };

        first_nfa.merge(second_nfa);
        self.nfa_stack.push(first_nfa);

        Ok(())
    }

    fn handle_beam(&mut self) -> Result<(), String> {
        let second_nfa = match self.nfa_stack.pop() {
            None => return Err(String::from("Error, no value to do an OR")),
            Some(ret) => ret,
        };
        let first_nfa = match self.nfa_stack.pop() {
            None => Nfa::new(TokenType::Empty),
            Some(ret) => ret,
        };

        let first_end = first_nfa.states.len();
        let mut push_nfa = Nfa::new(TokenType::Empty);

        push_nfa.merge(first_nfa);
        push_nfa.end = 0;
        push_nfa.merge(second_nfa);

        {
            // last index plus 1
            let new_last_len = push_nfa.states.len();
            push_nfa
                .states
                .get_mut(first_end)
                .unwrap()
                .add_edge(TokenType::Empty, new_last_len);
            push_nfa
                .states
                .last_mut()
                .unwrap()
                .add_edge(TokenType::Empty, new_last_len);
            push_nfa.add_state(State::new(TokenType::Empty));
        }

        self.nfa_stack.push(push_nfa);
        Ok(())
    }

    fn handle_star(&mut self) -> Result<(), String> {
        let star_nfa = self.nfa_stack.pop().unwrap();
        let mut new_nfa = Nfa::new(TokenType::Empty);

        new_nfa.merge(star_nfa);

        new_nfa.states[new_nfa.end].add_edge(TokenType::Empty, 0);
        new_nfa.merge(Nfa::new(TokenType::Empty));
        new_nfa.states[0].add_edge(TokenType::Empty, new_nfa.end);

        self.nfa_stack.push(new_nfa);
        Ok(())
    }
}

mod test;
