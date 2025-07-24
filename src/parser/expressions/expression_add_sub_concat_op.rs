use crate::nodes::expressions::BinaryOperator;
use crate::utils::span::Span;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::map;
use nom::{IResult, Parser};

pub fn expression_add_sub_concat_op_parser(input: Span) -> IResult<Span, BinaryOperator> {
    let (remaining_input, op) = alt((
        map(tag("++"), |_| BinaryOperator::Concat),
        map(char('+'), |_| BinaryOperator::Add),
        map(char('-'), |_| BinaryOperator::Subtract),
    ))
    .parse(input)?;

    Ok((remaining_input, op))
}
