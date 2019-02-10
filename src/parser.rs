use crate::token::{Token, Kind};
use crate::ast::{Program, Node};
use crate::util::MultiError;

use std::iter::Peekable;
use std::cell::RefCell;

type Error = Box<dyn std::error::Error>;

/// Parser transforms a stream of tokens into an AST for the monkey language.
pub struct Parser<Lexer>
    where Lexer: Iterator<Item=Token>,
{
    lexer: RefCell<Peekable<Lexer>>,
    token: RefCell<Token>,
}

impl<Lexer> Parser<Lexer>
    where Lexer: Iterator<Item=Token>,
{
    // new constructs a parser. 
    pub fn new(lexer: Lexer) -> Parser<Lexer> {
        Parser {
            lexer: RefCell::new(lexer.peekable()),
            token: RefCell::new(Token::new(Kind::Illegal, "")),
        }
    }

    pub fn parse(&mut self) -> Result<Program, Error> {
        let mut nodes: Vec<Node> = vec![];
        let mut errors: MultiError = MultiError::new();
        loop {
            self.advance();
            if self.token().kind == Kind::Eof {
                break;
            }
            match self.parse_statement() {
                Ok(node) => nodes.push(node),
                Err(err) => errors.push(err),
            }
        }
        if errors.len() > 0 {
            Err(Box::new(errors))
        } else {
            Ok(Program::new(nodes))
        }
    }

    fn parse_statement(&mut self) -> Result<Node, Error> {
        let node = match self.token().kind {
            Kind::Let => self.parse_let_statement()
                .map_err(|err| format!("parsing 'let' statement: {}", err))?,
            Kind::Return => self.parse_return_statement()
                .map_err(|err| format!("parsing 'return' statement: {}", err))?,
            _ => return Err("unimplemented token".into()),
        };
        Ok(node)
    }

    fn parse_let_statement(&mut self) -> Result<Node, Error> {
        let name = self.peek(Kind::Ident)?.literal;
        self.advance();
        self.peek(Kind::Assign)?;
        // Note: Skipping expression parsing for the moment.
        while self.token().kind != Kind::Semicolon {
            self.advance();
        }
        Ok(Node::Let{name: name, value: Box::new(Node::Placeholder)})
    }

    fn parse_return_statement(&mut self) -> Result<Node, Error> {

        Err("oof".into())
    }

    fn advance(&self) {
        let mut token = self.token.borrow_mut();
        let mut lexer = self.lexer.borrow_mut();
        *token = match lexer.next() {
            Some(token) => token,
            None => Token::new(Kind::Eof, "\0"),
        };
    }

    fn token(&self) -> Token {
        self.token.borrow().clone()
    }

    fn peek(&self, kind: Kind) -> Result<Token, Error> {
        let mut lexer = self.lexer.borrow_mut();
        match lexer.peek() {
            Some(t) => {
                if t.kind == kind {
                    Ok((*t).clone())
                } else {
                    Err(format!("expected {:?}, got {:?}", kind, t.kind).into())
                }
            },
            None => Err(format!("expected {:?}, got {:?}", kind, Kind::Eof).into()),
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