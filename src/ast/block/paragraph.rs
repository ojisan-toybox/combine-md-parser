use crate::ast::{
    ast::{Inline, Paragraph},
    inline::{parse_inline, text::parse_text},
};
use combine::{many, parser::char::char, ParseError, Parser, RangeStream};

pub fn parse_paragraph<'a, Input>() -> impl Parser<Input, Output = Paragraph>
where
    Input: RangeStream<Token = char, Range = &'a str>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many(parse_inline()).map(|x| Paragraph(x))
}

#[cfg(test)]
mod tests {
    use crate::ast::ast::{Anchor, Bold, Inline, Italic, Paragraph, Text};

    use super::parse_paragraph;
    use combine::Parser;

    #[test]
    fn it_works_inlines_with_single() {
        let input = "*italic*";
        let mut parser = parse_paragraph();
        let res = parser.parse(input);
        let italic = Italic("italic".to_string());
        let inline_italic = Inline::Italic(italic);
        let inlines = vec![inline_italic];
        let paragraph = Paragraph(inlines);
        assert_eq!(res.unwrap().0, paragraph);
    }

    #[test]
    fn it_works_inlines() {
        let input = "aaa [hoge](http://localhost:3000) *italic* is not **bold**";
        let mut parser = parse_paragraph();
        let res = parser.parse(input);
        let text = Text("aaa ".to_string());
        let inline_text = Inline::Text(text);
        let anchor = Anchor {
            title: "hoge".to_string(),
            link: "http://localhost:3000".to_string(),
        };
        let italic = Italic("italic".to_string());
        let inline_italic = Inline::Italic(italic);
        let inline_link = Inline::Anchor(anchor);
        let text2 = Text("is not ".to_string());
        let inline_text2 = Inline::Text(text2);
        let bold = Bold("bold".to_string());
        let inline_bold = Inline::Bold(bold);
        let inlines = vec![
            inline_text,
            inline_link,
            inline_italic,
            inline_text2,
            inline_bold,
        ];
        let paragraph = Paragraph(inlines);
        assert_eq!(res.unwrap().0, paragraph);
    }

    #[test]
    fn it_works_inlines_without_text() {
        let input = "[hoge](http://localhost:3000) *italic*";
        let mut parser = parse_paragraph();
        let res = parser.parse(input);
        let anchor = Anchor {
            title: "hoge".to_string(),
            link: "http://localhost:3000".to_string(),
        };
        let italic = Italic("italic".to_string());
        let inline_italic = Inline::Italic(italic);
        let inline_link = Inline::Anchor(anchor);
        let inlines = vec![inline_link, inline_italic];
        let paragraph = Paragraph(inlines);
        assert_eq!(res.unwrap().0, paragraph);
    }
}
