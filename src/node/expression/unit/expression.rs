use crate::node::expression::expression::ExpressionElement;
use crate::utils::position::Position;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct ExpressionUnitElement {
    pub element: Box<ExpressionElement>,
    #[serde(skip_serializing)]
    pub position: Position,
}
