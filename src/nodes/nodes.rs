use crate::nodes::expressions::ExpressionElement;
use crate::nodes::trivia::TriviaElement;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum KitsuneElement {
    Expression(ExpressionElement),
    Trivia(TriviaElement),
}
