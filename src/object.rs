use crate::callable::LoxFunc;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Object {
    Number(f64),
    String(String),
    Nil,
    Boolean(bool),
    Call(LoxFunc),
}

impl Default for Object {
    fn default() -> Self {
        Object::Nil
    }
}

impl<'a> fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Number(x) => write!(f, "{}", x),
            Object::String(x) => write!(f, "\"{}\"", x),
            Object::Boolean(x) => write!(f, "\"{}\"", x),
            Object::Nil => write!(f, "nil"),
            _ => write!(f, "Nil"),
        }
    }
}
