use crate::ast::{Document, Expression};

use std::io::{Error, Write};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Error while writing contents to file")]
    IoError(#[from] Error),
}

pub fn convert<T: Write>(document: &Document, output: &mut T) -> Result<(), ConversionError> {
    // As we are at top level, we have to split it in blocks
    for content in &document.contents {
        writeln!(output, "<p class=\"block\">")?;
        convert_expression(content, output)?;
        writeln!(output, "\n</p>")?;
    }
    Ok(())
}

fn convert_inner<T: Write>(
    elements: &Vec<Expression>,
    output: &mut T,
) -> Result<(), ConversionError> {
    for element in elements {
        convert_expression(&element, output)?;
    }

    Ok(())
}

fn convert_expression<T: Write>(expr: &Expression, output: &mut T) -> Result<(), ConversionError> {
    match expr {
        Expression::Text(text) => {
            output.write(text.as_bytes())?;
        }
        Expression::RawHyperLink(link) => {
            write!(output, "<a class=\"link\" href\"{0}\">{0}</a>", link)?;
        }
        Expression::InternalLink(link) => {
            // TODO: Resolve link + get name for display instead of path
            write!(
                output,
                "<a class=\"link internal-link\" href=\"{0}\"> {0}",
                link
            )?;
        }
        Expression::InlineCode(code) => {
            write!(output, "<span class=\"inline-code\">{}</span>", code)?;
        }
        Expression::InlineMath(math) => {
            // TODO: This is not supported yet, this will be when we will use
            // https://docs.rs/katex/latest/katex/fn.render.html
            write!(output, "${0}$", math)?;
        }
        Expression::BlockMath(math) => {
            // TODO: This is not supported yet, this will be when we will use
            // https://docs.rs/katex/latest/katex/fn.render.html
            write!(output, "$${0}$$", math)?;
        }
        Expression::Heading { level, text } => {
            let tag = match level {
                1 => "h1",
                2 => "h2",
                3 => "h3",
                4 => "h4",
                _ => "div",
            };

            write!(
                output,
                "<{tag} class=\"heading header-{level}\">{text}</div>"
            )?;
        }
        Expression::CodeBlock { lang: _, contents: _ } => {
            // TODO: Using https://docs.rs/tree-sitter-highlight/latest/tree_sitter_highlight/struct.HtmlRenderer.html
            //  implement syntax highlighting.
        }
        Expression::TaskList(_elements) => {
            // TODO: Support this.
        }
        Expression::Task { completed: _, content: _ } => {
            // TODO: Support this.
        }
        Expression::BlockQuote(contents) => {
            writeln!(output, "<quote class=\"blockquote\">")?;
            convert_inner(contents, output)?;
            writeln!(output, "</quote>")?;
        }
        Expression::Callout { .. } => {
            // TODO: Implement callouts.
        }
        Expression::ExternalImage { alt, url } => {
            // TODO: Add size attributes parsed from the text
            write!(
                output,
                "<img class=\"external-image\" alt=\"{alt}\" src=\"{url}\"/>"
            )?;
        }
        Expression::InternalImage(link) => {
            // TODO: Resolve link + get name for display instead of path, same for alt
            // TODO: Add size attributes parsed from the text
            write!(
                output,
                "<img class=\"external-image\" alt=\"{link}\" src=\"{link}\"/>"
            )?;
        }
        Expression::Tables(_) => {
            // TODO: Support this.
        }
        Expression::Italic(contents) => {
            writeln!(output, "<span class=\"italic\">")?;
            convert_inner(contents, output)?;
            writeln!(output, "</span>")?;
        }
        Expression::Bold(contents) => {
            writeln!(output, "<span class=\"bold\">")?;
            convert_inner(contents, output)?;
            writeln!(output, "</span>")?;
        }
        Expression::StrikeThrough(contents) => {
            writeln!(output, "<span class=\"strikethrough\">")?;
            convert_inner(contents, output)?;
            writeln!(output, "</span>")?;
        }
        Expression::Highlight(contents) => {
            writeln!(output, "<span class=\"highlight\">")?;
            convert_inner(contents, output)?;
            writeln!(output, "</span>")?;
        }
        Expression::HorizontalBar => {
            writeln!(output, "<div class=\"horizontal-bar\"> </div>")?;
        }
        Expression::ListElement { .. } => {
            // TODO: Implement this.
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::ast::{Document, Expression};
    use crate::converter::convert;
    

    fn compile_ast(contents: Vec<Expression>) -> String {
        let document = Document {
            frontmatter: "".to_string(),
            contents,
        };
        let mut buf = Vec::new();
        let _result = convert(&document, &mut buf).expect("An error occurred");

        let output = std::str::from_utf8(buf.as_slice()).unwrap().to_string();
        output
    }

    #[test]
    pub fn test_converter() {
        let contents = vec![Expression::Text("This is a test!")];

        assert_eq!(
            compile_ast(contents),
            r##"<p class="block">
This is a test!
</p>
"##
        )
    }
}
