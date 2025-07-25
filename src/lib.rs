mod ast;
mod lexer;
mod parser;

pub mod kitsune {
    include!(concat!(env!("OUT_DIR"), "/kitsune.rs"));
}