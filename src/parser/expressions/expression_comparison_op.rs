use crate::nodes::expressions::BinaryOperator;
use crate::utils::span::Span;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::map;
use nom::{IResult, Parser};

pub fn expression_comparison_op_parser(input: Span) -> IResult<Span, BinaryOperator> {
    let (remaining_input, op) = alt((
        map(tag("<="), |_| BinaryOperator::LessEqual),
        map(tag(">="), |_| BinaryOperator::GreaterEqual),
        map(tag("=="), |_| BinaryOperator::Equal),
        map(tag("!="), |_| BinaryOperator::NotEqual),
        map(char('<'), |_| BinaryOperator::Less),
        map(char('>'), |_| BinaryOperator::Greater),
    ))
    .parse(input)?;

    Ok((remaining_input, op))
}
