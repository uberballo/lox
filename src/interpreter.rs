use crate::callable::LoxFunc;
pub use crate::environment::Environment;
use crate::error::Error;
pub use crate::error::RuntimeError;
pub use crate::expr::{Expr, LiteralValue, Stmt};
pub use crate::object::Object;
pub use crate::token::{Token, TokenType};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            globals: Rc::new(RefCell::new(Environment::new())),
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    #[allow(dead_code)]
    fn get_clock() -> Object {
        let clock = Object::Call(LoxFunc::Callable {
            arity: 0,
            // ignore args, return new number object.
            func: Box::new(|_: Vec<Object>| {
                Object::Number(
                    (SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Somehow the time broke")
                        .as_millis()) as f64,
                )
            }),
        });
        return clock;
    }
    // Really refactor this
    fn interpret_stmt(&mut self, stmt: Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::Expression { .. } => self.visit_expression_stmt(stmt),
            Stmt::Print { .. } => self.visit_print_stmt(stmt),
            Stmt::Return { .. } => self.visit_return_stmt(stmt)?,
            Stmt::Var { .. } => self.visit_var_stmt(stmt),
            Stmt::Block { .. } => self.visit_block_stmt(stmt)?,
            Stmt::IfStmt { .. } => self.visit_if_stmt(stmt)?,
            Stmt::WhileStmt { .. } => self.visit_while_stmt(stmt)?,
            Stmt::Function { .. } => self.visit_function_stmt(stmt),
        }
        return Ok(());
    }

    pub fn interpret_stmts(&mut self, statements: Vec<Stmt>) {
        for stmt in statements.into_iter() {
            if let Err(err) = self.interpret_stmt(stmt) {
                panic!("{}", err)
            }
        }
    }

    pub fn interpret(&mut self, expr: Expr) -> Result<Object, RuntimeError> {
        match expr {
            Expr::Binary {
                left: _,
                operator: _,
                right: _,
            } => self.visit_binary_expr(expr),
            Expr::Unary {
                operator: _,
                right: _,
            } => Ok(self.visit_unary_expr(expr)),
            Expr::Literal { literal_value: _ } => Ok(self.visit_literal_expr(expr)),
            Expr::Variable { token: _ } => self.visit_var_expr(expr),
            Expr::Assign { name: _, value: _ } => self.visit_assign_expr(expr),
            Expr::Logical {
                left: _,
                operator: _,
                right: _,
            } => Ok(self.visit_logical_expr(expr)),
            Expr::Call {
                callee: _,
                paren: _,
                arguments: _,
            } => self.visit_call_expr(expr),
            _ => Ok(Object::Nil),
        }
    }

    fn is_truthy(&self, object: Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Boolean(false) => false,
            Object::Boolean(true) => true,
            _ => true,
        }
    }

    fn is_truthy_2(&self, object: &Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Boolean(false) => false,
            Object::Boolean(true) => true,
            _ => true,
        }
    }

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

    fn visit_logical_expr(&mut self, expr: Expr) -> Object {
        match expr {
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left_object = self.interpret(*left).unwrap().clone();
                if operator.token_type == TokenType::Or {
                    if self.is_truthy_2(&left_object) {
                        return left_object;
                    }
                } else {
                    if !self.is_truthy_2(&left_object) {
                        return left_object;
                    }
                }
                return self.interpret(*right).unwrap();
            }
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
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Nil, Object::Nil) => true,
            (Object::Nil, _) => false,
            _ => false,
        }
    }

    fn addition(&self, a: Object, b: Object) -> Object {
        match (a, b) {
            (Object::Number(left_value), Object::Number(right_value)) => {
                return Object::Number(left_value + right_value);
            }
            (Object::String(left_value), Object::String(right_value)) => {
                return Object::String(format!("{}{}", left_value, right_value));
            }
            _ => Object::Nil,
        }
    }

    fn visit_binary_expr(&mut self, expr: Expr) -> Result<Object, RuntimeError> {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_value: Object = self.interpret(*left)?;
                let right_value: Object = self.interpret(*right)?;

                match operator.token_type {
                    TokenType::Minus => {
                        self.check_number_operands(operator, &left_value, &right_value);
                        let left_number = self.object_number(left_value);
                        let right_number = self.object_number(right_value);
                        return Ok(Object::Number(left_number - right_number));
                    }
                    TokenType::Plus => return Ok(self.addition(left_value, right_value)),
                    TokenType::Slash => {
                        let left_number = self.object_number(left_value);
                        let right_number = self.object_number(right_value);
                        return Ok(Object::Number(left_number / right_number));
                    }
                    TokenType::Star => {
                        let left_number = self.object_number(left_value);
                        let right_number = self.object_number(right_value);
                        return Ok(Object::Number(left_number * right_number));
                    }
                    TokenType::Greater => {
                        let left_number = self.object_number(left_value);
                        let right_number = self.object_number(right_value);
                        return Ok(Object::Boolean(left_number > right_number));
                    }
                    TokenType::GreaterEqual => {
                        let left_number = self.object_number(left_value);
                        let right_number = self.object_number(right_value);
                        return Ok(Object::Boolean(left_number >= right_number));
                    }
                    TokenType::Less => {
                        let left_number = self.object_number(left_value);
                        let right_number = self.object_number(right_value);
                        return Ok(Object::Boolean(left_number < right_number));
                    }
                    TokenType::LessEqual => {
                        let left_number = self.object_number(left_value);
                        let right_number = self.object_number(right_value);
                        return Ok(Object::Boolean(left_number <= right_number));
                    }
                    TokenType::BangEqual => {
                        return Ok(Object::Boolean(!self.is_equal(left_value, right_value)));
                    }
                    TokenType::EqualEqual => {
                        return Ok(Object::Boolean(self.is_equal(left_value, right_value)));
                    }
                    _ => return Ok(Object::Nil),
                }
            }
            _ => Ok(Object::Nil),
        }
    }

    fn visit_call_expr(&mut self, expr: Expr) -> Result<Object, RuntimeError> {
        match expr {
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee_value = self.interpret(*callee)?;

                let argument_objects: Result<Vec<Object>, RuntimeError> = arguments
                    .into_iter()
                    .map(|x| self.interpret(x))
                    .collect::<Result<Vec<Object>, RuntimeError>>();

                let args = argument_objects?;

                // TODO check this
                match callee_value {
                    Object::Call(callable) => {
                        let arguments_len = args.len();
                        if callable.arity() != arguments_len {
                            return Err(RuntimeError {
                                token: paren,
                                message: "Something went wrong with the callable".to_string(),
                            });
                        }
                        return callable.call(self, args);
                    }
                    _ => return Ok(Object::Nil),
                }
            }
            _ => return Ok(Object::Nil),
        }
    }

    fn check_number_operand(&self, _operator: Token, operand: &Object) {
        match operand {
            Object::Number(_) => (),
            _ => println!("Error!"),
        }
    }

    //Not the best.
    fn check_number_operands(&self, operator: Token, left: &Object, right: &Object) {
        match (left, right) {
            (Object::Number(_), Object::Number(_)) => (),
            _ => println!("Error! {:?}", operator),
        }
    }

    fn visit_unary_expr(&self, expr: Expr) -> Object {
        match expr {
            Expr::Unary { operator, right } => match operator.token_type {
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
        let _object: Option<Result<Object, RuntimeError>> = match stmt {
            Stmt::Expression { expr } => Some(self.interpret(expr)),
            _ => None,
        };
    }

    fn visit_function_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Function { name, params, body } => {
                let function = LoxFunc::Function {
                    name: name.clone(),
                    params,
                    body,
                    closure: Rc::clone(&self.environment),
                };

                self.environment
                    .borrow_mut()
                    .define(name.lexeme, Object::Call(function));
            }
            _ => println!("No function"),
        }
    }

    fn visit_print_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Print { expr } => {
                println!("Printing: {:?}", self.interpret(expr).unwrap_or_default())
            }
            _ => println!("None"),
        };
    }

    fn visit_return_stmt(&mut self, stmt: Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::Return { value, .. } => {
                match value {
                    Some(expr) => {
                        return Err(Error::ReturnError {
                            value: self.interpret(expr).unwrap(),
                        });
                    }
                    None => return Ok(()),
                };
            }
            _ => return Ok(()),
        }
    }

    fn visit_var_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Var { name, initializer } => {
                let value = match initializer {
                    Some(initializer) => self.interpret(initializer),
                    None => Ok(Object::Nil),
                };
                match value {
                    Err(e) => panic!("{:?}", e),
                    Ok(value) => self.environment.borrow_mut().define(name.lexeme, value),
                }
            }
            _ => println!("None"),
        };
    }

    fn visit_block_stmt(&mut self, stmt: Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::Block { statements } => self.execute_block(
                statements,
                Rc::new(RefCell::new(Environment::new_with_enclosing(
                    &self.environment,
                ))),
            ),
            _ => Ok(()),
        }
    }

    fn visit_while_stmt(&mut self, stmt: Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::WhileStmt { condition, body } => {
                let condition = condition.clone();
                let mut value = self.interpret(condition.clone()).unwrap();
                while self.is_truthy_2(&value) {
                    self.interpret_stmt(*body.clone())?;
                    value = self.interpret(condition.clone())?;
                }
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    fn interpret_block_stmts(&mut self, statements: Vec<Stmt>) -> Result<(), Error> {
        for statement in statements {
            self.interpret_stmt(statement)?
        }
        Ok(())
    }

    pub fn execute_block(
        &mut self,
        statements: Vec<Stmt>,
        env: Rc<RefCell<Environment>>,
    ) -> Result<(), Error> {
        //Store previous environment
        let previous = self.environment.clone();
        // Check environment
        self.environment = env;

        let result = self.interpret_block_stmts(statements);
        //Set the environment back to previous one.
        self.environment = previous;
        result
    }

    fn visit_var_expr(&self, expr: Expr) -> Result<Object, RuntimeError> {
        match expr {
            Expr::Variable { token } => return self.environment.borrow().get(token),
            _ => panic!("Not here! Error"),
        }
    }

    fn visit_assign_expr(&mut self, expr: Expr) -> Result<Object, RuntimeError> {
        match expr {
            Expr::Assign { name, value } => {
                let new_value = self.interpret(*value);
                match new_value {
                    Ok(obj) => {
                        self.environment.borrow_mut().assign(name, obj.clone())?;
                        return Ok(obj);
                    }
                    Err(e) => return Err(e),
                }
            }
            _ => panic!("Not here! error visiting assign expression"),
        }
    }

    fn visit_if_stmt(&mut self, stmt: Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::IfStmt {
                condition,
                then_branch,
                else_branch,
            } => match self.interpret(condition) {
                Ok(obj) => {
                    if self.is_truthy(obj) {
                        return self.interpret_stmt(*then_branch);
                    }
                    if else_branch.is_some() {
                        return self.interpret_stmt(*else_branch.unwrap());
                    }
                    Ok(())
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    Err(e.into())
                }
            },
            _ => Ok(()),
        }
    }
}
