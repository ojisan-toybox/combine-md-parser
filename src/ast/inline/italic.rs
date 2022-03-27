use combine::{parser::char::{char, string}, ParseError, Parser, Stream, between, many, satisfy};

use crate::{Ast, ast::ast::Italic};

fn parse_italic<'a, Input>() -> impl Parser<Input, Output = Italic>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    between(string("*"), string("*"), many(satisfy(|c| c != '*')))
    .map(|name: String| Italic(name))
}

#[cfg(test)]
mod tests {
    use combine::Parser;

    use crate::ast::{inline::italic::parse_italic, ast::Italic};


    #[test]
    fn it_works() {
        let input = "*hello*";
        let mut parser = parse_italic();
        let res = parser.parse(input);
        assert_eq!(
            res.unwrap().0, Italic("hello".to_string())
        )
    }
}