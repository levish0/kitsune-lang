use crate::node::function::function::FunctionElement;
use crate::parser::function::function_declaration::function_declaration_parser;
use crate::utils::span::Span;
use nom::character::complete::multispace0;
use nom::sequence::preceded;
use nom::{IResult, Parser};

pub fn function_parser(input: Span) -> IResult<Span, FunctionElement> {
    let (remaining_input, (func, _)) = (
        preceded(multispace0, function_declaration_parser),
        multispace0,
    )
        .parse(input)?;

    Ok((remaining_input, func))
}
