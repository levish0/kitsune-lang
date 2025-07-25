use crate::node::statement::statement::Statement;
use crate::parser::expression::expression_parser::expression_parser;
use crate::parser::variable::variable_declaration::variable_declaration_parser;
use crate::utils::span::Span;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::character::complete::{char, multispace0};
use nom::combinator::map;

pub fn statement_parser(input: Span) -> IResult<Span, Statement> {
    println!("statement");
    let (remaining_input, (stmt, _, _)) = (
        alt((
            variable_declaration_parser,
            map(expression_parser, Statement::Expression),
        )),
        multispace0,
        char(';'),
    )
        .parse(input)?;

    Ok((remaining_input, stmt))
}
