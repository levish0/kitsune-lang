use crate::node::expression::expression::ExpressionElement;
use crate::node::function::call::FunctionCall;
use crate::node::function::definition::FunctionDefinition;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum FunctionElement {
    Expression(ExpressionElement),
    Definition(FunctionDefinition),
}
