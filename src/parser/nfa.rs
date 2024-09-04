use super::state::State;
use crate::lexer::token_type::TokenType;

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
    pub fn new(first_input: TokenType) -> Self {
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
        for state in another.states.iter_mut() {
            for edge in state.edges.iter_mut() {
                edge.1 += self.states.len();
            }
        }

        {
            // damn you borrow checker
            let l = self.states.len();
            self.states
                .get_mut(self.end)
                .unwrap()
                .add_edge(another.states.first().unwrap().token.clone(), l);
            self.states.append(&mut another.states);
        }

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
            println!("state (index {idx}): {}", state.token);
            print!("\tedges: ");
            for edge in state.edges.iter() {
                print!("{}, to index {}; ", edge.0, edge.1);
            }
            println!();
        }
    }
}
