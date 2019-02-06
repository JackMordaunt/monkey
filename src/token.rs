#![allow(dead_code)]

use std::iter::Peekable;

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

impl From<String> for Token {
    fn from(s: String) -> Token {
        match &s as &str {
            "=" => Token::Assign,
            "+" => Token::Plus,
            "(" => Token::LeftParen,
            ")" => Token::RightParen,
            "{" => Token::LeftBrace,
            "}" => Token::RightBrace,
            "," => Token::Comma,
            ";" => Token::Semicolon,
            "\0" => Token::Eof,
            "fn" => Token::Function,
            "let" => Token::Let,
            "return" => Token::Return,
            s if s.parse::<i32>().is_ok() => {
                Token::Int(s.parse().unwrap())
            }
            _ => {
                Token::Ident(s)
            }
        }
    }
}

struct Lexer<I>
    where I: Iterator<Item=char>,
{
    input: Peekable<I>,
    ch: char,
}

impl<I> Lexer<I>
    where I: Iterator<Item=char>,
{
    pub fn new(input: I) -> Lexer<I> {
        let mut l = Lexer {
            input: input.peekable(),
            ch: '\0',
        };
        l.next().unwrap();
        l
    }

    fn read<P>(&mut self, predicate: P) -> Token
        where P: Fn(&char) -> bool
    {
        let mut ident = self.ch.to_string();
        loop {
            let ch = match self.input.peek() {
                Some(ch) => ch,
                None => return Token::Eof,
            }; 
            if predicate(ch) {
                self.ch = match self.input.next() {
                    Some(ch) => ch,
                    None => return Token::Eof,
                };
                ident.push(self.ch);
            } else {
                break;
            }
        }
        Token::from(ident)
    }

    fn eat_space(&mut self) {
        while self.ch.is_whitespace() {
            self.ch = match self.input.next() {
                Some(ch) => ch,
                None => return,
            };
        }
    }
}

impl<I> Iterator for Lexer<I>
    where I: Iterator<Item=char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.eat_space();
        let tok = match self.ch {
            '=' => Token::Assign,
            '+' => Token::Plus,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '\0'=> Token::Eof,
            _ => {
                if self.ch.is_alphabetic() {
                    self.read(|c: &char| c.is_alphabetic())
                } else if self.ch.is_numeric() {
                    self.read(|c: &char| c.is_numeric())
                } else {
                    Token::Illegal(self.ch.to_string())
                }
            }
        };
        self.ch = match self.input.next() {
            Some(ch) => ch,
            None => '\0',
        };
        Some(tok)
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
            Token::Ident("five".to_owned()),
            Token::Comma,
            Token::Ident("ten".to_owned()),
            Token::RightParen,
            Token::Semicolon,
        ];
        let mut lexer = Lexer::new(input.chars());
        for token in tokens.into_iter() {
            assert_eq!(Some(token), lexer.next());
        }
    }
}