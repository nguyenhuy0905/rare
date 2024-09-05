use crate::{lexer::token_type::Token, regex::Regex};

pub(crate) mod nfa;
pub(crate) mod state;

use nfa::Nfa;
use state::State;

use crate::{
    lexer::{scanner::Scanner, token_type::TokenType},
    postfix_converter::PostfixConverter,
};

/// Parses a postfix stack into a NFA representing the regular expression.
/// The parser elements should not be accessed by the user manually. Instead, retrieve the regular
/// expression using `Parser::parse`.
///
/// * `postfix_stack`: the postfix stack passed in. It is assumed that this stack is provided by
///                    the postfix converter.
/// * `nfa_stack`: a temporary NFA stack. After `Parser::parse`, the stack should only have at most
///                2 NFAs left inside.
pub struct Parser {
    postfix_stack: Vec<Token>,
    nfa_stack: Vec<Nfa>,
}
impl Parser {
    /// Constructs a parser from the regex string passed in.
    /// After creating a parser, `Parser::parse` should be called.
    /// The `new` function does more than just saving the string. It processes the string to get a
    /// list of tokens, from which the parser can work on.
    ///
    /// * `regex`: regular expression string.
    /// * Return: the newly constructed `Parser` if successful, otherwise, a string describing the
    ///           error is returned.
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
            nfa_stack: vec![Nfa::new(Token::new(0, TokenType::Empty))],
        })
    }

    /// Parses the regular expression string held by this parser into a regular expression. After
    /// that, `Regex::is_match` can be called.
    ///
    /// * Return: the parsed `Regex` object.
    pub fn parse(&mut self) -> Result<Regex, String> {
        while let Some(tok) = self.postfix_stack.pop() {
            if tok.token.is_symbol() {
                self.handle_symbol(tok)?
            } else {
                self.nfa_stack.push(Nfa::new(tok));
            }
        }

        // note: there should always be at least 1 NFA by the time parsing finishes.
        let last_state = self.nfa_stack.pop().unwrap();
        // a case where there isn't another NFA down there: empty regular expression "".
        if let Some(mut ret) = self.nfa_stack.pop() {
            ret.merge(last_state);
            return Ok(Regex::from_nfa(ret));
        }
        Ok(Regex::from_nfa(last_state))
    }

    /// Handles the symbol passed in. This assumes that the input passed in is a symbol.
    ///
    /// * `input`: the symbol passed in.
    fn handle_symbol(&mut self, input: Token) -> Result<(), String> {
        debug_assert!(input.token.is_symbol());

        match input.token {
            TokenType::Concat => self.handle_concat(input.pos),
            TokenType::Beam => self.handle_beam(input.pos),
            TokenType::Star => self.handle_star(input.pos),
            TokenType::Plus => self.handle_plus(input.pos),
            TokenType::QuestionMark => self.handle_question_mark(input.pos),
            _ => Err(String::from(
                "Program bug in symbol handling. Contact the author about this error.",
            )),
        }
    }

    /// Processes the concat symbol.
    ///
    /// Concatenation can only succeed when there are at least 2 NFAs in the stack.
    fn handle_concat(&mut self, pos: usize) -> Result<(), String> {
        // Do I really need to tell you what this results in?
        // Also, the error should never throw unless it's the program's fault.
        let second_nfa = match self.nfa_stack.pop() {
            None => return Err(format!("At {pos}: No value to concatenate")),
            Some(ret) => ret,
        };
        let mut first_nfa = match self.nfa_stack.pop() {
            None => return Err(format!("At {pos}: Insufficient value to concatenate")),
            Some(ret) => ret,
        };

        first_nfa.merge(second_nfa);
        self.nfa_stack.push(first_nfa);

        Ok(())
    }

    /// Processes the beam symbol
    ///
    /// The beam symbol needs at least 1 NFA in the NFA stack.
    /// note: the parser doesn't check whether the 2 NFAs are exactly equivalent, because the
    /// resultant Regex is still valid without that check.
    fn handle_beam(&mut self, pos: usize) -> Result<(), String> {
        // TL;DR
        //
        //
        // (empty)────>(first_nfa)─────>(empty)
        //    └───>───(second_nfa)───>───┘

        let second_nfa = match self.nfa_stack.pop() {
            None => return Err(format!("Operation | at {0}: No value to do an OR |", pos + 1)),
            Some(ret) => ret,
        };
        let first_nfa = match self.nfa_stack.pop() {
            // position doesn't matter for empty.
            None => Nfa::new(Token::new(0, TokenType::Empty)),
            Some(ret) => ret,
        };

        // first NFA's end plus 1. After merging with push_nfa, this is where that end is.
        let first_end = first_nfa.states.len();
        let mut push_nfa = Nfa::new(Token::new(0, TokenType::Empty));

        push_nfa.merge(first_nfa);
        // maybe I should encapsulate this in a simple function. This is a bit of "magic", if you
        // don't know how `merge` works.
        push_nfa.end = 0;
        push_nfa.merge(second_nfa);

        {
            // last index plus 1
            let new_last_len = push_nfa.states.len();
            // point both ends of the newly merged NFAs to an empty node that is the end of the OR
            // boolean.
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
            push_nfa.add_state(State::new(Token::new(0, TokenType::Empty)));
        }

        self.nfa_stack.push(push_nfa);
        Ok(())
    }

    /// Handles the Kleene star symbol.
    ///
    /// Requires at least 1 NFA in the stack.
    fn handle_star(&mut self, pos: usize) -> Result<(), String> {
        // TL;DR
        //
        //   ┌────────────>─────────────┐
        // (empty)──>──(star_nfa)──>──(empty)
        //   └─────<──────┘

        let star_nfa = match self.nfa_stack.pop() {
            Some(r) => {
                if r.states.len() == 1 {
                    let last_state = r.states.last().unwrap();
                    if last_state.token.token == TokenType::Empty {
                        return Err(format!("Character *, position {0}: Missing preceding value", pos + 1))
                    }
                }
                r
            }
            None => return Err(format!("Character *, position {0}: Missing preceding value", pos + 1)),
        };
        let mut new_nfa = Nfa::new(Token::new(0, TokenType::Empty));

        new_nfa.merge(star_nfa);

        new_nfa.states[new_nfa.end].add_edge(TokenType::Empty, 0);
        new_nfa.merge(Nfa::new(Token::new(0, TokenType::Empty)));
        new_nfa.states[0].add_edge(TokenType::Empty, new_nfa.end);

        self.nfa_stack.push(new_nfa);
        Ok(())
    }

    /// Handles the plus symbol.
    ///
    /// Requires at least 1 NFA in the stack.
    fn handle_plus(&mut self, pos: usize) -> Result<(), String> {
        // TL;DR
        //
        // (empty)──>──(star_nfa)──>──(empty)
        //   └─────<──────┘
        // so, very similar to handle_star

        let star_nfa = match self.nfa_stack.pop() {
            Some(r) => {
                if r.states.len() == 1 {
                    let last_state = r.states.last().unwrap();
                    if last_state.token.token == TokenType::Empty {
                        return Err(format!("Character +, position {0}: Missing preceding value", pos + 1))
                    }
                }
                r
            }
            None => return Err(format!("Character +, position {0}: Missing preceding value", pos + 1)),
        };
        let mut new_nfa = Nfa::new(Token::new(0, TokenType::Empty));

        new_nfa.merge(star_nfa);

        new_nfa.states[new_nfa.end].add_edge(TokenType::Empty, 0);
        new_nfa.merge(Nfa::new(Token::new(0, TokenType::Empty)));
        // difference to star: this line
        // new_nfa.states[0].add_edge(TokenType::Empty, new_nfa.end);

        self.nfa_stack.push(new_nfa);
        Ok(())
    }

    /// Handles the question mark symbol.
    ///
    /// Requires at least 1 NFA in the stack.
    fn handle_question_mark(&mut self, pos: usize) -> Result<(), String> {
        // TL;DR
        //
        //   ┌────────────>─────────────┐
        // (empty)──>──(star_nfa)──>──(empty)
        // So, very similar to handle_star also.
        let star_nfa = match self.nfa_stack.pop() {
            Some(r) => {
                if r.states.len() == 1 {
                    let last_state = r.states.last().unwrap();
                    if last_state.token.token == TokenType::Empty {
                        return Err(format!("Character +, position {0}: Missing preceding value", pos + 1))
                    }
                }
                r
            }
            None => return Err(format!("Character +, position {0}: Missing preceding value", pos + 1)),
        };
        let mut new_nfa = Nfa::new(Token::new(0, TokenType::Empty));

        new_nfa.merge(star_nfa);

        // difference to star: this line
        // new_nfa.states[new_nfa.end].add_edge(TokenType::Empty, 0);
        new_nfa.merge(Nfa::new(Token::new(0, TokenType::Empty)));
        new_nfa.states[0].add_edge(TokenType::Empty, new_nfa.end);

        self.nfa_stack.push(new_nfa);
        Ok(())
    }
}

mod test;
