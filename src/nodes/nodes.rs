use crate::nodes::literals::LiteralElement;
use crate::nodes::trivia::TriviaElement;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum KitsuneElement {
    Literal(LiteralElement),
    Trivia(TriviaElement),
}
