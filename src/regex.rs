use std::collections::LinkedList;

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

    /// Returns the first matching substring.
    ///
    /// * `string`:
    pub fn first_match<'a>(&self, string: &'a str) -> Option<&'a str> {
        if let Some(ret) = self.match_step_substr(string) {
            return Some(ret);
        }
        if string.is_empty() {
            return None;
        }
        self.first_match(&string[1..])
    }

    pub fn match_all_index(&self, string: &str) -> Option<LinkedList<(usize, usize)>> {
        let mut ret: LinkedList<(usize, usize)> = LinkedList::new();
        let mut str_ptr: usize = 0;

        while str_ptr < string.len() {
            if let Some(substr_idx) = self.match_step_index(&string[str_ptr..]) {
                ret.push_back((str_ptr, str_ptr + substr_idx));
                str_ptr += if substr_idx > 0 {substr_idx} else {1};
            }
            else {
                str_ptr += 1;
            }
        }

        if ret.is_empty() {
            None
        } else {
            Some(ret)
        }
    }

    #[allow(dead_code)]
    /// Try to match until the end of the string.
    /// Since the return collection can grow really large, a LinkedList is used.
    ///
    /// * `string`: 
    /// * Return: A linked list of all matched string.
    pub fn match_all<'a>(&self, string: &'a str) -> Option<LinkedList<&'a str>> {
        let mut ret: LinkedList<&'a str> = LinkedList::new();
        let mut str_ptr: usize = 0;
        
        while str_ptr < string.len() {
            if let Some(substr_idx) = self.match_step_index(&string[str_ptr..]) {
                ret.push_back(&string[str_ptr..=(str_ptr + substr_idx)]);
                str_ptr += if substr_idx > 0 {substr_idx} else {1};
            }
            else {
                str_ptr += 1;
            }
        }

        if ret.is_empty() {
            None
        } else {
            Some(ret)
        }
    }

    #[allow(dead_code)]
    /// Try to match a string with this `Regex`.
    ///
    /// * `string`:
    /// * Return: `true` if part or all of the string matches the regular expression, `false` otherwise.
    pub fn is_match(&self, string: &str) -> bool {
        self.first_match(string).is_some()
    }

    /// Returns the matching part of the string if there is any. And None otherwise.
    ///
    /// * `string`:
    fn match_step_substr<'a>(&self, string: &'a str) -> Option<&'a str> {
        if let Some(idx) = self.match_step_index(string) {
            return Some(&string[0..=idx]);
        }

        // if this part of the string doesn't match and the string is not empty yet, try this on
        // the same string, minus the first letter.
        None
    }

    /// Returns the index, of which the substring from 0 to the returned index matches the regex.
    /// If it doesn't match, returns None.
    ///
    /// Note: the first tuple element is currently bugged. Do not use it.
    ///
    /// * `string`:
    fn match_step_index(&self, string: &str) -> Option<usize> {
        // TL;DR: a kind-of DFS algorithm. I would say it's a bit more complicated because the
        // program also needs to keep track of the parts of the string it's trying to match.

        // possible todo: separate this code out into more specific methods, such as, match_start
        // if "^" is included, or match_end if "$" is included, or match_part which returns the
        // string slice which matches the regex.

        struct RefStackElem {
            ref_ptr: usize,
            str_ptr: usize,
        }

        let mut ref_stack: Vec<RefStackElem> = vec![RefStackElem {
            ref_ptr: 0,
            str_ptr: 0,
        }];

        while let Some(ref_elem) = ref_stack.pop() {
            if ref_elem.ref_ptr == self.nfa.end {
                // why is it minus 1? I have no idea.
                return Some(ref_elem.str_ptr - 1);
            }
            // i should let the panic be inside the get_state function, if I want it to panic.
            let top_token = self.nfa.get_state(ref_elem.ref_ptr).unwrap();

            // this is a kind of lazy way to handle the stack. I may need to think of a better way
            // some time.

            // ordered in such a way that, when there are no concrete character matches found
            // anymore, reserve to the empty transitions list.
            {
                let mut empty_nexts: Vec<RefStackElem> = top_token
                    .get_next_indices(|token| token.0 == TokenType::Empty)
                    .iter()
                    .map(|&idx| RefStackElem {
                        ref_ptr: idx,
                        str_ptr: ref_elem.str_ptr,
                    })
                    .collect();
                ref_stack.append(&mut empty_nexts);
            }

            if let Some(match_token) = string
                .chars()
                .nth(ref_elem.str_ptr)
                .map(TokenType::Character)
            {
                let mut append_vec: Vec<RefStackElem> = top_token
                    .get_next_indices(|token| token.0 == TokenType::Dot || token.0 == match_token)
                    .iter()
                    .map(|&idx| RefStackElem {
                        ref_ptr: idx,
                        str_ptr: ref_elem.str_ptr + 1,
                    })
                    .collect();

                ref_stack.append(&mut append_vec);
            }
        }

        None
    }
}

mod test;
