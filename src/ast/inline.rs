use combine::{choice, ParseError, Parser, Stream};

use self::bold::parse_bold;
use self::italic::parse_italic;

use super::ast::Inline;

pub mod bold;
pub mod italic;

fn parse_inline<'a, Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((parse_bold(), parse_italic()))
}

#[cfg(test)]
mod tests {
    use combine::Parser;

    use crate::ast::{
        ast::{Bold, Inline, Italic},
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
}
