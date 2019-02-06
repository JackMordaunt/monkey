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
    Illegal(String),
    Eof,

    Ident(String),
    Int(i32),

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
            _ => Token::Illegal(ch.to_string()),
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
            Token::Ident("five".to_owned()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,

            Token::Let,
            Token::Ident("ten".to_owned()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,

            Token::Let,
            Token::Ident("add".to_owned()),
            Token::Assign,
            Token::Function,
            Token::LeftParen,
            Token::Ident("a".to_owned()),
            Token::Comma,
            Token::Ident("b".to_owned()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::Ident("a".to_owned()), 
            Token::Plus,
            Token::Ident("b".to_owned()),
            Token::Semicolon,
            Token::RightBrace,
            Token::Semicolon,

            Token::Let,
            Token::Ident("result".to_owned()),
            Token::Assign,
            Token::Ident("add".to_owned()),
            Token::LeftParen,
            Token::Ident("a".to_owned()),
            Token::Comma,
            Token::Ident("b".to_owned()),
            Token::RightParen,
            Token::Semicolon,
        ];
        let mut lexer = Lexer::new(input.chars());
        for token in tokens.into_iter() {
            assert_eq!(Some(token), lexer.next());
        }
    }
}