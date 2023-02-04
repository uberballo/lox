pub use crate::error::RuntimeError;
pub use crate::expr::{Expr, LiteralValue, Stmt};
pub use crate::object::Object;
pub use crate::token::{Token, TokenType};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn add_enclosing(&mut self, environment: Environment) {
        self.enclosing = Some(Box::new(environment));
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    //TODO think about this.
    fn get_from_enclosing(&self, token: Token) -> Result<Object, RuntimeError> {
        match &self.enclosing {
            Some(enclosing) => enclosing.get(token),
            None => {
                return Err(RuntimeError {
                    token,
                    message: "Undefined variable".to_string(),
                })
            }
        }
    }

    pub fn get(&self, token: Token) -> Result<Object, RuntimeError> {
        if self.enclosing.is_some() {
            return self.get_from_enclosing(token);
        }
        match self.values.get(&token.lexeme) {
            Some(obj) => return Ok(obj.clone()),
            None => {
                return Err(RuntimeError {
                    token,
                    message: "Undefined variable".to_string(),
                })
            }
        }
    }

    pub fn assign(&mut self, token: Token, value: Object) -> Result<(), RuntimeError> {
        if self.enclosing.is_some() {
            return self.enclosing.as_mut().unwrap().assign(token, value);
        }

        if self.values.contains_key(&token.lexeme) {
            self.values.insert(token.lexeme, value);
            return Ok(());
        }

        return Err(RuntimeError {
            token,
            message: "Undefined variable".to_string(),
        });
    }
}
