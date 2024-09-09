use crate::lexer::token_type::Token;

/// Represents a state inside the NFA.
///
/// * `token`: the token associated with this state.
/// * `edges`: the edges pointing to the next states in the NFA. Has 2 components, the token type
///            of the next state, and the pointer to the next state.
///            The pointers to states of the NFA is represented as vector indices.
pub(crate) struct State {
    pub token: Token,
    // at most, this is 2. so, maybe I can optimize this.
    pub edges: Vec<usize>,
}

impl State {
    #[inline]
    /// Constructs a new state with the specified token.
    ///
    /// * `token`: the specified token.
    pub fn new(token: Token) -> Self {
        Self {
            token,
            edges: Vec::new(), }
    }

    #[inline]
    /// Adds an edge to this state.
    ///
    /// * `input`: 
    /// * `index`: 
    pub fn add_edge(&mut self, index: usize) {
        self.edges.push(index)
    }
}
