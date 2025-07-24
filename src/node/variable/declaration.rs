use crate::node::expression::expression::ExpressionElement;
use crate::utils::position::Position;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum VariableDeclarationType {
    Let,   // let -> let mut은 나중에 visitor에서 처리
    Const, // const
}

#[derive(Serialize, Debug, Clone)]
pub struct VariableDeclaration {
    pub declaration_type: VariableDeclarationType,
    pub name: String,
    pub type_annotation: Option<String>,
    pub initializer: Option<ExpressionElement>,
    pub position: Position,
}
