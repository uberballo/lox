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

    //    @Override
    //public Object visitUnaryExpr(Expr.Unary expr) {
    //  Object right = evaluate(expr.right);

    //  switch (expr.operator.type) {
    //case Bang:
    //return !isTruthy(right)
    //    case MINUS:
    //      return -(double)right;
    //  }

    //  // Unreachable.
    //  return null;
    //}

    fn is_truthy(object: Object) -> bool {
        match object {
            Nil => false,
            False => false,
            True => true,
            _ => false,
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
}
