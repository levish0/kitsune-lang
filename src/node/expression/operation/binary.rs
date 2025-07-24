use crate::node::expression::expression::ExpressionElement;
use crate::node::expression::operator::binary::BinaryOperator;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct BinaryOperation {
    pub left: Box<ExpressionElement>,
    pub operator: BinaryOperator,
    pub right: Box<ExpressionElement>,
}
