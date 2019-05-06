use crate::token::{Token, Kind};
use std::fmt::{self, Display, Formatter};

/// Node is an object that can exist in an AST.
//
// TODO: Note that in cases where I expect a specific enum branch I am required
// generalise to `Node` because enum variants are not first class types.
// In order to be more correct I create individual struct types and wrap 
// them in the enum. 
//
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Node {
    // Placeholder just allows for a partialially constructed Node (for easier
    // development). Means I don't have to have all the parsing complete at once.
    Placeholder,
    Int(i64),
    String(String),
    Boolean(bool),
    Expression { precedence: Precedence, value: Box<Node> },
    Identifier { value: String },
    Let { name: String, value: Box<Node> },
    Return { value: Box<Node> },
    If { predicate: Box<Node>, success: Box<Node>, fail: Option<Box<Node>> },
    Block(Vec<Node>),
    Prefix { operator: Prefix, value: Box<Node> },
    Infix { left: Box<Node>, operator: Infix, right: Box<Node> },
    Function { parameters: Vec<Node>, body: Box<Node> },
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

#[derive(Eq, PartialEq, Debug, Clone, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Precedence {
    pub fn from(token: Kind) -> Precedence {
        match token {
            Kind::Equal | Kind::NotEqual => Precedence::Equals,
            Kind::ArrowLeft | Kind::ArrowRight => Precedence::LessGreater,
            Kind::Plus | Kind::Minus => Precedence::Sum,
            Kind::Slash | Kind::Asterisk  => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}

impl Node {
    fn token(&self) -> Token {
        Token::new(Kind::Illegal, "")
    }
}

#[derive(Debug)]
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

impl Display for Program {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", match self {
            Node::Prefix { operator, value } => format!("({}{})", operator, value),
            Node::Infix { left, operator, right } => format!("({} {} {})", left, operator, right),
            Node::Int(n) => n.to_string(),
            Node::String(s) => s.to_owned(),
            Node::Boolean(b) => b.to_string(),
            Node::Identifier { value } => value.to_owned(),
            Node::If { predicate, success, fail } => {
                match fail {
                    None => format!("if {} {{ {} }}", predicate, success),
                    Some(fail) => format!("if {} {{ {} }} else {{ {} }}", predicate, success, fail),
                }
            },
            Node::Block(list) => {
                let mut buf = String::new();
                for node in list {
                    write!(buf, "{}", node)?;
                }
                buf
            },
            Node::Function { parameters, body } => {
                let mut buf = String::new();
                buf.write_char('(')?;
                for (ii, param) in parameters.iter().enumerate() {
                    write!(buf, "{}", param)?;
                    if ii < parameters.len()-1 {
                        buf.write_char(',')?;
                    }
                }
                buf.write_char(')')?;
                write!(buf, "{}", body)?;
                buf
            },
            _ => format!("na"),
        })
    }
}

// Note: Instead of statically mapping variants to a token character, perhaps
// the token should be tied to the value. This would mean that if the token
// representation changes we don't need to make several changes in the code base
// (ie, one there and one here).

impl Display for Prefix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Prefix::Negative => "-",
            Prefix::Not => "!",
        })
    }
}

impl Display for Infix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Infix::Add => "+",
            Infix::Divide => "/",
            Infix::Eq => "==",
            Infix::GreaterThan => ">",
            Infix::LessThan => "<",
            Infix::Subtract => "-",
            Infix::Multiply => "*",
            Infix::NotEq => "!=",
        })
    }
}