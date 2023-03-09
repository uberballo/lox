pub use crate::error::RuntimeError;
pub use crate::expr::{Expr, LiteralValue, Stmt};
pub use crate::object::Object;
pub use crate::token::{Token, TokenType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_enclosing(enclosing: &Rc<RefCell<Environment>>) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: Some(Rc::clone(enclosing)),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn debug_print(&self) {
        println!("Debug print {:?}", self.values);
        if self.enclosing.is_some() {
            println!(
                "Debug print {:?}",
                self.enclosing.as_ref().unwrap().borrow().values
            );
        }
    }
    //TODO think about this.
    fn get_from_enclosing(&self, token: Token) -> Result<Object, RuntimeError> {
        match &self.enclosing {
            Some(enclosing) => enclosing.borrow().get(token),
            None => {
                return Err(RuntimeError {
                    token,
                    message: "Undefined variable".to_string(),
                })
            }
        }
    }

    pub fn get(&self, token: Token) -> Result<Object, RuntimeError> {
        match self.values.get(&token.lexeme) {
            Some(obj) => return Ok(obj.clone()),
            None => {
                if self.enclosing.is_some() {
                    return self.get_from_enclosing(token);
                } else {
                    return Err(RuntimeError {
                        token,
                        message: "Undefined variable".to_string(),
                    });
                }
            }
        }
    }

    pub fn assign(&mut self, token: Token, value: Object) -> Result<(), RuntimeError> {
        if self.values.contains_key(&token.lexeme) {
            self.values.insert(token.lexeme, value);
            return Ok(());
        }
        if self.enclosing.is_some() {
            return self
                .enclosing
                .as_ref()
                .unwrap()
                .borrow_mut()
                .assign(token, value);
        }

        return Err(RuntimeError {
            token,
            message: "Undefined variable".to_string(),
        });
    }
}
