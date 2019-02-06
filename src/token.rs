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
}

impl Token {
    /// ident constructs the appropriate Token for the given multi-character
    /// word. 
    fn ident(word: &str) -> Token {
        match word {
            "fn" => Token::Function,
            "let" => Token::Let,
            "return" => Token::Return,
            word if word.parse::<i32>().is_ok() => {
                Token::Int(word.parse().unwrap())
            }
            _ => {
                Token::Ident(word.to_owned())
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
        Token::ident(&ident)
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
            '=' => Token::Assign,
            '+' => Token::Plus,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '!' => Token::Bang,
            '-' => Token::Minus,
            '/' => Token::Slash,
            '<' => Token::ArrowLeft,
            '>' => Token::ArrowRight,
            '*' => Token::Asterisk,
            '\0'=> return None,
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
        Some(tok)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        "#;
        let want = vec![
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

            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,

            Token::Int(5),
            Token::ArrowLeft,
            Token::Int(10),
            Token::ArrowRight,
            Token::Int(5),
            Token::Semicolon,
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

type Diff<'a, 'b, T> = Vec<(usize, Option<&'a T>, Option<&'b T>)>;

fn diff<'a, 'b, T>(left: &'a [T], right: &'b [T]) -> Diff<'a, 'b, T>
    where T: PartialEq
{
    let mut diff = vec![];
    let min = std::cmp::min(left.len(), right.len());
    for ii in 0..min {
        if &left[ii] != &right[ii] {
            diff.push((ii, Some(&left[ii]), Some(&right[ii])));
        }
    }
    if left.len() > right.len() {
        for ii in 0..left.len()-right.len() {
            diff.push((min+ii, Some(&left[min+ii]), None))
        }
    }
    if left.len() < right.len() {
        for ii in 0..right.len()-left.len() {
            diff.push((min+ii, None, Some(&right[min+ii])))
        }
    }
    return diff;
}