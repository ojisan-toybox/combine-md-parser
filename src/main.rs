extern crate combine;

use ast::ast::LeafBlock;
use ast::{ast::Document, block::parse_block};
use combine::{attempt, many1};
use combine::{choice, error::ParseError, parser::char::char, Parser, RangeStream, Stream};

mod ast;
use crate::ast::ast::Heading;
use crate::ast::inline::parse_inline;

fn parse_blocks<'a, Input>() -> impl Parser<Input, Output = Vec<LeafBlock>>
where
    Input: RangeStream<Token = char, Range = &'a str>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let block = (parse_block(), char('\n')).map(|(x, _)| x);
    many1(block)
}

fn parse_doc<'a, Input>() -> impl Parser<Input, Output = Document>
where
    Input: RangeStream<Token = char, Range = &'a str>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((
        attempt(parse_blocks()).map(|x| Document::LeafBlocks(x)),
        attempt(parse_inline()).map(|x| Document::Inline(x)),
    ))
}

fn main() {
    let parsed = parse_doc().parse(r#"# h1

## h2

"#);
    println!("{:?}", parsed);
}
