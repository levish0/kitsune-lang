use crate::nodes::expressions::{ExpressionElement, ParenthesizedElement};
use crate::parser::expressions::expression_parser::expression_parser;
use crate::utils::position::make_position;
use crate::utils::span::Span;
use nom::IResult;
use nom::Parser;
use nom::character::complete::char;
use nom::combinator::complete;
use nom::sequence::delimited;
use nom_locate::position;

pub fn expression_parenthesized_parser(input: Span) -> IResult<Span, ExpressionElement> {
    let (input, start_pos) = position(input)?;
    let (input, element) =
        complete(delimited(char('('), expression_parser, char(')'))).parse(input)?;
    let (remaining_input, end_pos) = position(input)?;

    let position = make_position(start_pos, end_pos);

    Ok((
        remaining_input,
        ExpressionElement::Parenthesized(ParenthesizedElement {
            element: Box::new(element),
            position,
        }),
    ))
}
