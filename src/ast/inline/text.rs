use combine::{between, many, many1, parser::char::string, satisfy, ParseError, Parser, Stream};

use crate::ast::ast::{Bold, Inline, Text};

pub fn parse_text<'a, Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let f = |c| {
        let bool = c != '\n' && c != '[' && c != ']' && c != '(' && c != ')' && c != '*';
        bool
    };
    // HACK: 特別な意味を持つ文字を消費しないようにする(本当にこれしか方法ない？)
    let parsed = many1(satisfy(f)).map(|name: String| {
        let text = Text(name);
        Inline::Text(text)
    });
    parsed
}

#[cfg(test)]
mod tests {
    use combine::Parser;

    use crate::ast::{
        ast::{Inline, Text},
        inline::text::parse_text,
    };

    #[test]
    fn it_works() {
        let input = "hello";
        let mut parser = parse_text();
        let res = parser.parse(input);
        let text = Text("hello".to_string());
        assert_eq!(res.unwrap().0, Inline::Text(text))
    }

    #[test]
    fn it_works_with_sig() {
        let input = "hello [";
        let mut parser = parse_text();
        let res = parser.parse(input);
        let text = Text("hello ".to_string());
        assert_eq!(res.unwrap().0, Inline::Text(text))
    }
}
