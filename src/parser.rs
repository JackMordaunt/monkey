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
            Kind::Bool => {
                Node::Boolean(token.literal.parse()?)
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
            Kind::If => {
                self.expect(Kind::LeftParen)?;
                self.advance();
                self.advance();
                let predicate = self.parse_expression(Precedence::Lowest)?;
                self.expect(Kind::RightParen)?;
                self.advance();
                self.expect(Kind::LeftBrace)?;
                self.advance();
                let success = self.parse_block()?;
                if self.expect(Kind::Else).is_ok() {
                    self.advance();
                    self.expect(Kind::LeftBrace)?;
                    self.advance();
                    let failure = self.parse_block()?;
                    Node::If {
                        predicate: Box::new(predicate),
                        success: Box::new(success),
                        fail: Some(Box::new(failure)),
                    }
                } else {
                    Node::If {
                        predicate: Box::new(predicate),
                        success: Box::new(success),
                        fail: None,
                    }
                }
            }
            Kind::Function => {
                self.expect(Kind::LeftParen)?;
                self.advance();
                let mut params = vec![];
                while self.expect(Kind::Ident).is_ok() {
                    self.advance();
                    params.push(Node::Identifier {
                        value: self.token().literal,
                    });
                    if self.expect(Kind::Comma).is_err() {
                        break;
                    }
                    self.advance();
                }
                self.expect(Kind::RightParen)?;
                self.advance();
                self.advance();
                let body = self.parse_block()?;
                Node::Function {
                    parameters: params,
                    body: Box::new(body),
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
            Kind::LeftParen => {
                let function = Box::new(left.clone());
                let mut arguments = vec![];
                if self.expect(Kind::RightParen).is_ok() {
                    self.advance();
                    Node::Call {
                        function,
                        arguments,
                    }                    
                } else {
                    self.advance();
                    arguments.push(self.parse_expression(Precedence::Lowest)?);
                    while self.expect(Kind::Comma).is_ok() {
                        self.advance();
                        self.advance();
                        arguments.push(self.parse_expression(Precedence::Lowest)?);
                    }
                    self.expect(Kind::RightParen)?;
                    self.advance();
                    Node::Call {
                        function,
                        arguments,
                    }
                }
            }
            _ => {
                return Err(format!("infix: unimplemented for {:?}", token).into());
            }
        };
        Ok(node)
    }

    fn parse_block(&mut self) -> Result<Node, Error> {
        self.advance();
        let mut statements = vec![];
        while self.token().kind != Kind::RightBrace && self.token().kind != Kind::Eof {
            statements.push(self.parse_statement()?);
            self.advance();
        }
        let block = Node::Block(statements);
        Ok(block)
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
            !true;
            !false;
        "#;
        let want = vec![
            Node::Prefix { operator: Prefix::Not, value: Box::new(Node::Identifier { value: "foo".to_owned() } ) },
            Node::Prefix { operator: Prefix::Negative, value: Box::new(Node::Int(5)) },
            Node::Prefix { operator: Prefix::Not, value: Box::new(Node::Boolean(true)) },
            Node::Prefix { operator: Prefix::Not, value: Box::new(Node::Boolean(false)) },
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
    fn infix() -> Result<(), Error> {
        let tests = vec![
            (
                "5 + 5;",
                Node::Infix {
                    left: Box::new(Node::Int(5)),
                    operator: Infix::Add,
                    right: Box::new(Node::Int(5)),
                },
            ),
            (
                "5 - 5;",
                Node::Infix {
                    left: Box::new(Node::Int(5)),
                    operator: Infix::Subtract,
                    right: Box::new(Node::Int(5)),
                },
            ),
            (
                "5 * 5;",
                Node::Infix {
                    left: Box::new(Node::Int(5)),
                    operator: Infix::Multiply,
                    right: Box::new(Node::Int(5)),
                },
            ),
            (
                "5 / 5;",
                Node::Infix {
                    left: Box::new(Node::Int(5)),
                    operator: Infix::Divide,
                    right: Box::new(Node::Int(5)),
                },
            ),
            (
                "5 > 5;",
                Node::Infix {
                    left: Box::new(Node::Int(5)),
                    operator: Infix::GreaterThan,
                    right: Box::new(Node::Int(5)),
                },
            ),
            (
                "5 < 5;",
                Node::Infix {
                    left: Box::new(Node::Int(5)),
                    operator: Infix::LessThan,
                    right: Box::new(Node::Int(5)),
                },
            ),
            (
                "5 == 5;",
                Node::Infix {
                    left: Box::new(Node::Int(5)),
                    operator: Infix::Eq,
                    right: Box::new(Node::Int(5)),
                },
            ),
            (
                "5 != 5;",
                Node::Infix {
                    left: Box::new(Node::Int(5)),
                    operator: Infix::NotEq,
                    right: Box::new(Node::Int(5)),
                },
            ),
            (
                "true != false;",
                Node::Infix {
                    left: Box::new(Node::Boolean(true)),
                    operator: Infix::NotEq,
                    right: Box::new(Node::Boolean(false)),
                },
            ),
            (
                "true == true;",
                Node::Infix {
                    left: Box::new(Node::Boolean(true)),
                    operator: Infix::Eq,
                    right: Box::new(Node::Boolean(true)),
                },
            ),
            (
                "false == false;",
                Node::Infix {
                    left: Box::new(Node::Boolean(false)),
                    operator: Infix::Eq,
                    right: Box::new(Node::Boolean(false)),
                },
            ),
        ];
        for (ii, test) in tests.iter().enumerate() {
            let Program { statements } = Parser::new(Lexer::new(test.0.chars())).parse()
                .map_err(|err| format!("{}: {}", ii, err))?;
            if statements.len() != 1 {
                panic!("wrong number of statements: want 1, got {}", statements.len());
            }
            let got = &statements[0];
            if *got != test.1 {
                panic!("{}: want {}, got {}, input: {}", ii, test.1, got, test.0);
            }
        }
        Ok(())
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
            ("true;", "true"),
            ("false;", "false"),
            ("3 > 5 == false;", "((3 > 5) == false)"),
            ("3 < 5 == true;", "((3 < 5) == true)"),
        ];
        for (ii, test) in tests.iter().enumerate() {
            let program = Parser::new(Lexer::new(test.0.chars())).parse()
                .map_err(|err| format!("{}: {}", ii, err))?;
            if program.to_string() != test.1 {
                println!("{} \nInput {:#?} \n Want {:#?} \n  Got {:#?} \n{:#?}",
                    &ii, &test.0, &test.1, program.to_string(), &program.statements);
                assert_eq!(program.to_string(), test.1)
            }
        }
        Ok(())
    }

    #[test]
    fn if_expression() -> Result<(), Error> {
        let input = "if (x < y) { x };";
        let want = Node::If {
            predicate: Box::new(Node::Infix {
                left: Box::new(Node::Identifier { value: "x".into() }),
                operator: Infix::LessThan,
                right: Box::new(Node::Identifier { value: "y".into() }),
            }),
            success: Box::new(Node::Block(vec![Node::Identifier {value: "x".into() }])),
            fail: None,
        };
        let program = Parser::new(Lexer::new(input.chars())).parse()
            .map_err(|err| format!("parsing if statement: {}", err))?;
        println!("{}", program);
        println!("{}", want);
        assert!(program.statements.len() == 1);
        assert!(program.statements[0] == want);
        Ok(())
    }

    #[test]
    fn if_else_expression() -> Result<(), Error> {
        let input = "if (x < y) { x } else { y };";
        let want = Node::If {
            predicate: Box::new(Node::Infix {
                left: Box::new(Node::Identifier { value: "x".into() }),
                operator: Infix::LessThan,
                right: Box::new(Node::Identifier { value: "y".into() }),
            }),
            success: Box::new(Node::Block(vec![Node::Identifier {value: "x".into() }])),
            fail: Some(Box::new(Node::Block(vec![Node::Identifier {value: "y".into() }]))),
        };
        let program = Parser::new(Lexer::new(input.chars())).parse()
            .map_err(|err| format!("parsing if statement: {}", err))?;
        println!("{}", program);
        println!("{}", want);
        assert!(program.statements.len() == 1);
        assert!(program.statements[0] == want);
        Ok(())
    }

    #[test]
    fn function_literal() -> Result<(), Error> {
        let tests = vec![
            (
                "fn(x, y) { return x + y; };",
                Node::Function {
                    parameters: vec![
                        Node::Identifier { value: "x".into() },
                        Node::Identifier { value: "y".into() },
                    ],
                    body: Box::new(Node::Block(vec![
                        Node::Return {
                            value: Box::new(Node::Placeholder),
                            // value: Box::new(Node::Infix {
                            //     left: Box::new(Node::Identifier { value: "x".into() }),
                            //     operator: Infix::Add,
                            //     right: Box::new(Node::Identifier { value: "y".into() }),
                            // }),
                        },
                    ])),
                }
            ),
            (
                "fn() {};",
                Node::Function {
                    parameters: vec![],
                    body: Box::new(Node::Block(vec![])),
                },
            ),
            (
                "fn(x) {};",
                Node::Function {
                    parameters: vec![
                        Node::Identifier { value: "x".into() },
                    ],
                    body: Box::new(Node::Block(vec![])),
                },
            ),
            (
                "fn(x, y, z) {};",
                Node::Function {
                    parameters: vec![
                        Node::Identifier { value: "x".into() },
                        Node::Identifier { value: "y".into() },
                        Node::Identifier { value: "z".into() },
                    ],
                    body: Box::new(Node::Block(vec![])),
                },
            ),
        ];
        for (input, want) in tests {
            let program = Parser::new(Lexer::new(input.chars())).parse()
                .map_err(|err| format!("parsing function literal: {}", err))?;
            assert!(program.statements.len() == 1);
            println!("{:?} \n--- \n{:?}", program.statements[0], want);
            assert!(program.statements[0] == want);
        }
        Ok(())
    }

    #[test]
    fn function_call() -> Result<(), Error> {
        let tests = vec![
            (
                "foo();",
                Node::Call {
                    function: Box::new(Node::Identifier { value: "foo".into() }),
                    arguments: vec![],
                },
            ),
            (
                "add(1, 2);",
                Node::Call {
                    function: Box::new(Node::Identifier { value: "add".into() }),
                    arguments: vec![Node::Int(1), Node::Int(2)],
                },
            ),
            (
                "add(1, fn() { return 1; });",
                Node::Call {
                    function: Box::new(Node::Identifier { value: "add".into() }),
                    arguments: vec![
                        Node::Int(1),
                        Node::Function { 
                            parameters: vec![],
                            body: Box::new(Node::Block(vec![Node::Return { value: Box::new(Node::Placeholder) }])),
                        },
                    ],
                },
            ),
            (
                "fn(a, b) { return a + b; }(1, 2);",
                Node::Call {
                    function: Box::new(Node::Function {
                        parameters: vec![
                            Node::Identifier { value: "a".into() },
                            Node::Identifier { value: "b".into() },
                        ],
                        body: Box::new(Node::Block(vec![Node::Return {
                            // value: Box::new(Node::Infix { 
                            //     left: Box::new(Node::Identifier { value: "a".into() }),
                            //     operator: Infix::Add,
                            //     right: Box::new(Node::Identifier { value: "b".into() }),
                            // }),
                            value: Box::new(Node::Placeholder),
                        }]))
                    }),
                    arguments: vec![
                        Node::Int(1),
                        Node::Int(2),
                    ],
                },
            ),
        ];
        for (input, want) in tests {
            let program = Parser::new(Lexer::new(input.chars())).parse()
                .map_err(|err| format!("parsing function call: {}", err));
            let program = match program {
                Ok(p) => p,
                Err(err) => panic!("{}", err),
            };
            println!("want {:?}\n got {:?}\n", want, program.statements[0]);
            assert!(program.statements.len() == 1);
            assert!(program.statements[0] == want);
        }
        Ok(())
    }

}