use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::error::{Error, RuntimeError};
use crate::expr::Stmt;
use crate::token::Token;

use crate::interpreter::Environment;
pub use crate::interpreter::Interpreter;
pub use crate::object::Object;

#[derive(Debug, Clone)]
pub enum LoxFunc {
    Callable {
        arity: usize,
        func: Box<fn(Vec<Object>) -> Object>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    },
}

impl LoxFunc {
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        match self {
            LoxFunc::Callable { func, .. } => Ok(func(arguments)),
            LoxFunc::Function {
                params,
                body,
                closure,
                ..
            } => {
                let mut environment =
                    Rc::new(RefCell::new(Environment::new_with_enclosing(closure)));
                for (param, arg) in params.iter().zip(arguments.iter()) {
                    environment
                        .borrow_mut()
                        .define(param.lexeme.clone(), arg.clone());
                }

                match interpreter.execute_block(body.clone(), environment) {
                    Ok(()) => Ok(Object::Nil),
                    Err(Error::ReturnError { value }) => Ok(value),
                    _ => Ok(Object::Nil),
                }
            }
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            LoxFunc::Callable { arity, .. } => *arity,
            LoxFunc::Function { params, .. } => params.len(),
        }
    }
}

impl<'a> fmt::Display for LoxFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxFunc::Callable { .. } => write!(f, "Callable"),
            LoxFunc::Function { name, .. } => write!(f, "<fn {}>", name),
            _ => write!(f, "Nil"),
        }
    }
}
