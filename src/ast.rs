use crate::token::{Token, Kind};

/// Node is an object that can exist in an AST.
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Node {
    // Placeholder just allows for a partialially constructed Node (for easier
    // development). Means I don't have to have all the parsing complete at once.
    Placeholder,
    Int(i64),
    String(String),
    Expression { precedence: Precedence, value: Box<Node> },
    Identifier { value: String },
    Let { name: String, value: Box<Node> },
    Return { value: Box<Node> },
    If { predicate: Box<Node>, success: Box<Node>, fail: Option<Box<Node>> },
    Prefix { operator: Prefix, value: Box<Node> },
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

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Node {
    fn token(&self) -> Token {
        Token::new(Kind::Illegal, "")
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