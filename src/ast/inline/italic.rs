use combine::{between, many, parser::char::string, satisfy, ParseError, Parser, Stream};

use crate::ast::ast::{Inline, Italic};

pub fn parse_italic<'a, Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    between(string("*"), string("*"), many(satisfy(|c| c != '*'))).map(|name: String| {
        let italic = Italic(name);
        Inline::Italic(italic)
    })
}

#[cfg(test)]
mod tests {
    use combine::Parser;

    use crate::ast::{
        ast::{Inline, Italic},
        inline::italic::parse_italic,
    };

    #[test]
    fn it_works() {
        let input = "*hello*";
        let mut parser = parse_italic();
        let res = parser.parse(input);
        let italic = Italic("hello".to_string());
        assert_eq!(res.unwrap().0, Inline::Italic(italic))
    }
}
