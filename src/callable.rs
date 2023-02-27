use std::cell::RefCell;
use std::rc::Rc;

use crate::error::RuntimeError;
use crate::expr::Stmt;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Callable {
    pub arity: usize,
    pub func: Box<fn(Vec<Object>) -> Object>,
}

use crate::interpreter::Environment;
pub use crate::interpreter::Interpreter;
pub use crate::object::Object;

impl Callable {
    //Take vec objects, return object
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        return Ok((self.func)(arguments));
    }
}

// TODO check the Callable/Function

//Function {
//    name: Token,
//    params: Vec<Token>,
//    body: Vec<Stmt>,
//},
pub struct Function {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
}

impl Function {
    fn call(self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Object {
        let mut environment = Rc::new(RefCell::new(Environment::new()));
        for (param, arg) in self.params.iter().zip(arguments.iter()) {
            environment
                .borrow_mut()
                .define(param.lexeme.clone(), arg.clone());
        }
        interpreter.execute_block(self.body, environment);
        return Object::Nil;
    }
}
