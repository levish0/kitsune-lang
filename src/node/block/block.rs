use crate::node::statement::statement::Statement;
use crate::utils::position::Position;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub position: Position,
}
