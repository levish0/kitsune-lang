use crate::nodes::expressions::BinaryOperator;
use crate::utils::span::Span;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::map;
use nom::{IResult, Parser};

pub fn expression_logical_op_parser(input: Span) -> IResult<Span, BinaryOperator> {
    let (remaining_input, op) = alt((
        map(tag("&&"), |_| BinaryOperator::And),
        map(tag("||"), |_| BinaryOperator::Or),
    ))
    .parse(input)?;

    Ok((remaining_input, op))
}
