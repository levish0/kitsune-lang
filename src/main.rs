use kitsune_script::grammar::kitsune::KitsuneScriptParser;
use kitsune_script::lexer::kitsune_lexer::Lexer;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string("main.kitsune")?;

    let lexer = Lexer::new(&source);
    let parser = KitsuneScriptParser::new();
    let ast = parser.parse(lexer)?;

    println!("{:#?}", ast);

    Ok(())
}
