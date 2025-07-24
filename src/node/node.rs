use crate::node::expression::expression::ExpressionElement;
use crate::node::trivia::trivia::TriviaElement;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum KitsuneElement {
    Expression(ExpressionElement),
    Trivia(TriviaElement),
}
