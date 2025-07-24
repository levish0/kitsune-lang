use crate::node::expression::literal::literal::LiteralElement;
use crate::parser::expression::unit::literal::string::escaped_string::escaped_string_parser;
use crate::parser::expression::unit::literal::string::string_text::literal_string_text_parser;
use crate::utils::span::Span;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::multi::many1;

pub fn literal_string_core_parser(input: Span) -> IResult<Span, Vec<LiteralElement>> {
    many1(alt((escaped_string_parser, literal_string_text_parser))).parse(input)
}
