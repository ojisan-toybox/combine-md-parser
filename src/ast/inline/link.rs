use combine::{
    between, many,
    parser::char::{char, string},
    satisfy, ParseError, Parser, Stream,
};

use crate::ast::ast::{Anchor, Inline, Italic};

pub fn parse_anchor<'a, Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let title = between(char('['), char(']'), many(satisfy(|c| c != ']')));
    let link = between(char('('), char(')'), many(satisfy(|c| c != ')')));
    (title, link).map(|(title, link)| {
        let a = Anchor { link, title };
        Inline::Anchor(a)
    })
}

#[cfg(test)]
mod tests {
    use combine::Parser;

    use crate::ast::{
        ast::{Anchor, Inline},
        inline::link::parse_anchor,
    };

    #[test]
    fn it_works() {
        let input = "[test](http://localhost:3000)";
        let mut parser = parse_anchor();
        let res = parser.parse(input);
        let anchor = Anchor {
            title: "test".to_string(),
            link: "http://localhost:3000".to_string(),
        };
        assert_eq!(res.unwrap().0, Inline::Anchor(anchor))
    }
}
