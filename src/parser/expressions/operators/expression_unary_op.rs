use crate::nodes::expressions::expressions::UnaryOperator;
use crate::utils::position::make_position;
use crate::utils::span::Span;
use nom::branch::alt;
use nom::character::complete::{char, multispace0};
use nom::combinator::map;
use nom::sequence::preceded;
use nom::{IResult, Parser};
use nom_locate::position;

pub fn expression_unary_op_parser(input: Span) -> IResult<Span, UnaryOperator> {
    let (remaining_input, op) = preceded(
        multispace0,
        alt((
            map(char('+'), |_| UnaryOperator::Plus),
            map(char('-'), |_| UnaryOperator::Minus),
            map(char('!'), |_| UnaryOperator::Not),
        )),
    )
    .parse(input)?;

    Ok((remaining_input, op))
}
