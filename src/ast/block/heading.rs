use combine::{
    parser::{
        char::{space, string},
        range::take_while1,
    },
    ParseError, Parser, Stream,
};

use crate::Heading;

fn parse_heading<Input>() -> impl Parser<Input, Output = Heading>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    // TODO: compile 通らない
    let alphabet = take_while1(|c: char| c.is_alphabetic());
    (string("#"), space(), string("aaa")).map(|(level, _, content)| Heading {
        level: 1,
        content: content.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::parse_heading;
    use combine::Parser;

    #[test]
    fn it_works() {
        let input = "# aaa";
        let mut parser = parse_heading();
        let res = parser.parse(input);
        println!("res: {:?}", res);
        assert!(res.is_ok());
    }
}
