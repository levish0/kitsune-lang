use crate::nodes::expressions::expressions::ExpressionElement;
use crate::nodes::expressions::operators::unary::UnaryOperator;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Box<ExpressionElement>,
}
