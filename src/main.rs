#[macro_use]
extern crate combine;
use std::collections::HashMap;

use combine::{
    between, chainl1, choice,
    error::ParseError,
    many, many1,
    parser::{
        char::{char, digit, letter, spaces, string},
        range::take_while1,
    },
    satisfy, token, Parser, Stream,
};

mod ast;

enum Inline {
    Anchor,
    Strong,
    Italic,
}
#[derive(Debug)]
pub struct Paragraph {
    content: String,
}
#[derive(Debug)]
pub struct Heading {
    content: String,
    level: u8,
}

#[derive(Debug)]
pub enum LeafBlock {
    LeafBlock,
    Inline,
    Paragraph(Paragraph),
    Heading(Heading),
}

enum ContainerBlock {
    BlockQuotes(LeafBlock),
    ListItems,
}

#[derive(Debug)]
enum Ast {
    LeafBlock(LeafBlock),
    ContainerBlock,
    Inline,
}

fn parse_md<Input>() -> impl Parser<Input, Output = Ast>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let mut sum = choice((parse_block(), parse_inline()));
    sum
}

fn parse_bold<Input>() -> impl Parser<Input, Output = String>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    between(string("**"), string("**"), many(satisfy(|c| true))).map(|name: String| name)
}

fn parse_inline<Input>() -> impl Parser<Input, Output = Ast>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let tok = char('*').map(|_| Ast::Inline);
    tok
}

fn parse_block<Input>() -> impl Parser<Input, Output = Ast>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let tok = char('*').map(|_| Ast::Inline);
    tok
}

fn main() {
    let res = parse_md().parse("3*(1+2)");
    println!("{:?}", res)
    // Ok((Prod(Scalar(3.0), Sum(Scalar(1.0), Scalar(2.0))), ""))
}
