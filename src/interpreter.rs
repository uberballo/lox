pub use crate::environment::Environment;
pub use crate::error::RuntimeError;
pub use crate::expr::{Expr, LiteralValue, Stmt};
pub use crate::object::Object;
pub use crate::token::{Token, TokenType};

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    //pub fn visit_grouping_expr(expr: Expr) {
    //    self.evaluate(expr.expression)
    //}

    //fn evaluate(expr Expr) -> object {
    //    return expr.accept(this)
    //}
    pub fn interpret_stmt(&mut self, statements: Vec<Stmt>) {
        for stmt in statements.into_iter() {
            //TODO This is really stupid. Add the enum
            match stmt {
                Stmt {
                    expression: Some(_),
                    print: None,
                    var: None,
                    block: None,
                } => self.visit_expression_stmt(stmt),
                Stmt {
                    expression: None,
                    print: Some(_),
                    var: None,
                    block: None,
                } => self.visit_print_stmt(stmt),
                Stmt {
                    expression: None,
                    print: None,
                    var: Some(_),
                    block: None,
                } => self.visit_var_stmt(stmt),
                Stmt {
                    expression: None,
                    print: None,
                    var: None,
                    block: Some(_),
                } => self.visit_block_stmt(stmt),
                _ => println!("Invalid statement"),
            }
        }
    }

    pub fn interpret(&mut self, expr: Expr) -> Result<Object, RuntimeError> {
        match expr {
            Expr::Binary {
                left: _,
                operator: _,
                right: _,
            } => Ok(self.visit_binary_expr(expr)),
            Expr::Unary {
                operator: _,
                right: _,
            } => Ok(self.visit_unary_expr(expr)),
            Expr::Literal { literal_value: _ } => Ok(self.visit_literal_expr(expr)),
            //TODO remove unwrap
            Expr::Variable { token: _ } => self.visit_var_expr(expr),
            Expr::Assign { name: _, value: _ } => self.visit_assign_expr(expr),
            _ => Ok(Object::Nil),
        }
    }

    fn is_truthy(&self, object: Object) -> bool {
        match object {
            Nil => false,
            False => false,
            True => true,
            _ => true,
        }
    }

    //public Object visitLiteralExpr(Expr.Literal expr) {
    //    return expr.value;
    //  }

    fn literal_to_object(&self, literal_value: LiteralValue) -> Object {
        match literal_value {
            LiteralValue::Number(x) => Object::Number(x),
            LiteralValue::String(x) => Object::String(x),
            LiteralValue::Boolean(x) => Object::Boolean(x),
            LiteralValue::Null => Object::Nil,
        }
    }

    fn visit_literal_expr(&self, expr: Expr) -> Object {
        match expr {
            Expr::Literal {
                literal_value: value,
            } => self.literal_to_object(value),
            _ => Object::Nil,
        }
    }

    fn object_number(&self, object: Object) -> f64 {
        match object {
            Object::Number(x) => x,
            _ => 0.0,
        }
    }

    fn is_equal(&self, a: Object, b: Object) -> bool {
        match (a, b) {
            //TODO only number and String comparison
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Nil, Object::Nil) => true,
            (Object::Nil, _) => false,
            _ => false,
        }
    }

    fn visit_binary_expr(&mut self, expr: Expr) -> Object {
        match expr {
            Expr::Binary {
                left: left,
                operator: operator,
                right: right,
            } => {
                let left_value: Object = self.interpret(*left).unwrap();
                let right_value: Object = self.interpret(*right).unwrap();
                match operator.tokenType {
                    TokenType::Minus => {
                        let left_number = self.object_number(left_value);
                        let right_number = self.object_number(right_value);
                        return Object::Number(left_number - right_number);
                    }
                    TokenType::Plus => {
                        let left_number = self.object_number(left_value);
                        let right_number = self.object_number(right_value);
                        return Object::Number(left_number + right_number);
                    }
                    TokenType::Slash => {
                        let left_number = self.object_number(left_value);
                        let right_number = self.object_number(right_value);
                        return Object::Number(left_number / right_number);
                    }
                    TokenType::Star => {
                        let left_number = self.object_number(left_value);
                        let right_number = self.object_number(right_value);
                        return Object::Number(left_number * right_number);
                    }
                    TokenType::Greater => {
                        let left_number = self.object_number(left_value);
                        let right_number = self.object_number(right_value);
                        return Object::Boolean(left_number > right_number);
                    }
                    TokenType::GreaterEqual => {
                        let left_number = self.object_number(left_value);
                        let right_number = self.object_number(right_value);
                        return Object::Boolean(left_number >= right_number);
                    }
                    TokenType::Less => {
                        let left_number = self.object_number(left_value);
                        let right_number = self.object_number(right_value);
                        return Object::Boolean(left_number < right_number);
                    }
                    TokenType::LessEqual => {
                        let left_number = self.object_number(left_value);
                        let right_number = self.object_number(right_value);
                        return Object::Boolean(left_number <= right_number);
                    }
                    TokenType::BangEqual => {
                        return Object::Boolean(!self.is_equal(left_value, right_value));
                    }
                    TokenType::EqualEqual => {
                        return Object::Boolean(self.is_equal(left_value, right_value));
                    }
                    //TODO No string + string yet.
                    _ => return Object::Nil,
                }
            }
            _ => Object::Nil,
        }
    }

    fn check_number_operand(&self, operator: Token, operand: &Object) {
        match operand {
            Object::Number(_) => println!("No problem"),
            _ => println!("Error!"),
        }
    }

    fn check_number_operands(&self, operator: Token, left: &Object, right: &Object) {
        match (left, right) {
            (Object::Number(_), Object::Number(_)) => println!("No problem"),
            _ => println!("Error!"),
        }
    }

    fn visit_unary_expr(&self, expr: Expr) -> Object {
        match expr {
            Expr::Unary {
                operator: operator,
                right: right,
            } => match operator.tokenType {
                TokenType::Bang => {
                    Object::Boolean(!self.is_truthy(self.visit_literal_expr(*right)))
                }
                TokenType::Minus => {
                    let object = self.visit_literal_expr(*right);
                    self.check_number_operand(operator, &object);
                    Object::Number(-self.object_number(object))
                }
                _ => Object::Nil,
            },
            _ => Object::Nil,
        }
    }

    fn visit_expression_stmt(&mut self, stmt: Stmt) {
        println!("expression");
        let object: Option<Result<Object, RuntimeError>> = match stmt.expression {
            Some(expr) => Some(self.interpret(expr)),
            _ => None,
        };
    }

    fn visit_print_stmt(&mut self, stmt: Stmt) {
        match stmt.print {
            Some(expr) => println!("Printing: {:?}", self.interpret(expr).unwrap_or_default()),
            None => println!("None"),
        };
    }

    fn visit_var_stmt(&mut self, stmt: Stmt) {
        match stmt.var {
            Some(var) => {
                let value = match var.initializer {
                    Some(initializer) => self.interpret(initializer),
                    None => Ok(Object::Nil),
                };
                match value {
                    Err(e) => panic!("{:?}", e),
                    Ok(value) => self.environment.define(var.name.lexeme, value),
                }
            }
            None => println!("None"),
        };
    }

    fn visit_block_stmt(&mut self, stmt: Stmt) {
        match stmt.block {
            Some(statements) => self.execute_block(statements),
            _ => println!("None!"),
        }
    }

    fn execute_block(&mut self, statements: Vec<Stmt>) {
        //Store previous environment
        let previous = self.environment.clone();
        //self.environment = environment;
        self.interpret_stmt(statements);
        //Set the environment back to previous one.
        self.environment = previous;
    }

    fn visit_var_expr(&self, expr: Expr) -> Result<Object, RuntimeError> {
        match expr {
            Expr::Variable { token } => return self.environment.get(token),
            _ => panic!("Not here! Error"),
        }
    }

    fn visit_assign_expr(&mut self, expr: Expr) -> Result<Object, RuntimeError> {
        match expr {
            Expr::Assign { name, value } => {
                let new_value = self.interpret(*value);
                match new_value {
                    Ok(obj) => {
                        self.environment.assign(name, obj.clone());
                        return Ok(obj);
                    }
                    Err(e) => return Err(e),
                }
            }
            _ => panic!("Not here! error visiting assign expression"),
        }
    }
}
