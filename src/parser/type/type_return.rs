use crate::utils::span::Span;
use nom::Parser;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0},
    sequence::preceded,
};

pub fn return_type_parser(input: Span) -> IResult<Span, String> {
    let (input, type_span) = preceded(
        preceded(multispace0, tag("->")),
        preceded(multispace0, alphanumeric1),
    )
    .parse(input)?;

    Ok((input, type_span.fragment().to_string()))
}
