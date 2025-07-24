use crate::nodes::expressions::expressions::ExpressionElement;
use crate::nodes::expressions::operators::binary::BinaryOperator;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct BinaryOperation {
    pub left: Box<ExpressionElement>,
    pub operator: BinaryOperator,
    pub right: Box<ExpressionElement>,
}
