use crate::token::Token;
use crate::ast::{Program, Node};

use std::iter::Peekable;
use std::error::Error as StdError;
use std::cell::RefCell;
use std::fmt;

/// Parser transforms a stream of tokens into an AST for the monkey language.
pub struct Parser<Lexer>
where
    Lexer: Iterator<Item=Token>,
{
    lexer: RefCell<Peekable<Lexer>>,
    token: RefCell<Token>,
}

impl<Lexer> Parser<Lexer>
where
    Lexer: Iterator<Item=Token>,
{
    // new constructs a parser. 
    pub fn new(lexer: Lexer) -> Parser<Lexer> {
        Parser {
            lexer: RefCell::new(lexer.peekable()),
            token: RefCell::new(Token::Illegal("".to_owned())),
        }
    }

    pub fn parse(&mut self) -> Result<Program, Box<dyn StdError>> {
        let mut nodes: Vec<Node> = vec![];
        let mut errors: Vec<Box<dyn StdError>> = vec![];
        loop {
            self.advance();
            if self.token() == Token::Eof {
                break;
            }
            match self.parse_statement() {
                Ok(node) => nodes.push(node),
                Err(err) => errors.push(err),
            };
        }
        if errors.len() > 0 {
            let error = errors
                .into_iter()
                .fold(String::new(), |mut acc, err| {
                    acc.extend(format!("\n{}", err).chars()); acc
                })
                .into();
            Err(error)
        } else {
            Ok(Program::new(nodes))
        }
    }

    fn parse_statement(&mut self) -> Result<Node, Box<dyn StdError>> {
        let node = match self.token() {
            Token::Let => self.parse_let_statement()?,
            _ => return Err("unimplemented token".into()),
        };
        Ok(node)
    }

    fn parse_let_statement(&self) -> Result<Node, Box<dyn StdError>> {
        let name = match self.peek(Token::Ident)? {
            Token::Ident(name) => name,
        };
        self.advance();
        match self.peek() {
            Some(Token::Assign) => {},
            _ => return Err(Box::new(Error::Peek { want: Token::Assign, got: self.peek() })),
        };
        // Note: Skipping expression parsing for the moment.
        while self.token() != Token::Semicolon {
            self.advance();
        }
        Ok(Node::Let{name: name.to_owned(), value: Box::new(Node::Placeholder)})
    }

    fn advance(&self) {
        let mut token = self.token.borrow_mut();
        let mut lexer = self.lexer.borrow_mut();
        *token = match lexer.next() {
            Some(token) => token,
            None => Token::Eof,
        };
    }

    fn token(&self) -> Token {
        self.token.borrow().clone()
    }

    fn peek(&self, token: std::mem::Discriminant<Token>) -> Result<Token, Error> {
        let mut lexer = self.lexer.borrow_mut();
        let peek = match lexer.peek() {
            Some(peek) => peek,
            None => return Err(Error::Peek { want: token, got: None }),
        };
        if std::mem::discriminant(peek) != token {
            Err(Error::Peek { want: token, got: Some(peek.clone()) })
        } else {
            Ok(peek.clone())
        }
    }
}

#[derive(Debug)]
enum Error {
    Peek { want: std::mem::Discriminant<Token>, got: Option<Token> },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Error::Peek { want, got } => {
                format!("want {:?}, got {:?}", want, got.clone().unwrap_or(Token::Eof))
            }
        };
        write!(f, "{}", msg)
    }
}

// Note: Since Display and Debug are implemented, all we need to do is "opt-in"
// to implement trait `std::error::Error`.
impl StdError for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::ast::Node;
    use crate::util::diff;

    #[test]
    fn let_statement() {
        let input: &'static str = r#"
            let five = 5;
            let ten = 10;
        "#;
        let want = vec![
            Node::Let { name: "five".to_string(), value: Box::new(Node::Placeholder) },
            Node::Let { name: "ten".to_string(), value: Box::new(Node::Placeholder) },
            // Node::Let { name: "five".to_string(), value: Box::new(Node::Int(5)) },
            // Node::Let { name: "ten".to_string(), value: Box::new(Node::Int(10)) },
        ];
        let mut parser = Parser::new(Lexer::new(input.chars()));
        let Program { statements } = parser.parse()
            .map_err(|err| format!("parsing: {}", err))
            .unwrap();

        assert_eq!(want.len(), statements.len());
        let diffs = diff(&want, &statements);
        if diffs.len() > 0 {
            panic!("diff: {:?}", diff(&want, &statements));
        }
    }

    #[test]
    fn error_out() {
        let input: &'static str = r#"
            let 123 what = 12!3;
            let ;; ?;
            let foo = whadya want??;
        "#;
        // let want = vec![

        // ];
        match Parser::new(Lexer::new(input.chars())).parse() {
            Ok(_) => panic!("expect errors, got none"),
            Err(err) => panic!("got error: {}", err),
        };
    }
}