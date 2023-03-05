pub use crate::error::ParserError;
pub use crate::expr::{Expr, IfStmt, LiteralValue, Stmt, Var, WhileStmt};
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
        while !self.is_at_end() {
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
        if matches!(self, TokenType::Fun) {
            self.function("function")
        } else if matches!(self, TokenType::Var) {
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
                    return Ok(Stmt::Var {
                        name: name,
                        initializer: Some(expr),
                    });
                }
            }
        };
        self.synchronize();
        //TODO should return null
        return Ok(Stmt::Var {
            name: name,
            initializer: None,
        });
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.assignment()
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, ParserError> {
        let name = self.consume(TokenType::Identifier, format!("Expect {} name.", kind))?;
        self.consume(
            TokenType::LeftParen,
            format!("Expect '(' after {} name.", kind),
        );
        let mut parameters: Vec<Token> = Vec::new();
        if (!matches!(self, TokenType::RightParen)) {
            loop {
                if parameters.len() >= 255 {
                    return Err(ParserError {
                        token_type: self.peek().token_type,
                        message: "Can't have more than 255 parameters".to_string(),
                    });
                }
                parameters.push(
                    self.consume(TokenType::Identifier, "Expect parameter name.".to_string())?,
                );

                if (!matches!(self, TokenType::Comma)) {
                    break;
                }
            }
            print!("{}", self.peek());
        }
        self.consume(
            TokenType::RightParen,
            "Expect ')' after parameters".to_string(),
        )?;

        self.consume(
            TokenType::LeftBrace,
            format!("Expect `{{` before {} body", kind),
        )?;
        // TODO fix this into something prettier. Block_statement could return a
        // vector
        let body = match self.block_statement()? {
            Stmt::Block { statements } => statements,
            _ => Vec::new(),
        };

        return Ok(Stmt::Function {
            name,
            params: parameters,
            body: body,
        });
    }

    fn assignment(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.or()?;

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
                        token_type: equals.token_type,
                        message: "Invalid assignment target".to_string(),
                    });
                }
            }
        }
        return Ok(expr);
    }

    fn or(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.and()?;

        while matches!(self, TokenType::Or) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        return Ok(expr);
    }

    fn and(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.equality()?;

        while (matches!(self, TokenType::And)) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        return Ok(expr);
    }

    fn statement(&mut self) -> Result<Stmt, ParserError> {
        if matches!(self, TokenType::For) {
            return self.for_statement();
        }
        if matches!(self, TokenType::If) {
            return self.if_statement();
        }
        if matches!(self, TokenType::Print) {
            return self.print_statement();
        }
        if matches!(self, TokenType::While) {
            return self.while_statement();
        }
        if matches!(self, TokenType::LeftBrace) {
            return self.block_statement();
        }
        return self.expression_statement();
    }

    fn for_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.".to_string())?;
        let initializer = if matches!(self, TokenType::Semicolon) {
            None
        } else if matches!(self, TokenType::Var) {
            Some(self.var_declaration())
        } else {
            Some(self.expression_statement())
        };

        let mut condition = if matches!(self, TokenType::Semicolon) {
            None
        } else {
            Some(self.expression())
        };
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after loop condition.".to_string(),
        )?;

        let increment = if matches!(self, TokenType::RightParen) {
            None
        } else {
            Some(self.expression())
        };

        self.consume(
            TokenType::RightParen,
            "Expect ')' after for clause.".to_string(),
        )?;

        let mut body = self.statement()?;
        if increment.is_some() {
            body = Stmt::Block {
                statements: vec![
                    Stmt::Expression {
                        expr: increment.unwrap()?,
                    },
                    body,
                ],
            };
        }

        if condition.is_none() {
            condition = Some(Ok(Expr::Literal {
                literal_value: LiteralValue::Boolean(true),
            }));
        }

        body = Stmt::WhileStmt {
            condition: condition.unwrap()?,
            body: Box::new(body),
        };

        if initializer.is_some() {
            body = Stmt::Block {
                statements: vec![initializer.unwrap()?, body],
            };
        }
        return Ok(body);
    }

    fn while_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(
            TokenType::LeftParen,
            "Expect '(' after 'while'.".to_string(),
        )?;

        let condition = self.expression()?;
        println!("condition while stmt {}", condition);

        self.consume(
            TokenType::RightParen,
            "Expect ')' after condition.".to_string(),
        )?;
        let body = self.statement()?;
        return Ok(Stmt::WhileStmt {
            condition: condition,
            body: Box::new(body),
        });
    }

    fn if_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.".to_string())?;
        match self.expression() {
            Err(err) => return Err(err),
            Ok(expr) => {
                self.consume(TokenType::RightParen, "Expect ')' after 'if'.".to_string())?;
                let then_branch = self.statement()?;
                let else_branch = if matches!(self, TokenType::Else) {
                    Some(Box::new(self.statement()?))
                } else {
                    None
                };

                return Ok(Stmt::IfStmt {
                    condition: expr,
                    thenBranch: Box::new(then_branch),
                    elseBranch: else_branch,
                });
            }
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        let value = self.expression();
        match value {
            Err(err) => return Err(err),
            Ok(expr) => {
                self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string())?;

                return Ok(Stmt::Print { expr });
            }
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        let value = self.expression();
        match value {
            Err(_err) => {
                return Err(ParserError {
                    token_type: self.peek().token_type,
                    message: "error".to_string(),
                })
            }
            Ok(expr) => {
                self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string())?;
                return Ok(Stmt::Expression { expr });
            }
        }
    }

    fn block_statement(&mut self) -> Result<Stmt, ParserError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let stmt = self.declaration()?;
            statements.push(stmt);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.".to_string())?;
        return Ok(Stmt::Block { statements });
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
        return self.call();
    }

    fn call(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.primary()?;
        loop {
            if matches!(self, TokenType::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        return Ok(expr);
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParserError> {
        let mut arguments: Vec<Expr> = Vec::new();
        if !matches!(self, TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(ParserError {
                        token_type: self.peek().token_type,
                        message: "Can't have more than 255 arguments".to_string(),
                    });
                }
                arguments.push(self.expression()?);
                if !matches!(self, TokenType::Comma) {
                    break;
                }
            }
        }

        let paren = self.consume(
            TokenType::RightParen,
            "Expect ')' after arguments.".to_string(),
        )?;
        return Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments: Box::new(arguments),
        });
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        let expr = match self.peek().token_type {
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
                )?;
                Expr::Grouping {
                    group: Box::new(expr),
                }
            }
            TokenType::Identifier => Expr::Variable { token: self.peek() },
            _ => {
                self.advance();
                return Err(ParserError {
                    token_type: self.peek().token_type,
                    message: "Expected valid expression".to_string(),
                });
            }
        };
        self.advance();
        Ok(expr)
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
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

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, ParserError> {
        if self.check(token_type) {
            return Ok(self.advance().clone());
        } else {
            return Err(ParserError {
                token_type: self.peek().token_type,
                message,
            });
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
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
