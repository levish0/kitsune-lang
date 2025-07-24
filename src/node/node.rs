use crate::node::expression::expression::ExpressionElement;
use serde::Serialize;
use crate::node::trivia::trivia::TriviaElement;

#[derive(Debug, Serialize)]
pub enum KitsuneElement {
    Expression(ExpressionElement),
    Trivia(TriviaElement),
}
