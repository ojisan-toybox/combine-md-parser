use combine::{parser::char::char, ParseError, Parser, Stream};

use crate::Ast;

fn parse_italic<Input>() -> impl Parser<Input, Output = Ast>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let tok = char('*').map(|_| Ast::Inline);
    tok
}
