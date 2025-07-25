use crate::node::trivia::trivia::TriviaElement;
use crate::utils::span::Span;
use nom::IResult;
use nom::Parser;
use nom::character::complete::multispace1;
use nom::character::multispace0;

pub fn tokens_trivia_parser(input: Span) -> IResult<Span, TriviaElement> {
    let (remaining_input, _) = multispace1.parse(input)?;
    Ok((remaining_input, TriviaElement::Whitespace))
}
