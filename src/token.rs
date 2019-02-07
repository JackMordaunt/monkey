#[derive(Eq, PartialEq, Debug)]
pub enum Token {
    Illegal(String),
    Eof,

    Ident(String),
    Int(i32),
    Bool(bool),

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