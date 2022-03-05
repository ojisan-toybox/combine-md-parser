enum Inline {
    Anchor,
    Strong,
    Italic,
}

struct Paragraph {
    content: String,
}

struct Heading {
    content: String,
    level: u8,
}

enum LeafBlock {
    LeafBlock,
    Inline,
    Paragraph(Paragraph),
    Heading(Heading),
}

pub enum ContainerBlock {
    BlockQuotes(LeafBlock),
    ListItems,
}
