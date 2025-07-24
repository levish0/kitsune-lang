use crate::nodes::expressions::expressions::ExpressionElement;
use crate::nodes::expressions::units::ExpressionUnitElement;
use crate::parser::expressions::units::expression_parenthesized::expression_parenthesized_parser;
use crate::parser::expressions::units::expression_variable::expression_variable_parser;
use crate::parser::expressions::units::literals::literal_number::literal_number_parser;
use crate::parser::expressions::units::literals::literal_string::literal_string_parser;
use crate::utils::position::make_position;
use crate::utils::span::Span;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::sequence::preceded;
use nom::{IResult, Parser};
use nom_locate::position;

pub fn expression_unit_parser(input: Span) -> IResult<Span, ExpressionElement> {
    let (input, start_pos) = position(input)?;
    let (input, element) = preceded(
        multispace0,
        alt((
            expression_parenthesized_parser,
            literal_number_parser,
            literal_string_parser,
            expression_variable_parser,
        )),
    )
    .parse(input)?;
    let (remaining_input, end_pos) = position(input)?;

    let position = make_position(start_pos, end_pos);

    Ok((
        remaining_input,
        ExpressionElement::ExpressionUnit(ExpressionUnitElement {
            element: Box::new(element),
            position,
        }),
    ))
}
