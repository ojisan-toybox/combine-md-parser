use combine::{
    parser::{
        char::{space, string},
        range::take_while1,
    },
    ParseError, Parser, RangeStream,
};

use crate::Heading;

pub fn parse_heading_1<'a, Input>() -> impl Parser<Input, Output = Heading<'a>>
where
    Input: RangeStream<Token = char, Range = &'a str>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let heading_content = (
        string("#"),
        space(),
        take_while1(|c: char| c.is_alphabetic()),
    )
        .map(|(_, _, content)| Heading {
            level: 1,
            content: content,
        });
    heading_content
}

pub fn parse_heading_2<'a, Input>() -> impl Parser<Input, Output = Heading<'a>>
where
    Input: RangeStream<Token = char, Range = &'a str>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let heading_content = (
        string("##"),
        space(),
        take_while1(|c: char| c.is_alphabetic()),
    )
        .map(|(_, _, content)| Heading {
            level: 2,
            content: content,
        });
    heading_content
}

pub fn parse_heading_3<'a, Input>() -> impl Parser<Input, Output = Heading<'a>>
where
    Input: RangeStream<Token = char, Range = &'a str>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let heading_content = (
        string("###"),
        space(),
        take_while1(|c: char| c.is_alphabetic()),
    )
        .map(|(_, _, content)| Heading {
            level: 3,
            content: content,
        });
    heading_content
}


#[cfg(test)]
mod tests {
    use crate::ast::{ast::Heading, block::heading::{parse_heading_2, parse_heading_3}};

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
                content: "aaa"
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
                content: "aaa"
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
                content: "aaa"
            }
        );
    }
}
