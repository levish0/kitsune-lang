use crate::nodes::literals::LiteralElement;
use crate::nodes::literals::LiteralElement::PlainString;
use crate::utils::span::Span;
use nom::IResult;
use nom::Parser;
use nom::character::complete::none_of;
use nom::multi::many1;

pub fn literal_string_text_parser(input: Span) -> IResult<Span, LiteralElement> {
    let (remaining_input, elements) = many1(none_of("\"\\")).parse(input)?;
    Ok((remaining_input, PlainString(elements.into_iter().collect())))
}
