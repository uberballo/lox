pub use crate::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        //Box because otherwise it has infinite size.
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Literal {
        literalValue: LiteralValue,
    },
    Grouping {
        group: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
