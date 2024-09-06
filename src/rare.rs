#![allow(dead_code)]
use std::collections::{BTreeSet, HashSet, LinkedList};

use crate::lexer::token_type::TokenType;
use crate::parser::nfa::Nfa;
use crate::parser::Parser;

/// An encapsulated object over the parse result of the `Parser`, and obtained by calling
/// `Parser::parse`.
///
/// Also the name of the project.
pub struct RARE {
    pub(crate) nfa: Nfa,
}

struct CurrStatesData {
    curr_states: BTreeSet<usize>,
    // a temporary list that gets swapped with curr_states after curr_states is emptied.
    next_states: BTreeSet<usize>,
}

struct StringIterData {
    strlen: usize,
    curr_pos: usize,
    curr_char: Option<char>,
}

impl RARE {
    /// Constructs a `Regex` from a NFA. Should only be called by the `Parser`
    ///
    /// * `nfa`:
    pub(crate) fn from_nfa(nfa: Nfa) -> Self {
        Self { nfa }
    }

    /// Creates a new regex object.
    ///
    /// * `regex`:
    pub fn new(regex: &str) -> Result<Self, String> {
        let mut parser = Parser::new(regex)?;
        parser.parse()
    }

    pub fn is_match(&self, regex: &str) -> bool {
        let mut str_data = StringIterData {
            strlen: regex.len(),
            curr_pos: 0,
            curr_char: regex.chars().nth(0),
        };

        let mut curr_state_data = CurrStatesData {
            curr_states: {
                let mut ret = BTreeSet::new();
                ret.insert(0);
                ret
            },
            next_states: BTreeSet::new(),
        };

        while str_data.curr_pos < str_data.strlen + 1 {
            if curr_state_data.curr_states.is_empty() {
                curr_state_data.curr_states.insert(0);
            }
            if self.step_once(&mut curr_state_data, &str_data) {
                return true;
            }
            str_data.curr_pos += 1;
            str_data.curr_char = regex.chars().nth(str_data.curr_pos);
        }

        false
    }

    pub fn match_all(&self, regex: &str) -> Option<LinkedList<(usize, usize)>> {
        let mut ret_vec = LinkedList::new();
        let mut curr_str_ptr = 0;

        let mut str_data = StringIterData {
            strlen: regex.len(),
            curr_pos: 0,
            curr_char: regex.chars().nth(0),
        };

        let mut curr_state_data = CurrStatesData {
            curr_states: {
                let mut ret = BTreeSet::new();
                ret.insert(0);
                ret
            },
            next_states: BTreeSet::new(),
        };

        while curr_str_ptr < regex.len() {
            str_data.curr_pos = curr_str_ptr;
            str_data.curr_char = regex.chars().nth(curr_str_ptr);

            while str_data.curr_pos <= str_data.strlen {
                if curr_state_data.curr_states.is_empty() {
                    curr_str_ptr = str_data.curr_pos;
                    curr_state_data.curr_states.insert(0);
                }
                if self.step_once(&mut curr_state_data, &str_data) {
                    // otherwise, the next character may automatically match, even if it shouldn't
                    curr_state_data.curr_states.remove(&self.nfa.end);
                    ret_vec.push_back((curr_str_ptr, str_data.curr_pos));
                    curr_str_ptr = str_data.curr_pos;
                }
                str_data.curr_pos += 1;
                str_data.curr_char = regex.chars().nth(str_data.curr_pos);
            }

            curr_str_ptr += 1;
        }

        if ret_vec.is_empty() {
            None
        } else {
            Some(ret_vec)
        }
    }

    fn step_once(&self, ref_data: &mut CurrStatesData, str_data: &StringIterData) -> bool {
        debug_assert!(!ref_data.curr_states.is_empty());
        // print!("Current states: [");
        // for &ref_state in ref_data.curr_states.iter() {
        //     print!("{ref_state}, ");
        // }
        // println!("]");

        while let Some(curr_ref) = ref_data.curr_states.pop_first() {
            if curr_ref == self.nfa.end {
                // ref_data.next_states.remove(&self.nfa.end);
                std::mem::swap(&mut ref_data.curr_states, &mut ref_data.next_states);
                return true;
            }
            ref_data
                .next_states
                .extend(self.get_next_of(curr_ref, str_data));
        }

        std::mem::swap(&mut ref_data.curr_states, &mut ref_data.next_states);
        ref_data.curr_states.contains(&self.nfa.end)
    }

    fn get_next_of(&self, state_ref: usize, str_data: &StringIterData) -> HashSet<usize> {
        let mut ret = HashSet::new();
        // we want to skip empty transitions.
        // Hat (^) is the same as empty if the current position is the start of the string,
        // and dollar ($) is the same if the current position is the end of the string.
        let mut skip_set: BTreeSet<usize> = BTreeSet::new();
        skip_set.insert(state_ref);

        while let Some(skip_ref) = skip_set.pop_first() {
            if skip_ref == self.nfa.end {
                ret.insert(skip_ref);
                continue;
            }
            let skip_state = self.nfa.get_state(skip_ref).unwrap();

            for edge in skip_state.edges.iter() {
                // reminder; edge = (required match to transition, next state)
                match edge.0 {
                    TokenType::Character(c) => {
                        if str_data.curr_char.is_some() && c == str_data.curr_char.unwrap() {
                            ret.insert(edge.1);
                        }
                    }
                    TokenType::Dot => {
                        ret.insert(edge.1);
                    }
                    TokenType::Empty => {
                        skip_set.insert(edge.1);
                    }
                    // hat and dollar anchors: if they are at the matching positions in the string,
                    // act as if they were empty states. Otherwise, they are not valid next states.
                    TokenType::Hat => {
                        if str_data.curr_pos == 0 {
                            skip_set.insert(edge.1);
                        }
                    }
                    TokenType::Dollar => {
                        if str_data.curr_pos == str_data.strlen - 1 {
                            skip_set.insert(edge.1);
                        }
                    }
                    _ => todo!(),
                }
            }
        }

        ret
    }
}

mod test;
