use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,

    #[token(",")]
    Comma,

    #[regex(r"\s+", logos::skip)]
    Whitespace,

    #[error]
    Error,
}