use crate::lexer::token_type::{Token, TokenType};

/// Represents a state inside the NFA.
///
/// * `token`: the token associated with this state.
/// * `edges`: the edges pointing to the next states in the NFA. Has 2 components, the token type
///            of the next state, and the pointer to the next state.
///            The pointers to states of the NFA is represented as vector indices.
pub(crate) struct State {
    pub token: Token,
    // at most, this is 2. so, maybe I can optimize this.
    pub edges: Vec<(TokenType, usize)>,
}

impl State {
    /// Constructs a new state with the specified token.
    ///
    /// * `token`: the specified token.
    pub fn new(token: Token) -> Self {
        Self {
            token,
            edges: Vec::new(), }
    }

    /// Adds an edge to this state.
    ///
    /// * `input`: 
    /// * `index`: 
    pub fn add_edge(&mut self, input: TokenType, index: usize) {
        self.edges.push((input, index))
    }

    /// Given the input, return the next indices.
    ///
    /// note, this function may not be used in the future, as there will be more complicated
    /// conditions than just matching one type of input.
    ///
    /// * `input`: 
    pub fn get_next_indices<P: Fn(&&(TokenType, usize)) -> bool>(&self, pred: P) -> Vec<usize> {
        self.edges
            .iter()
            .filter(pred)
            .map(|edge| edge.1)
            .collect()
    }
}

