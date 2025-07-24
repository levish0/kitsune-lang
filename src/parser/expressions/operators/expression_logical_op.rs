use crate::nodes::expressions::expressions::BinaryOperator;
use crate::utils::span::Span;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0};
use nom::combinator::map;
use nom::sequence::preceded;
use nom::{IResult, Parser};

pub fn expression_logical_op_parser(input: Span) -> IResult<Span, BinaryOperator> {
    let (remaining_input, op) = preceded(
        multispace0,
        alt((
            map(tag("&&"), |_| BinaryOperator::And),
            map(tag("||"), |_| BinaryOperator::Or),
        )),
    )
    .parse(input)?;

    Ok((remaining_input, op))
}
