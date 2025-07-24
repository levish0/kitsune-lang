use serde::Serialize;
use crate::node::expression::expression::ExpressionElement;
use crate::node::variable::declaration::VariableDeclaration;

#[derive(Serialize, Debug, Clone)]
pub enum Statement {
    Expression(ExpressionElement),
    VariableDeclaration(VariableDeclaration),
}