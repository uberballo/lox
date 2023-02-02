pub use crate::token::{Token, TokenType};
use std::collections::HashMap;

pub struct Scanner {
    keywords: HashMap<&'static str, TokenType>,
    source: String,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let mut keywords = HashMap::with_capacity(16);
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);

        Scanner {
            keywords,
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while (!self.is_at_end()) {
            self.start = self.current;
            self.scan_token();
        }
        let tok = Token {
            tokenType: TokenType::Eof,
            lexeme: "".to_string(),
            line: self.line,
        };

        self.tokens.push(tok);
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ';' => self.add_token(TokenType::Semicolon),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            '*' => self.add_token(TokenType::Star),
            '!' if self.matches('=') => self.add_token(TokenType::BangEqual),
            '!' => self.add_token(TokenType::Bang),
            '=' if self.matches('=') => self.add_token(TokenType::EqualEqual),
            '=' => self.add_token(TokenType::Equal),
            '<' if self.matches('=') => self.add_token(TokenType::LessEqual),
            '<' => self.add_token(TokenType::Less),
            '>' if self.matches('=') => self.add_token(TokenType::GreaterEqual),
            '>' => self.add_token(TokenType::Greater),
            '/' => self.add_token(TokenType::Slash), //No comment handling
            'o' if self.matches('r') => self.add_token(TokenType::Or),
            ' ' => print!(""),
            '\r' => print!(""),
            '\n' => self.line += 1,
            '"' => self.string(),
            c if is_digit(c) => self.number(),
            c if is_alpha(c) => self.identifier(),
            _ => println!("OH no, lexical error!"),
        }
    }

    fn advance(&mut self) -> char {
        let character = self.peek();
        self.current += 1;
        return character;
    }

    fn add_token(&mut self, tokenType: TokenType) {
        let subString = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            tokenType,
            lexeme: subString,
            line: self.line,
        })
    }

    fn matches(&mut self, character: char) -> bool {
        if self.is_at_end() {
            return false;
        } else if self.peek() != character {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).expect("shit");
    }

    fn peek_next(&self) -> char {
        if (self.current + 1 >= self.source.len()) {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).expect("shit");
    }

    fn string(&mut self) {
        while (self.peek() != '"' && !self.is_at_end()) {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if (self.is_at_end()) {
            println!("Errori!!");
            return;
        }
        self.advance();
        let literal = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String { literal });
    }

    fn number(&mut self) {
        while (is_digit(self.peek())) {
            self.advance();
        }
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
            while (is_digit(self.peek())) {
                self.advance();
            }
        }
        let literal: f64 = self.source[self.start..self.current]
            .parse()
            .expect("invalid number");
        self.add_token(TokenType::Number { literal })
    }

    fn identifier(&mut self) {
        while (is_alpha_numeric(self.peek())) {
            self.advance();
        }
        let literal = &self.source[self.start..self.current];
        let tokenType: TokenType = self
            .keywords
            .get(literal)
            .cloned()
            .unwrap_or(TokenType::Identifier);

        self.add_token(tokenType)
    }
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic()
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}
