use std::fmt;

/// The types of token. The integer representation makrs the precedence for symbols,
/// which is needed when parsing. The characters (Character and Dot) don't need precedence,
/// but they still have their integer number to help distinguishing.
#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
#[repr(u8)]
pub enum TokenType {
    Empty = 0,
    LParen,
    RParen,
    Beam,
    Concat,
    QuestionMark,
    Plus,
    Star,
    /// Simply concatenates the 2 tokens it stands between
    /// Characters don't have precedence
    Character(char),
    Dot,
    /// The actual token is expected to be the next token.
    /// This token is not pushed to the token list when scanning the input string.
    Escape,
}

pub(crate) struct Token {
    pub pos: usize,
    pub token: TokenType,
}

impl Token {
    pub fn new(pos: usize, token: TokenType) -> Self {
        Self { pos, token }
    }
}

impl TokenType {
    #[allow(dead_code)]
    /// Returns the discriminant value
    fn discriminant(&self) -> u8 {
        // this is copied from Rust docs.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    /// Returns an arbitrary integer representing the precedence of the symbol. If the postfix
    /// converter is reading a symbol, it first pops off symbols on the symbol stack that has
    /// higher or equal precedence than the one it currently reads, and once it meets a symbol with
    /// lower precedence, it pushes the symbol it's currently reading onto the symbol stack.
    ///
    /// By this convention, the order of precedence is, from low to high, as follow:
    /// Parentheses,
    /// Beam,
    /// Concatenation operation,
    /// Quantifiers (for now, star, question mark and plus).
    /// Characters always go straight into the postfix stack, so they have the highest precedence.
    pub(crate) fn precedence(&self) -> u8 {
        match self {
            TokenType::LParen | TokenType::RParen => 0,
            TokenType::Beam => 1,
            TokenType::Concat => 2,
            TokenType::QuestionMark | TokenType::Plus | TokenType::Star => 3,
            // non-symbols anyways.
            _ => 4,
        }
    }

    /// Returns whether this token is a symbol.
    ///
    /// Using the convention described at the `precedence` function, a symbol's precedence is less
    /// than that of characters.
    pub(crate) fn is_symbol(&self) -> bool {
        self.precedence() <= TokenType::precedence(&TokenType::Star)
    }

    /// Returns whether the next token needs to be preceded by concatenation, if that token is not
    /// a symbol, or is the left parentheses. This function is only useful for the lexer.
    ///
    /// Beam and LParen shouldn't be concatenated, because they signal the start of a new string.
    /// The other token types may be a continuation of a string, so concatentation may be necessary.
    /// The escape character is handled by the lexer: it simply uses the concatenation value of the
    /// token before it.
    pub(crate) fn need_concat_next(&self) -> bool {
        !matches!(self, TokenType::LParen | TokenType::Beam)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Empty => write!(f, "Empty"),
            TokenType::Character(c) => write!(f, "Character: {}", c),
            TokenType::Dot => write!(f, "Dot (.)"),
            TokenType::Star => write!(f, "Star (*)"),
            TokenType::Beam => write!(f, "Beam (|)"),
            TokenType::QuestionMark => write!(f, "Question mark (?)"),
            TokenType::Plus => write!(f, "Plus (+)"),
            TokenType::LParen => write!(f, "Left parentheses (()"),
            TokenType::RParen => write!(f, "Right parentheses ())"),
            TokenType::Escape => write!(f, "Escape (\\)"),
            TokenType::Concat => write!(f, "Concatenation"),
        }
    }
}
