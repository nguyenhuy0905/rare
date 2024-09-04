use super::state::State;
use crate::lexer::token_type::TokenType;

pub struct Nfa {
    pub states: Vec<State>,
    pub end: usize,
}

impl Nfa {
    /// A new NFA should always has 1 state inside.
    /// but this function doesn't do that for you :).
    pub fn new(first_input: TokenType) -> Self {
        Self {
            states: vec![State::new(first_input)],
            end: 0,
        }
    }

    pub fn add_state(&mut self, state: State) {
        self.states.push(state);
        self.end += 1;
    }

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

    pub(crate) fn get_state(&self, index: usize) -> Option<&State> {
        self.states.get(index)
    }

    #[allow(dead_code)]
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
