#[derive(Debug, PartialEq)]
pub struct Italic(pub String);
#[derive(Debug, PartialEq)]
pub struct Bold(pub String);

#[derive(Debug, PartialEq)]
pub struct Anchor {
    pub link: String,
    pub title: String,
}

#[derive(Debug, PartialEq)]
pub enum Inline {
    Anchor(Anchor),
    Bold(Bold),
    Italic(Italic),
}
#[derive(Debug, PartialEq, Eq)]
pub struct Heading<'a> {
    pub content: &'a str,
    pub level: u8,
}

#[derive(Debug, PartialEq)]
pub struct Paragraph<'a>(pub &'a str);

#[derive(Debug)]
pub enum LeafBlock<'a> {
    LeafBlock,
    Inline,
    Paragraph(Paragraph<'a>),
    Heading(&'a Heading<'a>),
}

enum ContainerBlock<'a> {
    BlockQuotes(&'a LeafBlock<'a>),
    ListItems,
}

#[derive(Debug)]
pub enum Ast<'a> {
    LeafBlock(&'a LeafBlock<'a>),
    ContainerBlock,
    Inline,
}
