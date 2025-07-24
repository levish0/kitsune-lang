use crate::node::variable::parameter::Parameter;
use crate::parser::function::parameter::parameter_parser::parameter_parser;
use crate::utils::span::Span;
use nom::Parser;
use nom::{
    IResult,
    character::complete::{char, multispace0},
    combinator::complete,
    multi::separated_list0,
    sequence::{delimited, preceded},
};

pub fn parameter_list_parser(input: Span) -> IResult<Span, Vec<Parameter>> {
    let (input, params) = complete(delimited(
        preceded(multispace0, char('(')),
        separated_list0(
            preceded(multispace0, char(',')),
            preceded(multispace0, parameter_parser),
        ),
        preceded(multispace0, char(')')),
    ))
    .parse(input)?;

    Ok((input, params))
}
