use crate::ast::{ast::Paragraph, inline::parse_inline};
use combine::{parser::char::char, ParseError, Parser, RangeStream};

pub fn parse_paragraph<'a, Input>() -> impl Parser<Input, Output = Paragraph>
where
    Input: RangeStream<Token = char, Range = &'a str>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let heading_content =
        (char('\n'), parse_inline(), char('\n')).map(|(_, content, _)| Paragraph(content));
    heading_content
}

#[cfg(test)]
mod tests {
    use crate::ast::ast::{Inline, Paragraph, Text};

    use super::parse_paragraph;
    use combine::Parser;

    #[test]
    fn it_works() {
        // TODO: Use indoc!
        let input = r#"
aaa
"#;
        let mut parser = parse_paragraph();
        let res = parser.parse(input);
        let text = Text("aaa".to_string());
        let inline = Inline::Text(text);
        let paragraph = Paragraph(inline);
        assert_eq!(res.unwrap().0, paragraph);
    }
}
