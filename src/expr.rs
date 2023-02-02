pub use crate::token::Token;
use std::fmt;
// TODO implement some of the methods.
// evaluate, accept, execute
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
        literal_value: LiteralValue,
    },
    Variable {
        token: Token,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
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

#[derive(Debug)]
pub struct Var {
    pub name: Token,
    pub initializer: Option<Expr>,
}

#[derive(Debug)]
pub struct Stmt {
    pub expression: Option<Expr>,
    pub print: Option<Expr>,
    pub var: Option<Var>,
}
