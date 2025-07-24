use serde::Serialize;
use crate::node::expression::expression::ExpressionElement;
use crate::utils::position::Position;

#[derive(Serialize, Debug, Clone)]
pub enum VariableDeclarationType {
    Let,    // let
    Var,    // let mut
    Const,  // const
}

#[derive(Serialize, Debug, Clone)]
pub struct VariableDeclaration {
    pub declaration_type: VariableDeclarationType,
    pub name: String,
    pub type_annotation: Option<String>,
    pub initializer: Option<ExpressionElement>,
    pub position: Position,
}