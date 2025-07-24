use crate::utils::span::Span;
use nom::IResult;
use nom::Parser;
use nom::character::complete::{alpha1, alphanumeric1, char, multispace0};
use nom::combinator::map;
use nom::sequence::{pair, preceded};

pub fn type_annotation_parser(input: Span) -> IResult<Span, String> {
    map(
        preceded(
            preceded(multispace0, char(':')),
            preceded(multispace0, alphanumeric1),
        ),
        |type_span: Span| type_span.fragment().to_string(),
    )
    .parse(input)
}
