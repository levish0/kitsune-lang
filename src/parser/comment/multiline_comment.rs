use crate::node::trivia::trivia::TriviaElement;
use crate::utils::span::Span;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::complete;
use nom::sequence::delimited;
use nom::{IResult, Parser};

pub fn multiline_comment_parser(input: Span) -> IResult<Span, TriviaElement> {
    let (remaining_input, _) =
        complete(delimited(tag("/*"), take_until("*/"), tag("*/"))).parse(input)?;

    Ok((remaining_input, TriviaElement::Comment))
}
