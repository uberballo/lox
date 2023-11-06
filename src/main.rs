use std::fs;
use std::io;

mod callable;
mod environment;
mod error;
mod expr;
mod interpreter;
mod object;
mod parser;
mod scanner;
mod token;

fn main() {
    let mut pattern = std::env::args();
    let interpreter = interpreter::Interpreter::new();
    let mut lox = Lox { interpreter };

    match pattern.len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(pattern.nth(1).clone().expect("error")),
        _ => println!(
            "Invalid arguments. Run without arguments for REPL or pass a path to a .lox file"
        ),
    }
}
struct Lox {
    pub interpreter: interpreter::Interpreter,
}

impl Lox {
    pub fn run_file(&mut self, path: String) {
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
        let mut scanner = scanner::Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = parser::Parser::new(tokens);
        let statements = parser.parse();
        self.interpreter.interpret_stmts(statements);
    }
}
