use crate::node::node::KitsuneElement;
use crate::parser::comment::inline_comment::inline_comment_parser;
use crate::parser::comment::multiline_comment::multiline_comment_parser;
use crate::parser::expression::expression_parser::expression_parser;
use crate::parser::function::function_parser::function_parser;
use crate::parser::token::trivia::tokens_trivia_parser;
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
        tokens_trivia_parser.map(KitsuneElement::Trivia),
        // Function
        function_parser.map(KitsuneElement::Function),
        // Expression
        expression_parser.map(KitsuneElement::Expression),
    )))
    .parse(input)
}
