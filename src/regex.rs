use std::collections::HashSet;

use crate::lexer::token_type::TokenType;
use crate::parser::nfa::Nfa;

// just a wrapper around the NFA
pub struct Regex {
    pub(crate) nfa: Nfa,
}

impl Regex {
    pub(crate) fn from_nfa(nfa: Nfa) -> Self {
        Self { nfa }
    }

    pub fn is_match(&self, string: &str) -> bool {
        // TODO: finish matching
        let mut ref_stack: Vec<(usize, &str)> = vec![(0, string)];

        let mut curr_set: HashSet<(usize, &str)> = HashSet::new();
        curr_set.insert((0, string));

        while let Some(token_ref) = ref_stack.pop() {
            if token_ref.0 == self.nfa.end {
                return true;
            }
            // i should let the panic be at the NFA position.
            let top_token = self.nfa.get_state(token_ref.0).unwrap();

            {
                let mut empty_nexts: Vec<(usize, &str)> = top_token
                    .get_next_indices(TokenType::Empty)
                    .iter()
                    .map(|&idx| (idx, token_ref.1))
                    .collect();
                ref_stack.append(&mut empty_nexts);
            }

            if let Some(match_token) = token_ref.1.chars().nth(0).map(TokenType::Character) {
                let mut append_vec: Vec<(usize, &str)> = top_token
                    .get_next_indices(match_token)
                    .iter()
                    .map(|&idx| (idx, &token_ref.1[1..]))
                    .collect();

                ref_stack.append(&mut append_vec);
            }
        }

        // creep up 1 letter
        if string.is_empty() {
            return false;
        }
        self.is_match(&string[1..])
    }
}

mod test;
