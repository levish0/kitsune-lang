use crate::node::expression::literal::literal::LiteralElement;
use crate::node::expression::operation::{BinaryOperation, UnaryOperation};
use crate::node::expression::unit::{ExpressionUnitElement, ParenthesizedElement, VariableElement};
use crate::node::function::call::FunctionCall;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum ExpressionElement {
    FunctionCall(FunctionCall),
    ExpressionUnit(ExpressionUnitElement),
    Literal(LiteralElement),
    Binary(BinaryOperation),
    Unary(UnaryOperation),
    Parenthesized(ParenthesizedElement),
    Variable(VariableElement),
}
