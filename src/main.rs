use std::env;
use std::fs;
use std::io;
use std::process;
mod environment;
mod error;
mod expr;
mod interpreter;
mod object;
mod parser;
mod scanner;
mod token;

static mut HAD_ERROR: bool = false;

fn main() {
    let mut pattern = std::env::args();
    let mut environment = environment::Environment::new();
    let mut interpreter = interpreter::Interpreter { environment };
    let mut lox = Lox { interpreter };

    match pattern.len() {
        1 => lox.run_prompt(),
        2 => lox.runFile(pattern.nth(1).clone().expect("error")),
        _ => println!("Invalid arguments"),
    }
}
struct Lox {
    pub interpreter: interpreter::Interpreter,
}

impl Lox {
    pub fn runFile(&mut self, path: String) {
        let contents = fs::read_to_string(path).expect("no file found");

        self.run(contents);
        //if (HAD_ERROR) {
        //    process::exit(1);
        //}
    }

    pub fn run_prompt(&mut self) {
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
            self.run(user_input.clone());
            //HAD_ERROR = false;
            user_input = "".to_string();
        }
    }

    fn run(&mut self, source: String) {
        //scanner
        let mut scanner = scanner::Scanner::new(source);
        let mut tokens = scanner.scan_tokens();
        println!();
        for tk in &mut tokens {
            println!("{:?}", tk);
        }
        println!();
        let mut parser = parser::Parser::new(tokens);
        let mut statements = parser.parse();

        println!();
        for st in &mut statements {
            println!("{:?}", st);
        }
        println!();
        self.interpreter.interpret_stmts(statements);
    }

    fn error(&self, line: u32, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&self, line: u32, location: String, message: String) {
        println!("[line: {}] error {}: {}", line, location, message);
        //HAD_ERROR = true;
    }
}
