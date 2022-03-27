use combine::{between, many, parser::char::string, satisfy, ParseError, Parser, Stream};

use crate::ast::ast::{Bold, Inline};

pub fn parse_bold<'a, Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    between(string("**"), string("**"), many(satisfy(|c| c != '*'))).map(|name: String| {
        let bold = Bold(name);
        Inline::Bold(bold)
    })
}

#[cfg(test)]
mod tests {
    use combine::Parser;

    use crate::ast::{
        ast::{Bold, Inline},
        inline::bold::parse_bold,
    };

    #[test]
    fn it_works() {
        let input = "**hello**";
        let mut parser = parse_bold();
        let res = parser.parse(input);
        let bold = Bold("hello".to_string());
        assert_eq!(res.unwrap().0, Inline::Bold(bold))
    }
}
