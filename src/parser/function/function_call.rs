use crate::node::expression::expression::ExpressionElement;
use crate::node::function::call::FunctionCall;
use crate::parser::expression::expression_parser::expression_parser;
use crate::parser::name::identifier::identifier_parser;
use crate::utils::position::make_position;
use crate::utils::span::Span;
use nom::character::complete::{alpha1, char, multispace0};
use nom::combinator::complete;
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};
use nom_locate::position;

pub fn function_call(input: Span) -> IResult<Span, ExpressionElement> {
    let (input, start_pos) = position(input)?;
    let (input, (function_name, arguments)) = (
        preceded(multispace0, identifier_parser),
        complete(delimited(
            preceded(multispace0, char('(')),
            separated_list0(
                preceded(multispace0, char(',')),
                preceded(multispace0, expression_parser),
            ),
            preceded(multispace0, char(')')),
        )),
    )
        .parse(input)?;
    let (remaining_input, end_pos) = position(input)?;

    let position = make_position(start_pos, end_pos);

    Ok((
        remaining_input,
        ExpressionElement::FunctionCall(FunctionCall {
            function_name,
            arguments,
            position,
        }),
    ))
}
