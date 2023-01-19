pub use crate::error::ParserError;
pub use crate::expr::{Expr, LiteralValue};
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

    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        return self.expression();
        // catch ParseEerror return null
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
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
        println!("here");
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
                literalValue: LiteralValue::Boolean(false),
            },
            TokenType::True => Expr::Literal {
                literalValue: LiteralValue::Boolean(true),
            },
            TokenType::Nil => Expr::Literal {
                literalValue: LiteralValue::Null,
            },
            TokenType::Number { literal } => Expr::Literal {
                literalValue: LiteralValue::Number(literal),
            },
            TokenType::String { literal } => Expr::Literal {
                literalValue: LiteralValue::String(literal),
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
            _ => {
                return Err(ParserError {
                    tokenType: self.peek().tokenType,
                    message: "Expected valid expression".to_string(),
                })
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
