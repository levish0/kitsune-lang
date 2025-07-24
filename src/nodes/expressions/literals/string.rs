use crate::nodes::expressions::literals::literals::LiteralElement;
use crate::utils::position::Position;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct StringElement {
    pub elements: Vec<LiteralElement>,
    #[serde(skip_serializing)]
    pub position: Position,
}

#[derive(Serialize, Debug, Clone)]
pub struct EscapedStringElement {
    pub element: String,
    #[serde(skip_serializing)]
    pub position: Position,
}
