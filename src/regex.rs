use crate::lexer::token_type::TokenType;
use crate::parser::nfa::Nfa;

/// An encapsulated object over the parse result of the `Parser`, and obtained by calling
/// `Parser::parse`.
pub struct Regex {
    pub(crate) nfa: Nfa,
}

impl Regex {
    /// Constructs a `Regex` from a NFA. Should only be called by the `Parser`
    ///
    /// * `nfa`:
    pub(crate) fn from_nfa(nfa: Nfa) -> Self {
        Self { nfa }
    }

    /// Try to match a string with this `Regex`.
    ///
    /// * `string`:
    /// * Return: `true` if part or all of the string matches the regular expression, `false` otherwise.
    pub fn is_match(&self, string: &str) -> bool {
        // TL;DR: a kind-of DFS algorithm. I would say it's a bit more complicated because the
        // program also needs to keep track of the parts of the string it's trying to match.

        // possible todo: separate this code out into more specific methods, such as, match_start
        // if "^" is included, or match_end if "$" is included, or match_part which returns the
        // string slice which matches the regex.

        let mut ref_stack: Vec<(usize, &str)> = vec![(0, string)];

        while let Some(token_ref) = ref_stack.pop() {
            if token_ref.0 == self.nfa.end {
                return true;
            }
            // i should let the panic be inside the get_state function, if I want it to panic.
            let top_token = self.nfa.get_top_state(token_ref.0).unwrap();

            // ordered in such a way that, when there are no concrete character matches found
            // anymore, reserve to the empty transitions list.
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

        // if this part of the string doesn't match and the string is not empty yet, try this on
        // the same string, minus the first letter.
        if string.is_empty() {
            return false;
        }
        self.is_match(&string[1..])
    }
}

mod test;
