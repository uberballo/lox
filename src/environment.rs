pub use crate::error::RuntimeError;
pub use crate::expr::{Expr, LiteralValue, Stmt};
pub use crate::object::Object;
pub use crate::token::{Token, TokenType};
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, token: Token) -> Result<Object, RuntimeError> {
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
