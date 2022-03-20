extern crate combine;

use combine::{choice, error::ParseError, parser::char::char, Parser, Stream};

mod ast;
use crate::ast::ast::{Ast, Heading};

fn parse_md<'a, Input>() -> impl Parser<Input, Output = Ast<'a>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let parsed = choice((parse_block(), parse_inline()));
    parsed
}

fn parse_inline<'a, Input>() -> impl Parser<Input, Output = Ast<'a>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let tok = char('*').map(|_| Ast::Inline);
    tok
}

fn parse_block<'a, Input>() -> impl Parser<Input, Output = Ast<'a>>
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
