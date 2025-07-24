use serde::Serialize;
use crate::node::expression::expression::ExpressionElement;
use crate::utils::position::Position;

#[derive(Serialize, Debug, Clone)]
pub struct FunctionCall {
    pub function_name: String,
    pub arguments: Vec<ExpressionElement>,
    pub position: Position,
}