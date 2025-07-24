use crate::nodes::expressions::expressions::ExpressionElement;
use crate::nodes::literals::{LiteralElement, NumberElement};
use crate::utils::position::{Position, make_position};
use crate::utils::span::Span;
use nom::IResult;
use nom::Parser;
use nom::number::complete::recognize_float;
use nom_locate::position;

pub fn literal_number_parser(input: Span) -> IResult<Span, ExpressionElement> {
    let (input, start_pos) = position(input)?;
    let (input, element) = recognize_float.parse(input)?;
    let (remaining_input, end_pos) = position(input)?;

    let element = element.fragment().to_string();
    let position = make_position(start_pos, end_pos);

    Ok((
        remaining_input,
        ExpressionElement::Literal(LiteralElement::Number(NumberElement { element, position })),
    ))
}
