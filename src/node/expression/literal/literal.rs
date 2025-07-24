use crate::node::expression::literal::number::NumberElement;
use crate::node::expression::literal::string::{EscapedStringElement, StringElement};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum LiteralElement {
    Number(NumberElement),
    String(StringElement),
    PlainString(String),
    EscapedString(EscapedStringElement),
}
