use combine::{
    between, many,
    parser::char::{char, string},
    satisfy, ParseError, Parser, Stream,
};

use crate::Ast;

fn parse_bold<Input>() -> impl Parser<Input, Output = Ast>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    between(string("**"), string("**"), many(satisfy(|c| true)))
        .map(|name: String| Ast::ContainerBlock)
}

#[cfg(test)]
mod tests {
    use combine::Parser;

    use crate::ast::inline::bold::parse_bold;

    #[test]
    fn it_works() {
        let input = "**hello**";
        let mut parser = parse_bold();
        let res = parser.parse(input);
        println!("res: {:?}", res);
        assert!(res.is_ok());
    }
}
