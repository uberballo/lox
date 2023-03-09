use crate::callable::Object;
pub use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct ParserError {
    pub token_type: TokenType,
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

#[derive(Debug)]
pub struct ReturnError {
    pub value: Object,
}

fn error(line: u32, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: u32, location: String, message: String) {
    println!("[line: {}] error {}: {}", line, location, message);
    //HAD_ERROR = true;
}
