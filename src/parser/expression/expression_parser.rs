use crate::node::expression::expression::ExpressionElement;
use crate::node::expression::operation::BinaryOperation;
use crate::parser::expression::operation::expression_comparison::expression_comparison_parser;
use crate::parser::expression::operator::expression_logical_op::expression_logical_op_parser;
use crate::utils::span::Span;
use nom::multi::fold_many0;
use nom::sequence::pair;
use nom::{IResult, Parser};
use nom_locate::position;

pub fn expression_parser(input: Span) -> IResult<Span, ExpressionElement> {
    let (input, start_pos) = position(input)?;
    let (input, first) = expression_comparison_parser(input)?;
    let (input, result) = fold_many0(
        pair(expression_logical_op_parser, expression_comparison_parser),
        || first.clone(),
        |acc, (op, val)| {
            ExpressionElement::Binary(BinaryOperation {
                left: Box::new(acc),
                operator: op,
                right: Box::new(val),
            })
        },
    )
    .parse(input)?;
    let (remaining_input, end_pos) = position(input)?;

    Ok((remaining_input, result))
}
