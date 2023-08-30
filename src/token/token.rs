// Define an enum for different token types
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Illegal,
    EOF,
    // Identifiers + literals
    Ident(String), // add, foobar, x, y, ...
    Int(String),   // 1343456
    // Operators
    Assign,
    Plus,
    Bang,
    Minus,
    Slash,
    Asterisk,
    LT,
    GT,
    Eq,
    NotEq,
    LTE,
    GTE,
    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    // Keywords
    Let,
    Function,
    If,
    Else,
    Return,
    True,
    False,
}

// Token represents a token to be parsed
#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new<T: ToString>(kind: TokenType, stringer: T) -> Self {
        Self {
            kind,
            literal: stringer.to_string(),
        }
    }

    /// set_literal_str sets the literal field in the Token struct
    pub fn set_literal_str(&mut self, s: String) {
        self.literal = s;
    }

    /// set_kind sets the kind field in the Token struct
    pub fn set_kind(&mut self, t: TokenType) {
        self.kind = t;
    }
}
