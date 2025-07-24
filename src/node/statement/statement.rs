use crate::node::expression::expression::ExpressionElement;
use crate::node::variable::declaration::VariableDeclaration;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum Statement {
    Expression(ExpressionElement),
    VariableDeclaration(VariableDeclaration),
}
