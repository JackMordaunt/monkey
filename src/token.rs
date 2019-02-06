// Sample monkey to lex: 
//
// let five = 5; let ten = 10;
//
// let add = fn(x, y) {
//      x + y;
// };
//
// let result = add(five, ten);
//


// ILLEGAL = "ILLEGAL" EOF = "EOF"
// // Identifiers + literals
// IDENT = "IDENT" // add, foobar, x, y, ...
// INT = "INT"
// // Operators
// ASSIGN   = "="
// PLUS     = "+"
// // Delimiters
// COMMA     = ","
// SEMICOLON = ";"
// LPAREN = "("
// RPAREN = ")"
// LBRACE = "{"
// RBRACE = "}"
// // Keywords
//        FUNCTION = "FUNCTION"
// LET = "LET"

#[derive(Eq, PartialEq, Debug)]
enum Token {
    Illegal,
    Eof,

    Ident(String),
    Int,

    Assign,
    Plus,

    Comma,
    Semicolon,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Function,
    Let,
    Return,
}

impl From<char> for Token {
    fn from(ch: char) -> Token {
        match ch {
            '=' => Token::Assign,
            '+' => Token::Plus,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '\0'=> Token::Eof,
            _ => Token::Illegal,
        }
    }
}

struct Lexer<I>
    where I: Iterator<Item=char>,
{
    input: I,
    // position: u32,      // Current position.
    // read_position: u32, // Next position.
    // ch: Option<char>,   // Char at current position.
}

impl<I> Lexer<I>
    where I: Iterator<Item=char>,
{
    fn new(input: I) -> Lexer<I> {
        Lexer {
            input,
        }
    }
}

impl<I> Iterator for Lexer<I>
    where I: Iterator<Item=char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.input.next() {
            Some(ch) => Some(Token::from(ch)),
            None => Some(Token::Eof),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_char_tokens() {
        let input: &'static str = "=+(){},;";
        let tokens = vec![
            Token::Assign, 
            Token::Plus, 
            Token::LeftParen, 
            Token::RightParen,
            Token::LeftBrace, 
            Token::RightBrace, 
            Token::Comma, 
            Token::Semicolon, 
            Token::Eof,
        ];
        let mut lexer = Lexer::new(input.chars());
        for token in tokens.into_iter() {
            assert_eq!(Some(token), lexer.next());
        }
    }

    #[test]
    fn multi_char_tokens() {
        let input: &'static str = r#"
            let five = 5;

            let ten = 10;

            let add = fn(a, b) {
                return a + b;
            };

            let result = add(five, ten);
        "#;
        let tokens = vec![
            Token::Let,
            Token::Ident,
            Token::Assign,
            Token::Int,
            Token::Semicolon,

            Token::Let,
            Token::Ident,
            Token::Assign,
            Token::Int,
            Token::Semicolon,

            Token::Let,
            Token::Ident,
            Token::Assign,
            Token::Function,
            Token::LeftParen,
            Token::Ident,
            Token::Comma,
            Token::Ident,
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::Ident, 
            Token::Plus,
            Token::Ident,
            Token::Semicolon,
            Token::RightBrace,
            Token::Semicolon,

            Token::Let,
            Token::Ident,
            Token::Assign,
            Token::Ident,
            Token::LeftParen,
            Token::Ident,
            Token::Comma,
            Token::Ident,
            Token::RightParen,
            Token::Semicolon,
        ];
        let mut lexer = Lexer::new(input.chars());
        for token in tokens.into_iter() {
            assert_eq!(Some(token), lexer.next());
        }
    }
}