use crate::node::statement::statement::Statement;
use crate::node::variable::declaration::{VariableDeclaration, VariableDeclarationType};
use crate::parser::expression::expression_parser::expression_parser;
use crate::parser::name::identifier::identifier_parser;
use crate::parser::r#type::type_annotation::type_annotation_parser;
use crate::utils::position::make_position;
use crate::utils::span::Span;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::character::complete::{alpha1, char, multispace0, multispace1};
use nom::combinator::{eof, map, opt, peek, value};
use nom::sequence::{preceded, terminated};
use nom_locate::position;

pub fn variable_declaration_parser(input: Span) -> IResult<Span, Statement> {
    let (input, start_pos) = position(input)?;
    println!("variable");
    let (input, declaration_type) = preceded(
        multispace0,
        alt((
            map(terminated(tag("const"), alt((multispace1, eof))), |_| {
                VariableDeclarationType::Const
            }),
            map(terminated(tag("let"), alt((multispace1, eof))), |_| {
                VariableDeclarationType::Let
            }),
        )),
    )
    .parse(input)?;

    println!("before identifier");
    let (input, name) = identifier_parser.parse(input)?;
    println!("identifier");
    let (input, type_annotation) = opt(type_annotation_parser).parse(input)?;

    // 초기화 값 파싱 (= expression)
    let (input, initializer) = opt(preceded(
        (multispace0, char('='), multispace0),
        expression_parser,
    ))
    .parse(input)?;

    let (remaining_input, end_pos) = position(input)?;
    let position = make_position(start_pos, end_pos);

    Ok((
        remaining_input,
        Statement::VariableDeclaration(VariableDeclaration {
            declaration_type,
            name,
            type_annotation,
            initializer,
            position,
        }),
    ))
}
