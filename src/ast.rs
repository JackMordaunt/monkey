use crate::token::Token;

/// Node is an object that can exist in an AST.
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Node {
    // Placeholder just allows for a partialially constructed Node (for easier
    // development). Means I don't have to have all the parsing complete at once.
    Placeholder,
    Let { name: String, value: Box<Node> },
    Return { value: Box<Node> },
    Int(i32),
    If { predicate: Box<Node>, success: Box<Node>, fail: Option<Box<Node>> },
    Prefix { operator: Prefix, right: Box<Node> },
    Infix { left: Box<Node>, operator: Infix, right: Box<Node> },
}

// Prefix operator. 
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Prefix {
    Not, // !
    Negative, // -
}

// Infix operator. 
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Infix {
    Eq,
    NotEq,
    LessThan,
    GreaterThan,
    Add,
    Subtract,
    Divide,
    Multiply,
}

impl Node {
    fn token(&self) -> Token {
        Token::Illegal("".to_owned())
    }
}

pub struct Program {
    pub statements: Vec<Node>,
}

impl Program {
    pub fn new(statements: Vec<Node>) -> Program {
        Program {
            statements,
        }
    }
}