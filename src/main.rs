use std::env;
use std::fs;
use std::io;
use std::process;
mod error;
mod expr;
mod parser;
mod scanner;
mod token;

static mut HAD_ERROR: bool = false;

fn main() {
    let mut pattern = std::env::args();
    match pattern.len() {
        1 => run_prompt(),
        2 => runFile(pattern.nth(1).clone().expect("error")),
        _ => println!("Invalid arguments"),
    }
}

fn runFile(path: String) {
    let contents = fs::read_to_string(path).expect("no file found");

    run(contents);
    //if (HAD_ERROR) {
    //    process::exit(1);
    //}
}

fn run_prompt() {
    println!("prompt");
    let stdin = io::stdin();
    let mut user_input = String::new();

    loop {
        print!(">");
        stdin
            .read_line(&mut user_input)
            .expect("Error reading input");
        if user_input == "" {
            break;
        }
        run(user_input.clone());
        //HAD_ERROR = false;
        user_input = "".to_string();
    }
}

fn run(source: String) {
    //scanner
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();
    println!("{:?}", tokens);
    let mut parser = parser::Parser::new(tokens);
    let res = parser.parse();
    println!("{:?}", res);
    println!();
}

fn error(line: u32, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: u32, location: String, message: String) {
    println!("[line: {}] error {}: {}", line, location, message);
    //HAD_ERROR = true;
}
