use crate::utils::span::Span;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1};
use nom::multi::many0;

pub fn identifier_parser(input: Span) -> IResult<Span, String> {
    let (input, first) = alpha1(input)?;
    let (remaining_input, rest) = many0(alt((alphanumeric1, tag("_")))).parse(input)?;

    let mut result = first.fragment().to_string();
    for part in rest {
        result.push_str(part.fragment());
    }

    Ok((remaining_input, result))
}
