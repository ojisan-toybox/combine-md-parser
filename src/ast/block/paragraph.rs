use combine::{
    parser::{
        char::{char, space, string},
        range::take_while1,
    },
    ParseError, Parser, RangeStream,
};

use crate::{ast::ast::Paragraph, Heading};

pub fn parse_paragraph<'a, Input>() -> impl Parser<Input, Output = Paragraph<'a>>
where
    Input: RangeStream<Token = char, Range = &'a str>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let heading_content = (
        char('\n'),
        take_while1(|c: char| c.is_alphabetic()),
        char('\n'),
    )
        .map(|(_, content, _)| Paragraph(content));
    heading_content
}

#[cfg(test)]
mod tests {
    use crate::ast::ast::Paragraph;

    use super::parse_paragraph;
    use combine::Parser;

    #[test]
    fn it_works(){
        // TODO: Use indoc!
        let input = r#"
aaa
"#;
        let mut parser = parse_paragraph();
        let res = parser.parse(input);
        assert_eq!(res.unwrap().0, Paragraph("aaa"));
    }
}
