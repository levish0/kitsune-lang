use kitsune_script::kitsune;

fn main() {
    match kitsune::ExprParser::new().parse("2 + 3 * 4") {
        Ok(v)  => println!("Result: {v}"),
        Err(e) => println!("Parse error: {e:?}"),
    }
}