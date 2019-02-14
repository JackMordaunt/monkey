use crate::token::{Token, Kind};
use crate::ast::{Program, Node, Precedence, Prefix, Infix};
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
            Kind::Let => {
                self.parse_let_statement()
                    .map_err(|err| format!("'let' statement: {}", err))?
            },
            Kind::Return => {
                self.parse_return_statement()
                    .map_err(|err| format!("'return' statement: {}", err))?
            },
            _ => {
                self.parse_expression_statement()
                    .map_err(|err| format!("expression statement: {}", err))?
            },
        };
        Ok(node)
    }

    fn parse_let_statement(&mut self) -> Result<Node, Error> {
        let name = self.expect(Kind::Ident)?.literal;
        self.advance();
        self.expect(Kind::Assign)?;
        // Note: Skipping expression parsing for the moment.
        while self.token().kind != Kind::Semicolon {
            self.advance();
        }
        Ok(Node::Let{name: name, value: Box::new(Node::Placeholder)})
    }

    fn parse_return_statement(&mut self) -> Result<Node, Error> {
        self.advance();
        while self.token().kind != Kind::Semicolon {
            self.advance();
        }
        Ok(Node::Return { value: Box::new(Node::Placeholder) })
    }

    fn parse_expression_statement(&mut self) -> Result<Node, Error> {
        let exp = self.parse_expression(Precedence::Lowest)?;
        if self.expect(Kind::Semicolon).is_ok() {
            self.advance();
        }
        Ok(exp)
    }

    fn parse_expression(&mut self, p: Precedence) -> Result<Node, Error> {
        let mut left = self.parse_prefix()?;
        while !self.expect(Kind::Semicolon).is_ok() && p < Precedence::from(self.peek()?.kind) {
            self.advance();
            left = self.parse_infix(left)?;
        }
        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Node, Error> {
        let token = self.token();
        let node = match token.kind {
            Kind::Ident => {
                Node::Identifier {
                    value: token.literal,
                }
            }
            Kind::Int => {
                Node::Int(token.literal.parse()?)
            }
            Kind::Bang => {
                self.advance();
                Node::Prefix {
                    operator: Prefix::Not,
                    value: Box::new(self.parse_expression(Precedence::Prefix)?),
                }
            }
            Kind::Minus => {
                self.advance();
                Node::Prefix {
                    operator: Prefix::Negative,
                    value: Box::new(self.parse_expression(Precedence::Prefix)?),
                }
            }
            _ => {
                return Err(format!("prefix: unimplemented: {:?}", token).into());
            }
        };
        Ok(node)
    }

    fn parse_infix(&mut self, left: Node) -> Result<Node, Error> {
        let token = self.token();
        let node = match token.kind {
            Kind::Plus => {
                self.advance();
                Node::Infix {
                    left: Box::new(left),
                    operator: Infix::Add,
                    right: Box::new(self.parse_expression(Precedence::from(token.kind))?),
                }
            }
            Kind::Minus => {
                self.advance();
                Node::Infix {
                    left: Box::new(left),
                    operator: Infix::Subtract,
                    right: Box::new(self.parse_expression(Precedence::from(token.kind))?),
                }
            }
            Kind::Slash => {
                self.advance();
                Node::Infix {
                    left: Box::new(left),
                    operator: Infix::Divide,
                    right: Box::new(self.parse_expression(Precedence::from(token.kind))?),
                }
            }
            Kind::Asterisk => {
                self.advance();
                Node::Infix {
                    left: Box::new(left),
                    operator: Infix::Multiply,
                    right: Box::new(self.parse_expression(Precedence::from(token.kind))?),
                }
            }
            Kind::Equal => {
                self.advance();
                Node::Infix {
                    left: Box::new(left),
                    operator: Infix::Eq,
                    right: Box::new(self.parse_expression(Precedence::from(token.kind))?),
                }
            }
            Kind::NotEqual => {
                self.advance();
                Node::Infix {
                    left: Box::new(left),
                    operator: Infix::NotEq,
                    right: Box::new(self.parse_expression(Precedence::from(token.kind))?),
                }
            }
            Kind::ArrowLeft => {
                self.advance();
                Node::Infix {
                    left: Box::new(left),
                    operator: Infix::LessThan,
                    right: Box::new(self.parse_expression(Precedence::from(token.kind))?),
                }
            }
            Kind::ArrowRight => {
                self.advance();
                Node::Infix {
                    left: Box::new(left),
                    operator: Infix::GreaterThan,
                    right: Box::new(self.parse_expression(Precedence::from(token.kind))?),
                }
            }
            _ => {
                return Err(format!("infix: unimplemented for {:?}", token).into());
            }
        };
        Ok(node)
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

    fn expect(&self, kind: Kind) -> Result<Token, Error> {
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

    fn peek(&self) -> Result<Token, Error> {
        let mut lexer = self.lexer.borrow_mut();
        match lexer.peek() {
            Some(t) => Ok((*t).clone()),
            None => Err(format!("unexpected EOF").into()),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
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
    fn return_statement() {
        let input: &'static str = r#"
            return a + b;
            return 10;
            return "oof";
        "#;
        let want = vec![
            Node::Return { value: Box::new(Node::Placeholder) },
            Node::Return { value: Box::new(Node::Placeholder) },
            Node::Return { value: Box::new(Node::Placeholder) },
        ];
        let mut parser = Parser::new(Lexer::new(input.chars()));
        match parser.parse() {
            Ok(Program { statements }) => {
                assert_eq!(want.len(), statements.len());
                let diffs = diff(&want, &statements);
                if diffs.len() > 0 {
                    panic!("diff: {:?}", diff(&want, &statements));
                }
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    #[test]
    fn expressions() {
        let input: &'static str = r#"
            foo;
            5;
        "#;
        let want = vec![
            Node::Identifier { value: "foo".to_owned() },
            Node::Int(5),
        ];
        let mut parser = Parser::new(Lexer::new(input.chars()));
        match parser.parse() {
            Ok(Program { statements }) => {
                assert_eq!(want.len(), statements.len());
                let diffs = diff(&want, &statements);
                if diffs.len() > 0 {
                    panic!("diff: {:?}", diff(&want, &statements));
                }
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    #[test]
    fn prefix() {
        let input: &'static str = r#"
            !foo;
            -5;
        "#;
        let want = vec![
            Node::Prefix { operator: Prefix::Not, value: Box::new(Node::Identifier { value: "foo".to_owned() } ) },
            Node::Prefix { operator: Prefix::Negative, value: Box::new(Node::Int(5)) },
        ];
        let mut parser = Parser::new(Lexer::new(input.chars()));
        match parser.parse() {
            Ok(Program { statements }) => {
                assert_eq!(want.len(), statements.len());
                let diffs = diff(&want, &statements);
                if diffs.len() > 0 {
                    panic!("diff: {:?}", diff(&want, &statements));
                }
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    #[test]
    fn infix() {
        let input: &'static str = r#"
            5 + 5;
            5 - 5;
            5 * 5;
            5 / 5;
            5 > 5;
            5 < 5;
            5 == 5;
            5 != 5;
        "#;
        let want = vec![
            Node::Infix {
                left: Box::new(Node::Int(5)),
                operator: Infix::Add,
                right: Box::new(Node::Int(5)),
            },
            Node::Infix {
                left: Box::new(Node::Int(5)),
                operator: Infix::Subtract,
                right: Box::new(Node::Int(5)),
            },
            Node::Infix {
                left: Box::new(Node::Int(5)),
                operator: Infix::Multiply,
                right: Box::new(Node::Int(5)),
            },
            Node::Infix {
                left: Box::new(Node::Int(5)),
                operator: Infix::Divide,
                right: Box::new(Node::Int(5)),
            },
            Node::Infix {
                left: Box::new(Node::Int(5)),
                operator: Infix::GreaterThan,
                right: Box::new(Node::Int(5)),
            },
            Node::Infix {
                left: Box::new(Node::Int(5)),
                operator: Infix::LessThan,
                right: Box::new(Node::Int(5)),
            },
            Node::Infix {
                left: Box::new(Node::Int(5)),
                operator: Infix::Eq,
                right: Box::new(Node::Int(5)),
            },
            Node::Infix {
                left: Box::new(Node::Int(5)),
                operator: Infix::NotEq,
                right: Box::new(Node::Int(5)),
            },
        ];
        let mut parser = Parser::new(Lexer::new(input.chars()));
        match parser.parse() {
            Ok(Program { statements }) => {
                assert_eq!(want.len(), statements.len());
                let diffs = diff(&want, &statements);
                if diffs.len() > 0 {
                    panic!("diff: {:?}", diff(&want, &statements));
                }
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    #[test]
    fn precedence() -> Result<(), Error> {
        let tests = vec![
            ("-a * b;", "((-a) * b)"),
            ("!-a;", "(!(-a))"),
            ("a + b * c;", "(a + (b * c))"),
            ("a * b * c;", "((a * b) * c)"),
            ("a + b / c;", "(a + (b / c))"),
            ("a / b / c;", "((a / b) / c)"),
            ("5 > 4 == 3 < 4;", "((5 > 4) == (3 < 4))"),
            ("a + b * c + d / e - f;", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5;", "(3 + 4)((-5) * 5)"),
            ("5 < 4 != 3 > 4;", "((5 < 4) != (3 > 4))"),
            ("3 + 4 * 5 == 3 * 1 + 4 * 5;", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
        ];
        for (ii, test) in tests.iter().enumerate() {
            let program = Parser::new(Lexer::new(test.0.chars())).parse()
                .map_err(|err| format!("{}: {}", ii, err))?;
            println!("{} \nInput {:#?} \n Want {:#?} \n  Got {:#?} \n{:#?}", &ii, &test.0, &test.1, program.to_string(),  &program.statements);
            assert_eq!(program.to_string(), test.1)
        }
        Ok(())
    }
}