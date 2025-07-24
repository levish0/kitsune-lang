use crate::utils::position::Position;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct NumberElement {
    pub element: String,
    #[serde(skip_serializing)]
    pub position: Position,
}
