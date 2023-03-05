use std::cell::RefCell;
use std::rc::Rc;

use crate::error::RuntimeError;
use crate::expr::Stmt;
use crate::token::Token;

//#[derive(Debug, Clone)]
//pub struct Callable {
//    pub arity: usize,
//    pub func: Box<fn(Vec<Object>) -> Object>,
//}

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
            LoxFunc::Function { params, body, .. } => {
                let mut environment = Rc::new(RefCell::new(Environment::new()));
                for (param, arg) in params.iter().zip(arguments.iter()) {
                    environment
                        .borrow_mut()
                        .define(param.lexeme.clone(), arg.clone());
                }
                interpreter.execute_block(body.clone(), environment);
                return Ok(Object::Nil);
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

//impl Callable {
//    //Take vec objects, return object
//    pub fn call(
//        &self,
//        interpreter: &mut Interpreter,
//        arguments: Vec<Object>,
//    ) -> Result<Object, RuntimeError> {
//        return Ok((self.func)(arguments));
//    }
//}

//impl Function {
//    fn call(self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Object {}
//}
