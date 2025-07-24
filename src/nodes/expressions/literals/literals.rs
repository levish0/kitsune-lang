use crate::nodes::expressions::literals::number::NumberElement;
use crate::nodes::expressions::literals::string::{EscapedStringElement, StringElement};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum LiteralElement {
    Number(NumberElement),
    String(StringElement),
    PlainString(String),
    EscapedString(EscapedStringElement),
}
