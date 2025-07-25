use crate::node::block::block::Block;
use crate::parser::statement::statement::statement_parser;
use crate::utils::position::make_position;
use crate::utils::span::Span;
use nom::IResult;
use nom::Parser;
use nom::character::complete::{char, multispace0};
use nom::combinator::complete;
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom_locate::position;

pub fn block_parser(input: Span) -> IResult<Span, Block> {
    let (input, start_pos) = position(input)?;
    println!("block");
    let (input, statements) = complete(delimited(
        preceded(multispace0, char('{')),
        many0(preceded(multispace0, statement_parser)),
        preceded(multispace0, char('}')),
    ))
    .parse(input)?;

    let (remaining_input, end_pos) = position(input)?;
    let position = make_position(start_pos, end_pos);

    Ok((
        remaining_input,
        Block {
            statements,
            position,
        },
    ))
}
