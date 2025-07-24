use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum TriviaElement {
    Whitespace,
    Comment,
}
