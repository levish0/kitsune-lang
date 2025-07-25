#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Variable {
        name: String,
        value: Box<Expression>,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub param_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Variable(String),
    BinaryOperation {
        lhs: Box<Expression>,
        operator: Operator,
        rhs: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Box<Expression>>,
    },
    Block(Block),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    NotEq,
    Lt,
    Le,
    Gt,
    Ge,
}
