#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Token {
    pub kind: Kind,
    pub literal: String,
}

impl Token {
    pub fn new<S: Into<String>>(kind: Kind, literal: S) -> Token {
        Token { kind, literal: literal.into() }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Kind {
    Illegal,
    Eof,

    Ident,
    Int,
    Bool,

    Assign,
    Plus,

    Comma,
    Semicolon,
    Bang,
    Minus,
    Slash,
    Asterisk,
    ArrowLeft,
    ArrowRight,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Function,
    Let,
    Return,
    If,
    Else,

    Equal,
    NotEqual,
}