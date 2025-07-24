use crate::nodes::expressions::literals::literals::LiteralElement;
use crate::nodes::expressions::operations::{BinaryOperation, UnaryOperation};
use crate::nodes::expressions::units::{
    ExpressionUnitElement, ParenthesizedElement, VariableElement,
};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum ExpressionElement {
    ExpressionUnit(ExpressionUnitElement),
    Literal(LiteralElement),
    Binary(BinaryOperation),
    Unary(UnaryOperation),
    Parenthesized(ParenthesizedElement),
    Variable(VariableElement),
}
