use crate::node::block::block::Block;
use crate::node::variable::parameter::Parameter;
use crate::node::visibility::visibility::Visibility;
use crate::utils::position::Position;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct FunctionDefinition {
    pub visibility: Visibility,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<String>,
    pub body: Block,
    pub position: Position,
}
