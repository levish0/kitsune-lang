use kitsune_script::grammar::kitsune::KitsuneScriptParser;
use kitsune_script::lexer::kitsune_lexer::Lexer;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string("main.kitsune")?;

    let start = Instant::now();

    let lexer = Lexer::new(&source);
    let parser = KitsuneScriptParser::new();
    let ast = parser.parse(lexer)?;

    let duration = start.elapsed();

    let json = serde_json::to_string_pretty(&ast)?;
    let mut file = File::create("ast.json")?;
    file.write_all(json.as_bytes())?;

    println!("AST written to ast.json");
    println!("Parsing took: {:?} secs", duration);

    Ok(())
}
