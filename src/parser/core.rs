use crate::node::node::KitsuneElement;
use crate::parser::elements::element_parser;
use crate::utils::span::Span;
use nom::IResult;
use nom::Parser;
use nom::multi::many0;

pub fn parse_kitsune(input: &str) -> Vec<KitsuneElement> {
    let spanned_input = Span::new(input);
    let parsed_result = kitsune_parser(spanned_input);
    println!("{:?}", parsed_result);
    match parsed_result {
        Ok((_, parsed_result)) => parsed_result,
        Err(e) => {
            println!("Parse error: {:?}", e);
            Vec::new()
        }
    }
}

pub fn kitsune_parser(input: Span) -> IResult<Span, Vec<KitsuneElement>> {
    many0(|input| element_parser(input))
        .parse(input)
        .map(|(remaining_input, result)| {
            let elements: Vec<KitsuneElement> = result.into_iter().flatten().collect();
            (remaining_input, elements)
        })
}

