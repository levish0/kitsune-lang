use logos::Logos;
use kitsune_script::lexer::tokens::Token;

fn main() {
    let source = r#"
        pub fn add(x: i32, y: i32) -> i32 {
            return x + y;
        }
    "#;

    let lexer = Token::lexer(source);

    for token in lexer {
        println!("{:?}", token);
    }
}
