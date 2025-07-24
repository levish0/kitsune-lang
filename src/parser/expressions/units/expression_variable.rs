use crate::nodes::expressions::expressions::ExpressionElement;
use crate::nodes::expressions::units::VariableElement;
use crate::utils::position::make_position;
use crate::utils::span::Span;
use nom::character::complete::alphanumeric1;
use nom::{IResult, Parser};
use nom_locate::position;

pub fn expression_variable_parser(input: Span) -> IResult<Span, ExpressionElement> {
    let (input, start_pos) = position(input)?;
    let (input, element) = alphanumeric1.parse(input)?;
    let (remaining_input, end_pos) = position(input)?;

    let element = element.fragment().to_string();
    let position = make_position(start_pos, end_pos);

    Ok((
        remaining_input,
        ExpressionElement::Variable(VariableElement { element, position }),
    ))
}
