use combine::{between, many, parser::char::string, satisfy, ParseError, Parser, Stream};

use crate::{Ast, ast::ast::Bold};

fn parse_bold<'a, Input>() -> impl Parser<Input, Output = Bold>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    between(string("**"), string("**"), many(satisfy(|c| c != '*')))
        .map(|name: String| Bold(name))
}

#[cfg(test)]
mod tests {
    use combine::Parser;

    use crate::ast::{inline::bold::parse_bold, ast::Bold};

    #[test]
    fn it_works() {
        let input = "**hello**";
        let mut parser = parse_bold();
        let res = parser.parse(input);
        assert_eq!(
            res.unwrap().0, Bold("hello".to_string())
        )
    }
}
