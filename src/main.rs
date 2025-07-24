use crate::parser::core::parse_kitsune_lang;
use std::fs;
use std::io::{self, Write};
use std::time::Instant;

mod codegen;
mod node;
mod parser;
mod utils;

fn main() -> io::Result<()> {
    let input_path = "main.kitsune";
    let content = fs::read_to_string(input_path)?;
    let start_time = Instant::now();
    let parsed_result = parse_kitsune_lang(&content);

    let duration = start_time.elapsed();

    println!("Time elapsed: {:?}", duration);
    let output_path = "res.json";

    let mut output_file = fs::File::create(output_path)?;
    let parsed_json = serde_json::to_string_pretty(&parsed_result)?;
    write!(output_file, "{}", parsed_json)?;

    Ok(())
}
