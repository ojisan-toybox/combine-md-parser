enum Inline {
    Anchor,
    Strong,
    Italic,
}
#[derive(Debug)]
pub struct Paragraph {
    content: String,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Heading<'a> {
    pub content: &'a str,
    pub level: u8,
}

#[derive(Debug)]
pub enum LeafBlock<'a> {
    LeafBlock,
    Inline,
    Paragraph(Paragraph),
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
