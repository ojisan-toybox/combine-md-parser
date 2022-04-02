use combine::parser::char::{char, spaces};
use combine::{attempt, choice, many, opaque, ParseError, Parser, Stream, parser::char::space};

use self::bold::parse_bold;
use self::italic::parse_italic;
use self::link::parse_anchor;
use self::text::parse_text;

use super::ast::Inline;

pub mod bold;
pub mod italic;
pub mod link;
pub mod text;

pub fn parse_inline<'a, Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((
        attempt(parse_bold().skip(spaces())),
        attempt(parse_italic().skip(spaces())),
        attempt(parse_anchor().skip(spaces())),
        attempt(parse_text().skip(spaces())),
    ))
}

pub fn parse_inlines<'a, Input>() -> impl Parser<Input, Output = Vec<Inline>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many(parse_inline())
}

fn experiment<'a, Input>() -> impl Parser<Input, Output = Vec<char>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
        many(choice(
            (attempt(char('b')), attempt(char('c')))
        ))
}

#[cfg(test)]
mod tests {
    use combine::Parser;

    use crate::ast::{
        ast::{Anchor, Bold, Inline, Italic, Text},
        inline::{parse_inline, experiment},
    };

    use super::parse_inlines;

    #[test]
    fn it_works_ex() {
        let input = "bcc";
        let mut parser = experiment();
        let res = parser.parse(input);
        assert_eq!(res.unwrap().0, vec!['b', 'c', 'c'])
    }

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
        let input = "[test](http://localhost:3000)";
        let mut parser = parse_inline();
        let res = parser.parse(input);
        let anchor = Anchor {
            title: "test".to_string(),
            link: "http://localhost:3000".to_string(),
        };
        assert_eq!(res.unwrap().0, Inline::Anchor(anchor))
    }

    #[test]
    fn it_works_text() {
        let input = "hello world";
        let mut parser = parse_inline();
        let res = parser.parse(input);
        let text = Text("hello world".to_string());
        assert_eq!(res.unwrap().0, Inline::Text(text))
    }

    #[test]
    fn it_works_inlines_with_single() {
        let input = "*italic*";
        let mut parser = parse_inlines();
        let res = parser.parse(input);
        let italic = Italic("italic".to_string());
        let inline_italic = Inline::Italic(italic);
        let inlines = vec![
            inline_italic,
        ];
        assert_eq!(res.unwrap().0, inlines);
    }

    #[test]
    fn it_works_inlines() {
        let input = "aaa [hoge](http://localhost:3000) *italic* is not **bold**";
        let mut parser = parse_inlines();
        let res = parser.parse(input);
        let text = Text("aaa".to_string());
        let inline_text = Inline::Text(text);
        let anchor = Anchor {
            title: "hoge".to_string(),
            link: "http://localhost:3000".to_string(),
        };
        let italic = Italic("italic".to_string());
        let inline_italic = Inline::Italic(italic);
        let inline_link = Inline::Anchor(anchor);
        let text2 = Text("is not".to_string());
        let inline_text2 = Inline::Text(text2);
        let bold = Bold("italic".to_string());
        let inline_bold = Inline::Bold(bold);
        let inlines = vec![
            inline_text,
            inline_link,
            inline_italic,
            inline_text2,
            inline_bold,
        ];
        assert_eq!(res.unwrap().0, inlines);
    }

    #[test]
    fn it_works_inlines_without_text() {
        let input = "[hoge](http://localhost:3000) *italic*";
        let mut parser = parse_inlines();
        let res = parser.parse(input);
        let anchor = Anchor {
            title: "hoge".to_string(),
            link: "http://localhost:3000".to_string(),
        };
        let italic = Italic("italic".to_string());
        let inline_italic = Inline::Italic(italic);
        let inline_link = Inline::Anchor(anchor);
        let inlines = vec![
            inline_link,
            inline_italic,
        ];
        assert_eq!(res.unwrap().0, inlines);
    }
}
