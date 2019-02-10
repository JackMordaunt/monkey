#![allow(dead_code)]
use crate::token::{Token, Kind};

use std::iter::Peekable;

impl Token {
    /// ident constructs the appropriate Token for the given multi-character
    /// word. 
    fn ident(word: String) -> Token {
        match &word as &str {
            "fn" => Token::new(Kind::Function, word),
            "let" => Token::new(Kind::Let, word),
            "return" => Token::new(Kind::Return, word),
            "if" => Token::new(Kind::If, word),
            "else" => Token::new(Kind::Else, word),
            "true" => Token::new(Kind::Bool, word),
            "false" => Token::new(Kind::Bool, word),
            word if word.parse::<i32>().is_ok() => {
                Token::new(Kind::Int, word)
            }
            _ => {
                Token::new(Kind::Ident, word)
            }
        }
    }
}

pub struct Lexer<I>
    where I: Iterator<Item=char>,
{
    input: Peekable<I>,
    ch: char,
}

impl<I> Lexer<I>
    where I: Iterator<Item=char>,
{
    pub fn new(input: I) -> Lexer<I> {
        Lexer {
            input: input.peekable(),
            ch: '\0',
        }
    }

    fn read<P>(&mut self, predicate: P) -> Token
        where P: Fn(&char) -> bool
    {
        let mut ident = self.ch.to_string();
        loop {
            let ch = match self.input.peek() {
                Some(ch) => ch,
                None => return Token::new(Kind::Eof, "\0"),
            }; 
            if predicate(ch) {
                self.ch = match self.input.next() {
                    Some(ch) => ch,
                    None => return Token::new(Kind::Eof, "\0"),
                };
                ident.push(self.ch);
            } else {
                break;
            }
        }
        Token::ident(ident)
    }

    fn eat_space(&mut self) {
        while self.ch.is_whitespace() {
            self.advance();
        }
    }

    fn advance(&mut self) {
        self.ch = match self.input.next() {
            Some(ch) => ch,
            None => '\0',
        };
    }
}

impl<I> Iterator for Lexer<I>
    where I: Iterator<Item=char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance();
        self.eat_space();
        let tok = match self.ch {
            '+' => Token::new(Kind::Plus, "+"),
            '(' => Token::new(Kind::LeftParen, "("),
            ')' => Token::new(Kind::RightParen, ")"),
            '{' => Token::new(Kind::LeftBrace, "{"),
            '}' => Token::new(Kind::RightBrace, "}"),
            ',' => Token::new(Kind::Comma, ","),
            ';' => Token::new(Kind::Semicolon, ";"),
            '-' => Token::new(Kind::Minus, "-"),
            '/' => Token::new(Kind::Slash, "/"),
            '<' => Token::new(Kind::ArrowLeft, "<"),
            '>' => Token::new(Kind::ArrowRight, ">"),
            '*' => Token::new(Kind::Asterisk, "*"),
            '\0' => return None,
            '=' => {
                match self.input.peek() {
                    Some(next) => {
                        if next == &'=' {
                            self.advance();
                            Token::new(Kind::Equal, "==")
                        } else {
                            Token::new(Kind::Assign, "=")
                        }
                    }
                    None => Token::new(Kind::Assign, "=")
                }
            },
            '!' => {
                match self.input.peek() {
                    Some(next) => {
                        if next == &'=' {
                            self.advance();
                            Token::new(Kind::NotEqual, "!=")
                        } else {
                            Token::new(Kind::Bang, "!")
                        }
                    }
                    None => Token::new(Kind::Bang, "!")
                }
            },
            _ => {
                if self.ch.is_alphabetic() {
                    self.read(|c: &char| c.is_alphabetic())
                } else if self.ch.is_numeric() {
                    self.read(|c: &char| c.is_numeric())
                } else {
                    Token::new(Kind::Illegal, self.ch.to_string())
                }
            }
        };
        Some(tok)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::diff;
    #[test]
    fn tokens() {
        let input: &'static str = r#"
            let five = 5;

            let ten = 10;
            
            let add = fn(a, b) {
                return a + b;
            };

            let result = add(five, ten);

            !-/*5;

            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            5 == 5;
            5 != 10;
        "#;
        let want = vec![
            Token::new(Kind::Let, "let"),
            Token::new(Kind::Ident, "five"),
            Token::new(Kind::Assign, "="),
            Token::new(Kind::Int, "5"),
            Token::new(Kind::Semicolon, ";"),

            Token::new(Kind::Let, "let"),
            Token::new(Kind::Ident, "ten"),
            Token::new(Kind::Assign, "="),
            Token::new(Kind::Int, "10"),
            Token::new(Kind::Semicolon, ";"),

            Token::new(Kind::Let, "let"),
            Token::new(Kind::Ident, "add"),
            Token::new(Kind::Assign, "="),
            Token::new(Kind::Function, "fn"),
            Token::new(Kind::LeftParen, "("),
            Token::new(Kind::Ident, "a"),
            Token::new(Kind::Comma, ","),
            Token::new(Kind::Ident, "b"),
            Token::new(Kind::RightParen, ")"),
            Token::new(Kind::LeftBrace, "{"),
            Token::new(Kind::Return, "return"),
            Token::new(Kind::Ident, "a"),
            Token::new(Kind::Plus, "+"),
            Token::new(Kind::Ident, "b"),
            Token::new(Kind::Semicolon, ";"),
            Token::new(Kind::RightBrace, "}"),
            Token::new(Kind::Semicolon, ";"),

            Token::new(Kind::Let, "let"),
            Token::new(Kind::Ident, "result"),
            Token::new(Kind::Assign, "="),
            Token::new(Kind::Ident, "add"),
            Token::new(Kind::LeftParen, "("),
            Token::new(Kind::Ident, "five"),
            Token::new(Kind::Comma, ","),
            Token::new(Kind::Ident, "ten"),
            Token::new(Kind::RightParen, ")"),
            Token::new(Kind::Semicolon, ";"),

            Token::new(Kind::Bang, "!"),
            Token::new(Kind::Minus, "-"),
            Token::new(Kind::Slash, "/"),
            Token::new(Kind::Asterisk, "*"),
            Token::new(Kind::Int, "5"),
            Token::new(Kind::Semicolon, ";"),

            Token::new(Kind::Int, "5"),
            Token::new(Kind::ArrowLeft, "<"),
            Token::new(Kind::Int, "10"),
            Token::new(Kind::ArrowRight, ">"),
            Token::new(Kind::Int, "5"),
            Token::new(Kind::Semicolon, ";"),

            Token::new(Kind::If, "if"),
            Token::new(Kind::LeftParen, "("),
            Token::new(Kind::Int, "5"),
            Token::new(Kind::ArrowLeft, "<"),
            Token::new(Kind::Int, "10"),
            Token::new(Kind::RightParen, ")"),
            Token::new(Kind::LeftBrace, "{"),
            Token::new(Kind::Return, "return"),
            Token::new(Kind::Bool, "true"),
            Token::new(Kind::Semicolon, ";"),
            Token::new(Kind::RightBrace, "}"),
            Token::new(Kind::Else, "else"),
            Token::new(Kind::LeftBrace, "{"),
            Token::new(Kind::Return, "return"),
            Token::new(Kind::Bool, "false"),
            Token::new(Kind::Semicolon, ";"),
            Token::new(Kind::RightBrace, "}"),

            Token::new(Kind::Int, "5"),
            Token::new(Kind::Equal, "=="),
            Token::new(Kind::Int, "5"),
            Token::new(Kind::Semicolon, ";"),

            Token::new(Kind::Int, "5"),
            Token::new(Kind::NotEqual, "!="),
            Token::new(Kind::Int, "10"),
            Token::new(Kind::Semicolon, ";"),
        ];
        let got: Vec<Token> = Lexer::new(input.chars()).collect();
        if want.len() != got.len() {
            panic!("want={:?} \ngot={:?} \ndiff={:?}", want, got, diff(&want, &got));
        }
        for (ii, token) in want.into_iter().enumerate() {
            assert_eq!(token, got[ii]);
        }
    }
}