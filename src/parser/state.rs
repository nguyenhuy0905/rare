use crate::lexer::token_type::TokenType;

pub struct State {
    pub token: TokenType,
    // at most, this is 2. so, maybe I can optimize this.
    pub edges: Vec<(TokenType, usize)>,
}

impl State {
    pub fn new(token: TokenType) -> Self {
        Self {
            token,
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, input: TokenType, index: usize) {
        self.edges.push((input, index))
    }

    pub fn get_next_indices(&self, input: TokenType) -> Vec<usize> {
        self.edges
            .iter()
            .filter(|edge| edge.0 == input)
            .map(|edge| edge.1)
            .collect()
    }
}

