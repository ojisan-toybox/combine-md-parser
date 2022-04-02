use combine::{Parser, RangeStream, ParseError, attempt, choice};

use self::{paragraph::parse_paragraph, heading::{parse_heading_1, parse_heading_2, parse_heading_3}};

use super::ast::LeafBlock;

pub mod heading;
pub mod paragraph;

pub fn parse_block<'a, Input>() -> impl Parser<Input, Output = LeafBlock<'a>>
where
    Input: RangeStream<Token = char, Range = &'a str>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let parsers = (
        attempt(parse_paragraph()).map(|p|{LeafBlock::Paragraph(p)}),
        attempt(parse_heading_1()).map(|h1|{LeafBlock::Heading(&h1)}),
        attempt(parse_heading_2()).map(|h2|{LeafBlock::Heading(&h2)}),
        attempt(parse_heading_3()).map(|h3|{LeafBlock::Heading(&h3)}),
    );
    choice(parsers)
}