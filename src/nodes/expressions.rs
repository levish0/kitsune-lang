use crate::nodes::literals::LiteralElement;
use crate::utils::position::Position;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum BinaryOperator {
    // 산술 연산
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    // 문자열 연결
    Concat,

    // 비교 연산
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    // 논리 연산
    And,
    Or,
}

#[derive(Serialize, Debug)]
pub enum UnaryOperator {
    Plus,  // +x
    Minus, // -x
    Not,   // !x
}

#[derive(Serialize, Debug)]
pub struct BinaryOperation {
    pub left: Box<ExpressionElement>,
    pub operator: BinaryOperator,
    pub right: Box<ExpressionElement>,
    pub position: Position,
}

#[derive(Serialize, Debug)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Box<ExpressionElement>,
    pub position: Position,
}

#[derive(Serialize, Debug)]
pub struct VariableElement {
    pub element: String,
    pub position: Position,
}

#[derive(Serialize, Debug)]
pub enum ExpressionElement {
    Literal(LiteralElement),
    Binary(BinaryOperation),
    Unary(UnaryOperation),
    Variable(VariableElement),
}
