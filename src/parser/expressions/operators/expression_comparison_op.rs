use crate::nodes::expressions::expressions::BinaryOperator;
use crate::utils::span::Span;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::{IResult, Parser};

pub fn expression_comparison_op_parser(input: Span) -> IResult<Span, BinaryOperator> {
    let (remaining_input, op) = preceded(
        multispace0,
        alt((
            map(tag("<="), |_| BinaryOperator::LessEqual),
            map(tag(">="), |_| BinaryOperator::GreaterEqual),
            map(tag("=="), |_| BinaryOperator::Equal),
            map(tag("!="), |_| BinaryOperator::NotEqual),
            map(char('<'), |_| BinaryOperator::Less),
            map(char('>'), |_| BinaryOperator::Greater),
        )),
    )
    .parse(input)?;

    Ok((remaining_input, op))
}
