use crate::nodes::literals::LiteralElement;
use crate::utils::position::Position;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
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

#[derive(Serialize, Debug, Clone)]
pub enum UnaryOperator {
    Plus,  // +x
    Minus, // -x
    Not,   // !x
}

#[derive(Serialize, Debug, Clone)]
pub struct BinaryOperation {
    pub left: Box<ExpressionElement>,
    pub operator: BinaryOperator,
    pub right: Box<ExpressionElement>,
}

#[derive(Serialize, Debug, Clone)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Box<ExpressionElement>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ParenthesizedElement {
    pub element: Box<ExpressionElement>,
    #[serde(skip_serializing)]
    pub position: Position,
}
#[derive(Serialize, Debug, Clone)]
pub struct VariableElement {
    pub element: String,

    #[serde(skip_serializing)]
    pub position: Position,
}
#[derive(Serialize, Debug, Clone)]
pub struct ExpressionUnitElement {
    pub element: Box<ExpressionElement>,
    #[serde(skip_serializing)]
    pub position: Position,
}

#[derive(Serialize, Debug, Clone)]
pub enum ExpressionElement {
    ExpressionUnit(ExpressionUnitElement),
    Literal(LiteralElement),
    Binary(BinaryOperation),
    Unary(UnaryOperation),
    Parenthesized(ParenthesizedElement),
    Variable(VariableElement),
}
