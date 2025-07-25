use crate::node::function::definition::FunctionDefinition;
use crate::node::function::function::FunctionElement;
use crate::parser::block::block::block_parser;
use crate::parser::function::parameter::parameter_list_parser::parameter_list_parser;
use crate::parser::name::identifier::identifier_parser;
use crate::parser::r#type::type_return::return_type_parser;
use crate::parser::visibility::visibility_parser;
use crate::utils::position::make_position;
use crate::utils::span::Span;
use nom::bytes::tag;
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::opt;
use nom::sequence::{preceded, terminated};
use nom::{IResult, Parser};
use nom_locate::position;

pub fn function_declaration_parser(input: Span) -> IResult<Span, FunctionElement> {
    println!("function_declaration_parser");
    let (input, start_pos) = position(input)?;
    let (input, visibility) = visibility_parser.parse(input)?;
    println!("function_declaration_parser: visibility = {:?}", visibility);
    let (input, _) = multispace0(input)?;
    let (input, name) = terminated(tag("fn"), multispace1).parse(input)?;
    println!("function_declaration: fn");
    let (input, name) = identifier_parser.parse(input)?;
    let (input, parameters) = parameter_list_parser(input)?;
    let (input, return_type) = opt(return_type_parser).parse(input)?;
    let (input, body) = block_parser(input)?;
    let (remaining_input, end_pos) = position(input)?;

    let position = make_position(start_pos, end_pos);

    Ok((
        remaining_input,
        FunctionElement::Definition(FunctionDefinition {
            visibility,
            name,
            parameters,
            return_type,
            body,
            position,
        }),
    ))
}
