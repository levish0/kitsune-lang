use crate::utils::position::Position;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct NumberElement {
    pub element: String,
    pub position: Position,
}

#[derive(Serialize, Debug, Clone)]
pub struct StringElement {
    pub elements: Vec<LiteralElement>,
    pub position: Position,
}
#[derive(Serialize, Debug, Clone)]
pub struct EscapedStringElement {
    pub element: String,
    pub position: Position,
}
#[derive(Serialize, Debug, Clone)]
pub enum LiteralElement {
    Number(NumberElement),
    String(StringElement),
    PlainString(String),
    EscapedString(EscapedStringElement),
}
