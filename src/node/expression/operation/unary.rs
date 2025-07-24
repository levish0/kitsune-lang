use crate::node::expression::expression::ExpressionElement;
use crate::node::expression::operator::unary::UnaryOperator;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Box<ExpressionElement>,
}
