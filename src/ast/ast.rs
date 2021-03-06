#[derive(Debug, PartialEq)]
pub struct Italic(pub String);
#[derive(Debug, PartialEq)]
pub struct Bold(pub String);

#[derive(Debug, PartialEq)]
pub struct Text(pub String);

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
    Text(Text),
}
#[derive(Debug, PartialEq, Eq)]
pub struct Heading {
    pub content: String,
    pub level: u8,
}

#[derive(Debug, PartialEq)]
pub struct Paragraph(pub Vec<Inline>);

#[derive(Debug, PartialEq)]
pub enum LeafBlock {
    LeafBlock,
    Inline,
    Paragraph(Paragraph),
    Heading(Heading),
}

#[derive(Debug, PartialEq)]
pub enum Document {
    LeafBlocks(Vec<LeafBlock>),
    Inline(Inline),
}
