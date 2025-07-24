use serde::Serialize;
use crate::utils::position::Position;

#[derive(Serialize, Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<String>,
    pub position: Position,
}