use super::state::State;
use crate::lexer::token_type::{Token, TokenType};

/// Nondeterministic finite automaton (NFA). Basically the representation of the regular
/// expression.
/// A NFA can be made of smaller NFAs. In that case, `Nfa::merge` should be called. The `self` NFA
/// in that function call precedes the NFA passed in as the other parameter.
///
/// * `states`: a vector of all states contained by the NFA. The beginning of the NFA is guaranteed
///             to be element 0 of the NFA.
/// * `end`: the final state of the current NFA.
pub(crate) struct Nfa {
    pub states: Vec<State>,
    pub end: usize,
}

impl Nfa {
    /// Constructs a new NFA.
    /// A new NFA should always has 1 state inside.
    ///
    /// * Return: the newly constructed NFA.
    pub fn new(first_input: Token) -> Self {
        Self {
            states: vec![State::new(first_input)],
            end: 0,
        }
    }

    /// Adds the specified state to the NFA.
    /// IMPORTANT: `Nfa::end` is assumed to be the last element in the NFA.
    ///
    /// * `state`:
    pub fn add_state(&mut self, state: State) {
        self.states.push(state);
        self.end += 1;
    }

    /// Merges 2 NFAs together. The `self` NFA precedes the `another` NFA, that is, `self::end`
    /// points to `another::start` (which is, the first element in `another`'s states list).
    ///
    /// * `another`:
    pub fn merge(&mut self, mut another: Nfa) {
        // for state in another.states.iter_mut() {
        //     for edge in state.edges.iter_mut() {
        //         edge.1 += self.states.len();
        //     }
        // }
        //
        // {
        //     // damn you borrow checker
        //     let l = self.states.len();
        //     self.states
        //         .get_mut(self.end)
        //         .unwrap()
        //         .add_edge(another.states.first().unwrap().token.token_type.clone(), l);
        //     self.states.append(&mut another.states);
        // }

        let skips = {
            let self_end_type = &self.states[self.end].token.token_type;
            let another_start_type = &another.states[0].token.token_type;

            if *self_end_type == TokenType::Empty && *another_start_type == TokenType::Empty {
                1
            } else {
                0
            }
        };

        let self_old_len = self.states.len() - skips;
        if skips == 1 {
            let self_end_state = &mut self.states[self.end];
            for edge in another.states[0].edges.iter() {
                // alternative to edge.0.clone() would be to remove the front state of another.
                self_end_state.add_edge(edge + self_old_len);
            }
        }

        for state in another.states.iter_mut() {
            for edge in state.edges.iter_mut() {
                *edge += self_old_len;
            }
        }

        if skips == 0 {
            self.states[self.end]
                .add_edge(self_old_len);
        }
        self.states.extend(another.states.into_iter().skip(skips));

        self.end = self.states.len() - 1;
    }

    /// A more graceful way of accessing the NFA's state.
    /// But the author may be too lazy to use this function :(.
    ///
    /// * `index`:
    pub fn get_state(&self, index: usize) -> Option<&State> {
        self.states.get(index)
    }

    #[allow(dead_code)]
    /// Prints the current list of states. Only useful for debugging.
    pub fn print_states(&self) {
        for (idx, state) in self.states.iter().enumerate() {
            if std::ptr::eq(state, self.states.last().unwrap()) {
                print!("last ");
            }
            println!("state (index {idx}): {}", state.token.token_type);
            print!("\tedges: ");
            for edge in state.edges.iter() {
                print!("{}, to index {}; ", self.states[*edge].token.token_type, edge);
            }
            println!();
        }
    }
}
