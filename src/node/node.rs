use crate::node::expression::expression::ExpressionElement;
use crate::node::function::function::FunctionElement;
use crate::node::trivia::trivia::TriviaElement;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum KitsuneElement {
    Expression(ExpressionElement),
    Function(FunctionElement),
    Trivia(TriviaElement),
}
