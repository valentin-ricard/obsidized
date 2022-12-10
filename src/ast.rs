pub struct Document<'a> {
    // TODO: Convert this to a serde struct
    pub frontmatter: String,
    pub contents: Vec<Expression<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression<'a> {
    Text(&'a str),
    RawHyperLink(&'a str),
    InternalLink(&'a str),
    InlineCode(&'a str),
    InlineMath(&'a str),
    BlockMath(&'a str),
    Heading {
        level: usize,
        text: &'a str,
    },
    CodeBlock {
        lang: &'a str,
        contents: &'a str,
    },
    TaskList(Vec<Expression<'a>>),
    Task {
        completed: bool,
        content: Vec<Expression<'a>>,
    },
    BlockQuote(Vec<Expression<'a>>),
    Callout {
        callout_type: &'a str,
        contents: Vec<Expression<'a>>,
    },
    ExternalImage {
        alt: &'a str,
        url: &'a str,
    },
    InternalImage(&'a str),
    Tables(Vec<Vec<Expression<'a>>>),
    Italic(Vec<Expression<'a>>),
    Bold(Vec<Expression<'a>>),
    StrikeThrough(Vec<Expression<'a>>),
    Highlight(Vec<Expression<'a>>),
    HorizontalBar,
    ListElement {
        style: &'a str,
        loose: bool,
        contents: Vec<Expression<'a>>,
    },
}
