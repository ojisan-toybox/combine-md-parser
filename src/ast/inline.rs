use combine::{attempt, choice, ParseError, Parser, Stream};

use self::bold::parse_bold;
use self::italic::parse_italic;
use self::link::parse_anchor;

use super::ast::Inline;

pub mod bold;
pub mod italic;
pub mod link;

fn parse_inline<'a, Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((attempt(parse_bold()), attempt(parse_italic()), attempt(parse_anchor())))
}

#[cfg(test)]
mod tests {
    use combine::Parser;

    use crate::ast::{
        ast::{Bold, Inline, Italic, Anchor},
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
    fn it_works() {
        let input = "[test](http://localhost:3000)";
        let mut parser = parse_inline();
        let res = parser.parse(input);
        let anchor = Anchor {
            title: "test".to_string(),
            link: "http://localhost:3000".to_string(),
        };
        assert_eq!(res.unwrap().0, Inline::Anchor(anchor))
    }
}
