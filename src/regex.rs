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
    /// * `string`: the string to try match on.
    pub fn first_match<'a>(&self, string: &'a str) -> Option<&'a str> {
        self.first_match_priv(string, 0)
    }

    /// The private, recursive part of first_match.
    ///
    /// * `string`: the string to try match on.
    /// * `pos`: the string index.
    fn first_match_priv<'a>(&self, string: &'a str, pos: usize) -> Option<&'a str> {
        if let Some(ret) = self.match_step_substr(string, pos) {
            return Some(ret);
        }
        if string.is_empty() {
            return None;
        }

        self.first_match_priv(string, 1 + pos)
    }

    /// Returns a collection (in this case, LinkedList) of number pairs. The two numbers in each
    /// pair represents the substring range that matches the regular expression.
    ///
    /// * `string`: the string to try match on.
    pub fn match_all_index(&self, string: &str) -> Option<LinkedList<(usize, usize)>> {
        let mut ret: LinkedList<(usize, usize)> = LinkedList::new();
        let mut str_ptr: usize = 0;

        while str_ptr < string.len() {
            if let Some(substr_idx) = self.match_step_index(string, str_ptr) {
                ret.push_back((str_ptr, substr_idx));
                str_ptr += if substr_idx > str_ptr {
                    substr_idx - str_ptr
                } else {
                    1
                };
            } else {
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
            if let Some(substr_idx) = self.match_step_index(string, str_ptr) {
                ret.push_back(&string[str_ptr..=substr_idx]);
                str_ptr += if substr_idx > 0 {
                    substr_idx - str_ptr
                } else {
                    1
                };
            } else {
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

    /// Tries to match the substring starting at `pos` with the regular expression.
    /// Returns `true` if that substring matches, `false` otherwise.
    ///
    /// * `string`:
    fn match_step_substr<'a>(&self, string: &'a str, pos: usize) -> Option<&'a str> {
        if let Some(idx) = self.match_step_index(string, pos) {
            return Some(&string[0..idx]);
        }

        None
    }

    /// Returns the index, of which the substring from `pos` to the returned index matches the regex.
    /// If it doesn't match, returns None.
    ///
    /// * `string`: the string to try match on.
    /// * pos: the string index. This method tries match on &string[pos..]
    fn match_step_index(&self, ref_str: &str, pos: usize) -> Option<usize> {
        // TL;DR: a kind-of DFS algorithm. I would say it's a bit more complicated because the
        // program also needs to keep track of the parts of the string it's trying to match.

        // possible todo: separate this code out into more specific methods, such as, match_start
        // if "^" is included, or match_end if "$" is included, or match_part which returns the
        // string slice which matches the regex.

        struct RefStackElem {
            ref_ptr: usize,
            str_ptr: usize,
        }

        // since I only execute the next step, this is pretty much as much as almost all
        // situations may need.
        // Also, this function isn't run recursively, so at most 36 wasted bytes is nothing.
        
        let mut ref_stack: Vec<RefStackElem> = Vec::with_capacity(3);
        ref_stack.push(RefStackElem {
            ref_ptr: 0,
            str_ptr: pos,
        });

        let mut max_match: Option<usize> = None;

        while let Some(ref_elem) = ref_stack.pop() {
            // println!("element number: {}", ref_elem.ref_ptr);
            let top_token = self.nfa.get_state(ref_elem.ref_ptr).unwrap();
            // println!("{0}, ref {1}", top_token.token.token_type, top_token.token.pos);
            match top_token.token.token_type {
                TokenType::Dollar => {
                    if ref_elem.str_ptr < ref_str.len() - 1 {
                        continue;
                    }
                }
                TokenType::Hat => {
                    if pos > 0 {
                        continue;
                    }
                }
                _ => {}
            };
            // i should let the panic be inside the get_state function, if I want it to panic.
            if ref_elem.ref_ptr == self.nfa.end {
                if max_match.is_none() || ref_elem.str_ptr > max_match.unwrap() {
                    max_match = Some(ref_elem.str_ptr);
                }
                continue;
            }

            // this is a kind of lazy way to handle the stack. I may need to think of a better way
            // some time.

            // ordered in such a way that, when there are no concrete character matches found
            // anymore, reserve to the empty transitions list.
            {
                let mut empty_nexts: Vec<RefStackElem> = top_token
                    .get_next_indices(|token| {
                        token.0 == TokenType::Empty
                            || token.0 == TokenType::Hat
                            || token.0 == TokenType::Dollar
                    })
                    .iter()
                    .map(|&idx| RefStackElem {
                        ref_ptr: idx,
                        str_ptr: ref_elem.str_ptr,
                    })
                    .collect();
                ref_stack.append(&mut empty_nexts);
            }
            if let Some(match_token) = ref_str
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

        max_match
    }
}

mod test;
