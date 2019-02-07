use crate::token::Token;

pub trait Node {
    fn token(&self) -> Token;
}

trait Expression {}

pub struct Program<N> {
    statements: Vec<N>,
}

impl<N> Node for Program<N> 
    where N: Node,
{
    fn token(&self) -> Token {
        if self.statements.len() > 0 {
            self.statements[0].token()
        } else {
            Token::Eof
        }
    }
}

struct Let<E> {
    name: Identifier,
    value: E,
}

impl<E> Node for Let<E>
    where E: Expression,
{
    fn token(&self) -> Token {
        Token::Let
    }
}

struct Identifier {
    value: String,
}

impl Node for Identifier {
    fn token(&self) -> Token {
        Token::Ident(self.value.to_owned())
    }
}