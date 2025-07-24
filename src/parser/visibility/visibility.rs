use crate::node::visibility::visibility::Visibility;
use crate::utils::span::Span;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{eof, map, success};
use nom::sequence::{preceded, terminated};
use nom::{IResult, Parser};

pub fn visibility_parser(input: Span) -> IResult<Span, Visibility> {
    alt((
        map(
            preceded(multispace0, terminated(tag("pub"), alt((multispace1, eof)))),
            |_| Visibility::Public,
        ),
        map(success(()), |_| Visibility::Private),
    ))
    .parse(input)
}
