pub use crate::expr::{Expr, LiteralValue};
pub use crate::object::Object;
pub use crate::token::{Token, TokenType};

pub struct Interpreter {}

impl Interpreter {
    //pub fn visit_grouping_expr(expr: Expr) {
    //    self.evaluate(expr.expression)
    //}

    //fn evaluate(expr Expr) -> object {
    //    return expr.accept(this)
    //}

    pub fn interpret(&self, expr: Expr) {
        let res = match expr {
            Expr::Binary {
                left: _,
                operator: _,
                right: _,
            } => self.visit_binary_expr(expr),
            Expr::Unary {
                operator: _,
                right: _,
            } => self.visit_unary_expr(expr),
            Expr::Literal { literalValue: _ } => self.visit_literal_expr(expr),
            _ => Object::Nil,
        };
        println!("{:?}", res);
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

    fn literal_to_object(&self, literalValue: LiteralValue) -> Object {
        match literalValue {
            LiteralValue::Number(x) => Object::Number(x),
            LiteralValue::String(x) => Object::String(x),
            LiteralValue::Boolean(x) => Object::Boolean(x),
            LiteralValue::Null => Object::Nil,
        }
    }

    fn visit_literal_expr(&self, expr: Expr) -> Object {
        match expr {
            Expr::Literal {
                literalValue: value,
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

    fn visit_binary_expr(&self, expr: Expr) -> Object {
        match expr {
            Expr::Binary {
                left: left,
                operator: operator,
                right: right,
            } => match operator.tokenType {
                TokenType::Minus => {
                    let left_value = self.object_number(self.visit_literal_expr(*left));
                    let right_value = self.object_number(self.visit_literal_expr(*right));
                    Object::Number(left_value - right_value)
                }
                TokenType::Plus => {
                    let left_value = self.object_number(self.visit_literal_expr(*left));
                    let right_value = self.object_number(self.visit_literal_expr(*right));
                    Object::Number(left_value + right_value)
                }
                TokenType::Slash => {
                    let left_value = self.object_number(self.visit_literal_expr(*left));
                    let right_value = self.object_number(self.visit_literal_expr(*right));
                    Object::Number(left_value / right_value)
                }
                TokenType::Star => {
                    let left_value = self.object_number(self.visit_literal_expr(*left));
                    let right_value = self.object_number(self.visit_literal_expr(*right));
                    Object::Number(left_value * right_value)
                }
                TokenType::Greater => {
                    let left_value = self.object_number(self.visit_literal_expr(*left));
                    let right_value = self.object_number(self.visit_literal_expr(*right));
                    Object::Boolean(left_value > right_value)
                }
                TokenType::GreaterEqual => {
                    let left_value = self.object_number(self.visit_literal_expr(*left));
                    let right_value = self.object_number(self.visit_literal_expr(*right));
                    Object::Boolean(left_value >= right_value)
                }
                TokenType::Less => {
                    let left_value = self.object_number(self.visit_literal_expr(*left));
                    let right_value = self.object_number(self.visit_literal_expr(*right));
                    Object::Boolean(left_value < right_value)
                }
                TokenType::LessEqual => {
                    let left_value = self.object_number(self.visit_literal_expr(*left));
                    let right_value = self.object_number(self.visit_literal_expr(*right));
                    Object::Boolean(left_value <= right_value)
                }
                TokenType::BangEqual => {
                    let left_value = self.visit_literal_expr(*left);
                    let right_value = self.visit_literal_expr(*right);
                    Object::Boolean(!self.is_equal(left_value, right_value))
                }
                TokenType::EqualEqual => {
                    let left_value = self.visit_literal_expr(*left);
                    let right_value = self.visit_literal_expr(*right);
                    Object::Boolean(self.is_equal(left_value, right_value))
                }
                //TODO No string + string yet.
                _ => Object::Nil,
            },
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
}
