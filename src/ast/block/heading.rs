use combine::{
    parser::{
        char::{space, string},
        range::take_while1,
    },
    ParseError, Parser, RangeStream,
};

use crate::{
    ast::{ast::Inline, inline::text::parse_text},
    Heading,
};

pub fn parse_heading_1<'a, Input>() -> impl Parser<Input, Output = Heading>
where
    Input: RangeStream<Token = char, Range = &'a str>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let heading_content = (
        string("#"),
        space(),
        // こうすれば take_while 使わなくて良い。
        parse_text(),
    )
        .map(|(_, _, content)| match content {
            Inline::Text(text) => Heading {
                level: 1,
                content: text.0,
            },
            _ => panic!(""),
        });
    heading_content
}

pub fn parse_heading_2<'a, Input>() -> impl Parser<Input, Output = Heading>
where
    Input: RangeStream<Token = char, Range = &'a str>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let heading_content = (
        string("##"),
        space(),
        // こうすれば take_while 使わなくて良い。
        parse_text(),
    )
        .map(|(_, _, content)| match content {
            Inline::Text(text) => Heading {
                level: 2,
                content: text.0,
            },
            _ => panic!(""),
        });
    heading_content
}

pub fn parse_heading_3<'a, Input>() -> impl Parser<Input, Output = Heading>
where
    Input: RangeStream<Token = char, Range = &'a str>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let heading_content = (
        string("###"),
        space(),
        // こうすれば take_while 使わなくて良い。
        parse_text(),
    )
        .map(|(_, _, content)| match content {
            Inline::Text(text) => Heading {
                level: 3,
                content: text.0,
            },
            _ => panic!(""),
        });
    heading_content
}
#[cfg(test)]
mod tests {
    use crate::ast::{
        ast::Heading,
        block::heading::{parse_heading_2, parse_heading_3},
    };

    use super::parse_heading_1;
    use combine::Parser;

    #[test]
    fn it_works_1() {
        let input = "# aaa";
        let mut parser = parse_heading_1();
        let res = parser.parse(input);
        assert_eq!(
            res.unwrap().0,
            Heading {
                level: 1,
                content: "aaa".to_string()
            }
        );
    }

    #[test]
    fn it_works_2() {
        let input = "## aaa";
        let mut parser = parse_heading_2();
        let res = parser.parse(input);
        assert_eq!(
            res.unwrap().0,
            Heading {
                level: 2,
                content: "aaa".to_string()
            }
        );
    }

    #[test]
    fn it_works_3() {
        let input = "### aaa";
        let mut parser = parse_heading_3();
        let res = parser.parse(input);
        assert_eq!(
            res.unwrap().0,
            Heading {
                level: 3,
                content: "aaa".to_string()
            }
        );
    }
}
