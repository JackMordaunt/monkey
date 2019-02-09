// TODO(jfm): Implement each value based variant as a struct and wrap it in the 
// variant, so that I can treat the enum separate from the value. 
// Fundamentall I want to distinguish between a token's type (or kind) and a
// token's value.
#[derive(Eq, PartialEq, Debug, Clone)]
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

impl Token {
    pub fn kind(&self) -> std::mem::Discriminant<Token> {
        std::mem::discriminant(self)
    }
}