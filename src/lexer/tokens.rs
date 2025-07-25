use logos::Logos;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(error = LexicalError)]
pub enum Token {
    // 기호
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("=")]
    Eq,
    #[token("==")]
    EqEq,
    #[token("!=")]
    NotEq,
    #[token("<")]
    Lt,
    #[token("<=")]
    Le,
    #[token(">")]
    Gt,
    #[token(">=")]
    Ge,
    #[token("->")]
    Arrow,
    #[token(":")]
    Colon,

    // 키워드
    #[token("let")]
    Let,
    #[token("const")]
    Const,
    #[token("fn")]
    Fn,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("elif")]
    ElseIf,
    #[token("match")]
    Match,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("pub")]
    Pub,
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("false")]
    False,

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        let inner_str = &s[1..s.len() - 1];
        inner_str.to_string()
    })]
    StringLiteral(String),
    // Identifier
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string(), priority = 1)]
    Identifier(String),
    // 부동소수점 리터럴
    #[regex(r"([0-9]+\.[0-9]+|[0-9]+[eE][+-]?[0-9]+)", |lex| lex.slice().parse())]
    FloatLiteral(f64),
    // 정수 리터럴
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    IntegerLiteral(i64),

    #[regex(r"\s+", logos::skip)]
    Whitespace,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    InvalidFloat(ParseFloatError),
    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}

impl From<ParseFloatError> for LexicalError {
    fn from(err: ParseFloatError) -> Self {
        LexicalError::InvalidFloat(err)
    }
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexicalError::InvalidInteger(e) => write!(f, "invalid integer literal: {e}"),
            LexicalError::InvalidFloat(e) => write!(f, "invalid float literal: {e}"),
            LexicalError::InvalidToken => write!(f, "invalid token"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
