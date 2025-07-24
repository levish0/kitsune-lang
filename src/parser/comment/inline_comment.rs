
use crate::utils::span::Span;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::line_ending;
use nom::combinator::{eof, opt};
use nom::sequence::{preceded, terminated};
use nom::{IResult, Parser};
use crate::node::trivia::trivia::TriviaElement;

pub fn inline_comment_parser(input: Span) -> IResult<Span, TriviaElement> {
    let (remaining_input, _) = preceded(
        tag("//"),
        terminated(opt(is_not("\n")), alt((line_ending, eof))),
    )
    .parse(input)?;

    Ok((remaining_input, TriviaElement::Comment))
}
