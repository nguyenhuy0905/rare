use std::cell::RefCell;

use crate::parser::nfa::Nfa;

// just a nicer name for the NFA
pub struct Regex {
    pub(in crate) nfa: Nfa,
    pub(in crate) ref_stack: RefCell<Vec<usize>>,
}

impl Regex {
    pub(in crate) fn from_nfa(nfa: Nfa) -> Self {
        Self {
            nfa,
            ref_stack: RefCell::new(vec![0]),
        }
    }

    pub fn match_string(&self, _string: &str) -> bool {
        false
    }
}

