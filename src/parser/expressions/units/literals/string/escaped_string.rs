use crate::nodes::expressions::literals::literals::LiteralElement;
use crate::nodes::expressions::literals::string::EscapedStringElement;
use crate::utils::position::make_position;
use crate::utils::span::Span;
use nom::character::complete::{anychar, char};
use nom::sequence::preceded;
use nom::{IResult, Parser};
use nom_locate::position;

pub fn escaped_string_parser(input: Span) -> IResult<Span, LiteralElement> {
    let (input, start_pos) = position(input)?;
    let (input, value) = preceded(char('\\'), anychar).parse(input)?;
    let (remaining_input, end_pos) = position(input)?;

    let position = make_position(start_pos, end_pos);

    Ok((
        remaining_input,
        LiteralElement::EscapedString(EscapedStringElement {
            element: String::from(value),
            position,
        }),
    ))
}
