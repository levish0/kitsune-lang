use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // 기호
    #[token("{")] LBrace,
    #[token("}")] RBrace,
    #[token("(")] LParen,
    #[token(")")] RParen,
    #[token(",")] Comma,
    #[token(";")] Semicolon,
    #[token("+")] Plus,
    #[token("-")] Minus,
    #[token("*")] Star,
    #[token("/")] Slash,
    #[token("%")] Percent,
    #[token("=")] Eq,
    #[token("==")] EqEq,
    #[token("!=")] NotEq,
    #[token("<")] Lt,
    #[token("<=")] Le,
    #[token(">")] Gt,
    #[token(">=")] Ge,
    #[token("->")] Arrow,
    #[token(":")] Colon,

    // 키워드
    #[token("let")] Let,
    #[token("const")] Const,
    #[token("fn")] Fn,
    #[token("if")] If,
    #[token("else")] Else,
    #[token("elif")] ElseIf,
    #[token("match")] Match,
    #[token("while")] TyWhile,
    #[token("for")] For,
    #[token("in")] In,
    #[token("pub")] Pub,
    #[token("return")] Return,
    #[token("true")] True,
    #[token("false")] False,

    // 타입
    // int
    #[token("i8")] TyI8,
    #[token("i16")] TyI16,
    #[token("i32")] TyI32,
    #[token("i64")] TyI64,
    #[token("i128")] TyI128,
    #[token("isize")] TyIsize,
    // unsigned int
    #[token("u8")] TyU8,
    #[token("u16")] TyU16,
    #[token("u32")] TyU32,
    #[token("u64")] TyU64,
    #[token("u128")] TyU128,
    #[token("usize")] TyUsize,
    // float
    #[token("f32")] TyF32,
    #[token("f64")] TyF64,
    // boolean
    #[token("bool")] TyBool,
    // string
    #[token("char")] TyChar,
    #[token("str")] TyStr,
    // Identifier
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 1)]
    Identifier,
    // 부동소수점 리터럴
    #[regex(r"([0-9]+\.[0-9]+|[0-9]+[eE][+-]?[0-9]+)")]
    FloatLiteral,
    // 정수 리터럴
    #[regex(r"[0-9]+")]
    IntegerLiteral,

    #[regex(r"\s+", logos::skip)]
    Whitespace,
}