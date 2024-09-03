use crate::lexer::token_type::TokenType;
use crate::parser::nfa::Nfa;

// just a nicer name for the NFA
pub struct Regex {
    pub(crate) nfa: Nfa,
}

impl Regex {
    pub(crate) fn from_nfa(nfa: Nfa) -> Self {
        Self {
            nfa,
        }
    }

    pub fn is_match(&self, string: & str) -> bool {
        let mut ref_stack: Vec<usize> = vec![0];
        // TODO: finish matching

        false
    }
}

mod test;
