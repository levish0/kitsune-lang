use crate::nodes::expressions::expressions::BinaryOperator;
use crate::utils::span::Span;
use nom::branch::alt;
use nom::character::complete::{char, multispace0};
use nom::combinator::map;
use nom::sequence::preceded;
use nom::{IResult, Parser};

pub fn expression_mul_div_mod_op_parser(input: Span) -> IResult<Span, BinaryOperator> {
    let (remaining_input, op) = preceded(
        multispace0,
        alt((
            map(char('*'), |_| BinaryOperator::Multiply),
            map(char('/'), |_| BinaryOperator::Divide),
            map(char('%'), |_| BinaryOperator::Modulo),
        )),
    )
    .parse(input)?;

    Ok((remaining_input, op))
}
