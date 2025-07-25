use crate::node::variable::parameter::Parameter;
use crate::parser::name::identifier::identifier_parser;
use crate::parser::r#type::type_annotation::type_annotation_parser;
use crate::utils::position::make_position;
use crate::utils::span::Span;
use nom::IResult;
use nom::Parser;
use nom::character::complete::alpha1;
use nom::combinator::opt;
use nom_locate::position;

pub fn parameter_parser(input: Span) -> IResult<Span, Parameter> {
    let (input, start_pos) = position(input)?;
    let (input, name) = identifier_parser.parse(input)?;
    let (input, type_annotation) = opt(type_annotation_parser).parse(input)?;
    let (remaining_input, end_pos) = position(input)?;

    let position = make_position(start_pos, end_pos);

    Ok((
        remaining_input,
        Parameter {
            name,
            type_annotation,
            position,
        },
    ))
}
