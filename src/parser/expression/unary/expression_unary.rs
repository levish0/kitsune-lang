use crate::node::expression::expression::ExpressionElement;
use crate::node::expression::operation::UnaryOperation;
use crate::parser::expression::operator::expression_unary_op::expression_unary_op_parser;
use crate::parser::expression::unit::expression_unit_parser::expression_unit_parser;
use crate::utils::span::Span;
use nom::branch::alt;
use nom::combinator::map;
use nom::sequence::pair;
use nom::{IResult, Parser};
use nom_locate::position;

pub fn expression_unary_parser(input: Span) -> IResult<Span, ExpressionElement> {
    let (input, start_pos) = position(input)?;
    let (remaining_input, element) = alt((
        map(
            pair(expression_unary_op_parser, expression_unary_parser),
            |(operator, operand)| {
                ExpressionElement::Unary(UnaryOperation {
                    operator,
                    operand: Box::new(operand),
                })
            },
        ),
        expression_unit_parser,
    ))
    .parse(input)?;

    Ok((remaining_input, element))
}
