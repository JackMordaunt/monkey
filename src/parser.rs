use crate::token::Token;
use crate::ast::{Program, Node};

use std::iter::Peekable;
use std::error::Error;
use std::cell::RefCell;

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

    pub fn parse(&mut self) -> Result<Program, Box<dyn Error>> {
        let mut nodes: Vec<Node> = vec![];
        loop {
            self.advance();
            if self.token() == Token::Eof {
                break;
            }
            let node = self.parse_statement()
                .map_err(|err| format!("statement: {}", err))?;
            nodes.push(node);
        }
        Ok(Program::new(nodes))
    }

    fn parse_statement(&mut self) -> Result<Node, Box<dyn Error>> {
        let node = match self.token() {
            Token::Let => self.parse_let_statement()?,
            _ => return Err("unimplemented token".into()),
        };
        Ok(node)
    }

    fn parse_let_statement(&mut self) -> Result<Node, Box<dyn Error>> {
        let name = match self.peek() {
            Some(Token::Ident(name)) => name,
            _ => return Err("invalid let statement".into()),
        };
        self.advance();
        match self.peek() {
            Some(Token::Assign) => {},
            Some(t) => return Err(format!("invalid let statement, expected {:?}, got {:?}", Token::Assign, t).into()),
            None => return Err(format!("invalid let statement, expected {:?}, got {:?}", Token::Assign, Token::Eof).into()),
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

    fn peek(&self) -> Option<Token> {
        let mut lexer = self.lexer.borrow_mut();
        match lexer.peek() {
            Some(token) => Some((*token).clone()),
            None => None,
        }
    }
}

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
}