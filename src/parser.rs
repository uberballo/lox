pub use crate::error::ParserError;
pub use crate::expr::{Expr, LiteralValue, Stmt, Var};
pub use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// Match is reserved
macro_rules! matches {
    ($s:ident, $( $x: expr),* ) => {
        {
            if $($s.check($x)) || * {
                $s.advance();
                true
            } else {
                false
            }
        }
    };
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    //pub fn parse(&mut self) -> Result<Expr, ParserError> {
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        while (!self.is_at_end()) {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => println!("{:?}", err),
            }
        }
        //return self.expression();
        return statements;
        // catch ParseEerror return null
    }

    fn declaration(&mut self) -> Result<Stmt, ParserError> {
        if matches!(self, TokenType::Var) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParserError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.".to_string())?;

        if matches!(self, TokenType::Equal) {
            match self.expression() {
                Err(err) => return Err(err),
                Ok(expr) => {
                    self.consume(TokenType::Semicolon, "Expect variable name.".to_string())?;
                    return Ok(Stmt {
                        expression: None,
                        print: None,
                        var: Some(Var {
                            name: name,
                            initializer: Some(expr),
                        }),
                    });
                }
            }
        };
        self.synchronize();
        //TODO should return null
        return Err(ParserError {
            tokenType: self.peek().tokenType,
            message: "bad broken".to_string(),
        });
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.equality()?;

        while matches!(self, TokenType::Equal) {
            let equals = self.previous();
            let value = self.assignment()?;

            match expr {
                Expr::Variable { token } => {
                    return Ok(Expr::Assign {
                        name: token,
                        value: Box::new(value),
                    });
                }
                _ => {
                    return Err(ParserError {
                        tokenType: equals.tokenType,
                        message: "Invalid assignment target".to_string(),
                    });
                }
            }
        }
        return Ok(expr);
    }

    fn statement(&mut self) -> Result<Stmt, ParserError> {
        if matches!(self, TokenType::Print) {
            return self.print_statement();
        }
        return self.expression_statement();
    }

    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        let value = self.expression();
        match value {
            Err(err) => return Err(err),
            Ok(expr) => {
                let a = self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string());

                return Ok(Stmt {
                    expression: None,
                    print: Some(expr),
                    var: None,
                });
            }
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        let value = self.expression();
        match value {
            Err(err) => {
                return Err(ParserError {
                    tokenType: self.peek().tokenType,
                    message: "error".to_string(),
                })
            }
            Ok(expr) => {
                self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string());
                return Ok(Stmt {
                    expression: Some(expr),
                    print: None,
                    var: None,
                });
            }
        }
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.comparison()?;

        while matches!(self, TokenType::BangEqual, TokenType::EqualEqual) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.term()?;
        while matches!(
            self,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual
        ) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.factor()?;
        while matches!(self, TokenType::Minus, TokenType::Plus) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.unary()?;
        while matches!(self, TokenType::Slash, TokenType::Star) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        while matches!(self, TokenType::Bang, TokenType::Minus) {
            let operator = self.previous();
            let right = self.unary()?;

            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        let expr = match self.peek().tokenType {
            TokenType::False => Expr::Literal {
                literal_value: LiteralValue::Boolean(false),
            },
            TokenType::True => Expr::Literal {
                literal_value: LiteralValue::Boolean(true),
            },
            TokenType::Nil => Expr::Literal {
                literal_value: LiteralValue::Null,
            },
            TokenType::Number { literal } => Expr::Literal {
                literal_value: LiteralValue::Number(literal),
            },
            TokenType::String { literal } => Expr::Literal {
                literal_value: LiteralValue::String(literal),
            },
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(
                    TokenType::RightParen,
                    "Expect ')' after expression.".to_string(),
                );
                Expr::Grouping {
                    group: Box::new(expr),
                }
            }
            TokenType::Identifier => Expr::Variable { token: self.peek() },
            _ => {
                self.advance();
                return Err(ParserError {
                    tokenType: self.peek().tokenType,
                    message: "Expected valid expression".to_string(),
                });
            }
        };
        self.advance();
        Ok(expr)
    }

    fn check(&self, tokenType: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().tokenType == tokenType
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().tokenType == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).cloned().expect("No previous")
    }

    fn previous(&self) -> Token {
        self.tokens
            .get(self.current - 1)
            .cloned()
            .expect("No previous")
    }

    fn consume(&mut self, tokenType: TokenType, message: String) -> Result<Token, ParserError> {
        if self.check(tokenType) {
            return Ok(self.advance().clone());
        } else {
            return Err(ParserError {
                tokenType: self.peek().tokenType,
                message,
            });
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().tokenType == TokenType::Semicolon {
                return;
            }
            match self.peek().tokenType {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => self.advance(),
            };
        }
    }
}
