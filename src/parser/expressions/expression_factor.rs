use crate::nodes::expressions::{BinaryOperation, ExpressionElement};
use crate::parser::expressions::expression_mul_div_mod_op::expression_mul_div_mod_op_parser;
use crate::parser::expressions::expression_unary::expression_unary_parser;
use crate::utils::span::Span;
use nom::multi::fold_many0;
use nom::sequence::pair;
use nom::{IResult, Parser};
use nom_locate::position;

pub fn expression_factor_parser(input: Span) -> IResult<Span, ExpressionElement> {
    let (input, start_pos) = position(input)?;
    let (input, first) = expression_unary_parser(input)?;
    let (remaining_input, result) = fold_many0(
        pair(expression_mul_div_mod_op_parser, expression_unary_parser),
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
