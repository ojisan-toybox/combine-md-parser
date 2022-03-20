extern crate combine;

use combine::{
    between, choice,
    error::ParseError,
    many,
    parser::char::{char, string},
    satisfy, Parser, Stream,
};

mod ast;
use crate::ast::ast::{Ast, Heading};

fn parse_md<'a, Input>() -> impl Parser<Input, Output = Ast<'a>>
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
