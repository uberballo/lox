use std::fmt;

use crate::callable::Object;
pub use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct ParserError {
    pub token: Token,
    pub message: String,
}

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

#[derive(Debug)]
pub enum Error {
    ReturnError { value: Object },
    RuntimeError { token: Token, message: String },
}

impl From<RuntimeError> for Error {
    fn from(runtime_error: RuntimeError) -> Self {
        Error::RuntimeError {
            token: runtime_error.token,
            message: runtime_error.message,
        }
    }
}

impl From<ReturnError> for Error {
    fn from(error: ReturnError) -> Self {
        Error::ReturnError {
            value: (error.value),
        }
    }
}

impl<'a> fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ReturnError { value } => write!(f, "Value: {}", value),
            Error::RuntimeError { token, message } => {
                write!(f, "Token: {}, message: {}", token, message)
            }
        }
    }
}

#[derive(Debug)]
pub struct ReturnError {
    pub value: Object,
}
// TODO reporting needs to be implemented
#[allow(dead_code)]
fn report(line: u32, location: String, message: String) {
    println!("[line: {}] error {}: {}", line, location, message);
    //HAD_ERROR = true;
}
