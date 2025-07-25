use kitsune_script::lexer::kitsune_lexer::{Lexer};

fn main() {
    let source = r#"
        pub fn add(xxx: i32, y: i32) -> i32 {
            return x + y;
        }
    "#;

    let lexer = Lexer::new(&source);

    println!("{:?}", lexer.collect::<Vec<_>>());
}
