use crate::utils::position::Position;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct VariableElement {
    pub element: String,
    #[serde(skip_serializing)]
    pub position: Position,
}
