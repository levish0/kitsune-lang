use crate::nodes::expressions::{ExpressionElement, ParenthesizedElement, VariableElement};
use crate::utils::position::make_position;
use crate::utils::span::Span;
use nom::IResult;
use nom::Parser;
use nom::character::complete::{alphanumeric1, char};
use nom::character::multispace0;
use nom::combinator::complete;
use nom::sequence::{delimited, preceded};
use nom_locate::position;

pub fn expression_parenthesized_parser(input: Span) -> IResult<Span, ExpressionElement> {
    let (input, start_pos) = position(input)?;
    let (input, element) = complete(delimited(char('('), todo!(), char(')'))).parse(input)?;
    let (remaining_input, end_pos) = position(input)?;

    let element = element.fragment().to_string();
    let position = make_position(start_pos, end_pos);

    Ok((
        remaining_input,
        ExpressionElement::Parenthesized(ParenthesizedElement { element, position }),
    ))
}
