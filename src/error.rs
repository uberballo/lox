pub use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct ParserError {
    pub tokenType: TokenType,
    pub message: String,
}

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

fn error(line: u32, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: u32, location: String, message: String) {
    println!("[line: {}] error {}: {}", line, location, message);
    //HAD_ERROR = true;
}
