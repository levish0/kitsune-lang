use nom::bytes::complete::tag;

use crate::parser::expressions::units::literals::string::string_core::literal_string_core_parser;
use crate::utils::position::make_position;
use crate::utils::span::Span;

use crate::nodes::expressions::expressions::ExpressionElement;
use crate::nodes::expressions::literals::literals::LiteralElement;
use crate::nodes::expressions::literals::string::StringElement;
use nom::IResult;
use nom::Parser;
use nom::combinator::complete;
use nom::sequence::delimited;
use nom_locate::position;

pub fn literal_string_parser(input: Span) -> IResult<Span, ExpressionElement> {
    let (input, start_pos) = position(input)?;
    let (input, elements) =
        complete(delimited(tag("\""), literal_string_core_parser, tag("\""))).parse(input)?;
    let (remaining_input, end_pos) = position(input)?;

    let position = make_position(start_pos, end_pos);

    Ok((
        remaining_input,
        ExpressionElement::Literal(LiteralElement::String(StringElement { elements, position })),
    ))
}
