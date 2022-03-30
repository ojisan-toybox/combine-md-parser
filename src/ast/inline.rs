use combine::{attempt, choice, ParseError, Parser, Stream};

use self::bold::parse_bold;
use self::italic::parse_italic;
use self::link::parse_anchor;
use self::text::parse_text;

use super::ast::Inline;

pub mod bold;
pub mod italic;
pub mod link;
pub mod text;

fn parse_inline<'a, Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((
        attempt(parse_bold()),
        attempt(parse_italic()),
        attempt(parse_anchor()),
        attempt(parse_text())
    ))
}

#[cfg(test)]
mod tests {
    use combine::Parser;

    use crate::ast::{
        ast::{Anchor, Bold, Inline, Italic, Text},
        inline::parse_inline,
    };

    #[test]
    fn it_works_bold() {
        let input = "**hello**";
        let mut parser = parse_inline();
        let res = parser.parse(input);
        let bold = Bold("hello".to_string());
        assert_eq!(res.unwrap().0, Inline::Bold(bold))
    }

    #[test]
    fn it_works_italic() {
        let input = "*hello*";
        let mut parser = parse_inline();
        let res = parser.parse(input);
        let italic = Italic("hello".to_string());
        assert_eq!(res.unwrap().0, Inline::Italic(italic))
    }

    #[test]
    fn it_works_link() {
        let input = "hello";
        let mut parser = parse_inline();
        let res = parser.parse(input);
        let text = Text("hello".to_string());
        assert_eq!(res.unwrap().0, Inline::Text(text))
    }

    #[test]
    fn it_works_text() {
        let input = "aaa";
        let mut parser = parse_inline();
        let res = parser.parse(input);
        let anchor = Anchor {
            title: "test".to_string(),
            link: "http://localhost:3000".to_string(),
        };
        assert_eq!(res.unwrap().0, Inline::Anchor(anchor))
    }
}
