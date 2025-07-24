use crate::nodes::nodes::KitsuneElement;
use crate::parser::comments::inline_comment::inline_comment_parser;
use crate::parser::comments::multiline_comment::multiline_comment_parser;
use crate::parser::expressions::expression_parser::expression_parser;
use crate::parser::tokens::trivia::tokens_trivia_parser;
use crate::utils::span::Span;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::multi::many1;

pub fn element_parser(input: Span) -> IResult<Span, Vec<KitsuneElement>> {
    many1(alt((
        // Comment
        multiline_comment_parser.map(KitsuneElement::Trivia),
        inline_comment_parser.map(KitsuneElement::Trivia),
        // Expression
        expression_parser.map(KitsuneElement::Expression),
        // Trivia
        tokens_trivia_parser.map(KitsuneElement::Trivia),
    )))
    .parse(input)
}
