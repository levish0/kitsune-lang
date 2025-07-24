use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum UnaryOperator {
    Plus,  // +x
    Minus, // -x
    Not,   // !x
}
