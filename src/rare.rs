#![allow(dead_code)]
use std::collections::{BTreeSet, HashSet, LinkedList};

use crate::lexer::token_type::TokenType;
use crate::parser::nfa::Nfa;
use crate::parser::Parser;

/// An encapsulated object over the parse result of the `Parser`. Obtained by calling the method
/// `RARE::new`
///
/// `RARE` can call match functions without any thread synchronization method, assuming the user
/// doesn't change the `RARE` instance they hold.
///
/// Also the name of the project.
pub struct RARE {
    pub(crate) nfa: Nfa,
}

/// Memo lists for the current list of states that is being processed. Also provides a next_states
/// list, which is swapped with curr_states after curr_states is emptied in the function
/// `step_once`. The latter is mainly to avoid reallocation.
///
/// The fact that this is a struct separate from `RARE` also means that you can match RARE with
/// however many threads you like, since each time a match function is called, an instance of this
/// is constructed.
///
/// * `curr_states`:
/// * `next_states`:
struct CurrStatesData {
    // the results between BTreeSet and HashSet is benchmarked by me. It's somewhat surprising that
    // BTreeSet runs faster than HashSet.
    // The current theory I have is, if I use a HashSet, I need to drain the set and collect using
    // a newly allocated vector before adding elements again; otherwise, the borrow checker isn't
    // happy with me. For a BTreeSet though, since the elements are always in a specific order, I
    // can pop an element off and insert a new one without much trouble.
    
    curr_states: BTreeSet<usize>,
    // a temporary list that gets swapped with curr_states after curr_states is emptied.
    next_states: BTreeSet<usize>,
}

/// Holds information about the current string matched. The entire point of this struct is to
/// remove any possibly modifiable members out of `RARE`, making any `RARE` instance thread-safe,
/// at the cost of some memory to hold each of this object.
///
/// * `strlen`:
/// * `curr_pos`:
/// * `curr_char`:
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

    /// Constructs a new `RARE` matcher.
    ///
    /// * `regex`:
    pub fn new(regex: &str) -> Result<Self, String> {
        let mut parser = Parser::new(regex)?;
        parser.parse()
    }

    /// Returns whether there is a match in the string passed in.
    ///
    /// * `string`:
    pub fn is_match(&self, string: &str) -> bool {
        let mut str_data = StringIterData {
            strlen: string.len(),
            curr_pos: 0,
            curr_char: string.chars().nth(0),
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
            str_data.curr_char = string.chars().nth(str_data.curr_pos);
        }

        false
    }

    /// Returns a list of pairs, whose start and end represents the substring that matches the
    /// expression of this `RARE` instance.
    ///
    /// * `string`:
    pub fn match_all(&self, string: &str) -> Option<LinkedList<(usize, usize)>> {
        let mut ret_vec = LinkedList::new();
        let mut curr_str_ptr = 0;

        let mut str_data = StringIterData {
            strlen: string.len(),
            curr_pos: 0,
            curr_char: string.chars().nth(0),
        };

        let mut curr_state_data = CurrStatesData {
            curr_states: {
                let mut ret = BTreeSet::new();
                ret.insert(0);
                ret
            },
            next_states: BTreeSet::new(),
        };

        while curr_str_ptr < string.len() {
            str_data.curr_pos = curr_str_ptr;
            str_data.curr_char = string.chars().nth(curr_str_ptr);

            // Why do we accept the case it's equal to strlen (aka, 1 over the last valid string
            // index)?
            //
            // At the very end of the string, the matcher may need to run once more. The simplest
            // example is a dollar anchor: even if the entire string has already matches, the
            // matcher isn't yet at the final state; between it and the final state, there's still
            // the anchor; hence it needs to run once more. And of course, this last run only
            // yields a match if there's a path from the list of current states to the end state,
            // such that all other states on it are empty or anchors.
            
            // if we don't find any match for the substring from curr_str_ptr, increment
            // curr_str_ptr.
            let mut incre = 1;
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
                    incre = 0;
                }
                str_data.curr_pos += 1;
                str_data.curr_char = string.chars().nth(str_data.curr_pos);
            }

            curr_str_ptr += incre;
        }

        if ret_vec.is_empty() {
            None
        } else {
            Some(ret_vec)
        }
    }

    /// Given the list of states and the string input, move the list of current states by one step.
    /// Returns whether the next step indicates a match.
    ///
    /// * `state_data`: the collection of current states.
    /// * `str_data`: the input data.
    fn step_once(&self, state_data: &mut CurrStatesData, str_data: &StringIterData) -> bool {
        debug_assert!(!state_data.curr_states.is_empty());
        // print!("Current states: [");
        // for &ref_state in ref_data.curr_states.iter() {
        //     print!("{ref_state}, ");
        // }
        // println!("]");

        while let Some(curr_ref) = state_data.curr_states.pop_first() {
            if curr_ref == self.nfa.end {
                // ref_data.next_states.remove(&self.nfa.end);
                std::mem::swap(&mut state_data.curr_states, &mut state_data.next_states);
                return true;
            }
            state_data
                .next_states
                .extend(self.get_next_of(curr_ref, str_data));
        }

        std::mem::swap(&mut state_data.curr_states, &mut state_data.next_states);
        state_data.curr_states.contains(&self.nfa.end)
    }

    /// Gets the next state(s) given the current state and the input.
    ///
    /// * `state_ref`: the current state.
    /// * `str_data`: the input data.
    fn get_next_of(&self, state_ref: usize, str_data: &StringIterData) -> HashSet<usize> {
        let mut ret = HashSet::new();
        // We want to skip empty transitions.
        // Hat (^) is the same as empty if the current position is the start of the string,
        // and dollar ($) is the same if the current position is the end of the string.
        //
        // The skip set basically works like a stack or queue, but we don't want to traverse
        // elements already inside. So, BTreeSet is chosen.
        //
        // Note to self, HashSet doesn't have a concrete order. So, doing both iteration and
        // insertion at the same time is not a good idea (and Rust knows this).
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
                        if str_data.curr_pos >= str_data.strlen - 1 {
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
