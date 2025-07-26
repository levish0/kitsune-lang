use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Statement {
    Let {
        name: String,
        value: Box<Expression>,
    },
    Const {
        visibility: Visibility,
        name: String,
        value: Box<Expression>,
    },
    Static {
        visibility: Visibility,
        name: String,
        value: Box<Expression>,
    },
    For {
        var_name: String,
        iterable: Box<Expression>,
        body: Block,
    },
    While {
        condition: Box<Expression>,
        body: Block,
    },
    Function {
        visibility: Visibility,
        name: String,
        params: Vec<Parameter>,
        return_type: Option<String>,
        body: Block,
    },
    Expression(Box<Expression>),
    Return(Option<Box<Expression>>),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Block {
    pub statements: Vec<Statement>,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Variable(String),
    UnaryOperation {
        operator: UnaryOperator,
        expr: Box<Expression>,
    },
    BinaryOperation {
        lhs: Box<Expression>,
        operator: Operator,
        rhs: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Box<Expression>>,
    },
    Range {
        start: Box<Expression>,
        end: Box<Expression>,
        inclusive: bool,
    },
    Block(Block),
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum UnaryOperator {
    Neg, // -
    Pos, // +
    Not, // !
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
}
