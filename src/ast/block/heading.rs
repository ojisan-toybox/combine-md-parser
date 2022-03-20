use combine::{
    parser::{
        char::{space, string},
        range::take_while1,
    },
    ParseError, Parser, RangeStream,
};

use crate::Heading;

pub fn parse_heading<'a, Input>() -> impl Parser<Input, Output = Heading<'a>>
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

#[cfg(test)]
mod tests {
    use crate::ast::ast::Heading;

    use super::parse_heading;
    use combine::Parser;

    #[test]
    fn it_works() {
        let input = "# aaa";
        let mut parser = parse_heading();
        let res = parser.parse(input);
        assert_eq!(
            res.unwrap().0,
            Heading {
                level: 1,
                content: "aaa"
            }
        );
    }
}
