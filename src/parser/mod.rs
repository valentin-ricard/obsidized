use std::mem::{swap, take};
use crate::ast::Expression;
use crate::parser::utils::{fenced, new_line, style};
use nom::branch::alt;
use nom::bytes::complete::{is_a, tag, take_till, take_until};
use nom::character::complete::{char, space0};
use nom::character::is_newline;
use nom::combinator::{all_consuming, eof, map, value};
use nom::sequence::{pair, preceded, tuple};
use nom::{ExtendInto, InputTakeAtPosition, IResult};
use nom::multi::{many1, many_till};
use crate::ast::Expression::Block;

mod utils;
mod directive;

type ParseResult<'a, T> = IResult<&'a str, T>;
type StrParseResult<'a> = ParseResult<'a, &'a str>;
type NestedParseResult<'a> = ParseResult<'a, Vec<Expression<'a>>>;

pub fn parse(
    input: &str,
) -> Result<Vec<Expression>, nom::Err<nom::error::Error<&str>>> {
    // We treat each line separately:
    let mut contents: Vec<Expression> = Vec::new();
    let mut current_block = Vec::with_capacity(4);

    for line in input.lines()  {
        // First, if the line is empty,
        if line.trim().is_empty() {
            // We need to create a block
            contents.push(Block(
                take(&mut current_block)
            ));
        } else {
            // We can simplify alt
            // We need to check if the line is a quote
            let (_, mut expressions)
                = all_consuming(parse_inline)(line)?;
            current_block.append(&mut expressions);
        }
    }

    // Push the last block
    contents.push(Block(
        take(&mut current_block)
    ));
    Ok(contents)
}

pub fn parse_block(
    input: &str,
) -> Result<Vec<Expression>, nom::Err<nom::error::Error<&str>>> {
    alt((
        map(all_consuming(tag("---")), |_| {
            vec![Expression::HorizontalBar]
        }),
        /*
        TODO: Add support for callouts in addition to block quotes using a custom parser.
        map(all_consuming(preceded(tag("> "), parse_inline)), |values| {
            vec![Expression::BlockQuote(values)]
        }),
        map(all_consuming(attribute), |(name, value)| {
            vec![Expression::Attribute { name, value }]
        }),
         */
        // Split it in
        all_consuming(parse_inline),
    ))(input)
        .map(|(_, results)| results)
}

#[cfg(test)]
mod tests {
    use crate::ast::Expression;
    use crate::parser::parse;
    use stringify;

    macro_rules! test_specific {
        ($name:ident, $input:literal, $result: expr) => {
            let expression_result = parse($input).expect(&*format!(
                "An error occurred while parsing {}",
                stringify!($name)
            ));

            assert_eq!(expression_result, $result)
        };
    }

    #[test]
    fn test_basic() {
        test_specific!(
            basic,
            "This is a test",
            vec![Expression::Text("This is a test")]
        );

        test_specific!(
            bold_star,
            "**Bold Testing**",
            vec![Expression::Bold(vec![Expression::Text("Bold Testing")])]
        );

        test_specific!(
            bold_underscore,
            "__Bold Testing__",
            vec![Expression::Bold(vec![Expression::Text("Bold Testing")])]
        );

        test_specific!(
            italic_star,
            "*Italic Testing*",
            vec![Expression::Italic(vec![Expression::Text("Italic Testing")])]
        );

        test_specific!(
            italic_underscore,
            "_Italic Testing_",
            vec![Expression::Italic(vec![Expression::Text("Italic Testing")])]
        );

        test_specific!(
            strikethrough,
            "~~Not like this~~",
            vec![Expression::StrikeThrough(vec![Expression::Text(
                "Not like this"
            )])]
        );

        test_specific!(
            highlight,
            "==REACTION==",
            vec![Expression::Highlight(vec![Expression::Text("REACTION")])]
        );

        test_specific!(
            inline_code,
            "`inline_code`",
            vec![Expression::InlineCode("inline_code")]
        );

        test_specific!(
            inline_math,
            "$b^2-4ac$",
            vec![Expression::InlineMath("b^2-4ac")]
        );
    }

    #[test]
    fn test_codeblock() {
        test_specific!(
            no_lang,
            r#"```
        fn test() -> bool {}
        ```"#,
            vec![Expression::CodeBlock {
                lang: "",
                contents: "\n        fn test() -> bool {}"
            }]
        );
    }

    #[test]
    fn test_blocks() {
        test_specific!(
            hbar,
            r#"This is a test
            ---
            This is another test"#,
            vec![
                Expression::Text("This is a test"),
                Expression::HorizontalBar,
                Expression::Text("This is another test")
            ]
        );

        test_specific!(
            line_break,
            r#"This is a test

            This is another test"#,
            vec![
                Expression::Text("This is a test"),
                Expression::LineBreak,
                Expression::Text("This is another test")
            ]
        );
    }

    #[test]
    fn lifetime_test() {
        let contents = "This is a test".to_string();

        parse(&*contents).unwrap();
    }
}
