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
