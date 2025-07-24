use serde::Serialize;
use crate::node::statement::statement::Statement;
use crate::utils::position::Position;

#[derive(Serialize, Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub position: Position,
}