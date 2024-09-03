use std::fmt;

/// The types of token. The integer representation makrs the precedence for symbols,
/// which is needed when parsing. The characters (Character and Dot) don't need precedence,
/// but they still have their integer number to help distinguishing.
#[derive(PartialEq, Eq, PartialOrd, Clone, Hash, Debug)]
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

impl TokenType {
    /// Returns the discriminant value
    fn discriminant(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    /// Returns an arbitrary integer representing the precedence of the symbol. Characters don't
    /// really have precedence.
    ///
    /// The number returned here is supposedly used for precedence comparison, which is useful for
    /// postfix conversion.
    pub fn precedence(&self) -> u8 {
        match self {
            TokenType::LParen | TokenType::RParen => 0,
            TokenType::Beam => 1,
            TokenType::Concat => 2,
            TokenType::QuestionMark | TokenType::Plus | TokenType::Star => 3,
            // non-symbols anyways.
            _ => 4,
        }
    }

    pub fn is_symbol(&self) -> bool {
        self.precedence() <= TokenType::precedence(&TokenType::Star)
    }

    /// Returns whether the next token needs to be preceded by concatenation, if that token is not
    /// a symbol, or is the left parentheses.
    ///
    /// Beam and LParen shouldn't be concatenated, because they signal the start of a new string.
    /// The other token types may be a continuation of a string, so concatentation may be necessary.
    pub(crate) fn need_concat_next(&self) -> bool {
        !matches!(
            self,
            TokenType::LParen | TokenType::Beam 
        )
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
