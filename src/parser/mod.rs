use crate::ast::Expression;
use crate::parser::utils::{fenced, fenced_char, style, style_char};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{char};
use nom::combinator::{all_consuming, map};
use nom::sequence::{pair, preceded};
use nom::IResult;

mod utils;

// Generate the actual parsers here

fn internal_link(input: &str) -> IResult<&str, &str> {
    fenced("[[", "]]")(input)
}

fn markdown_link(input: &str) -> IResult<&str, (&str, &str)> {
    pair(fenced_char('[', ']'), fenced_char('(', ')'))(input)
}

fn triple_backtick(input: &str) -> IResult<&str, (&str, &str)> {
    let (remaining, inner) = fenced("```", "```")(input)?;

    let (contents, lang) = take_till(|c| c == '\n')(inner)?;
    // Remove \r if it exists
    Ok((
        remaining,
        (lang.trim_end_matches('\r'), contents.trim_end()),
    ))
}

fn single_backtick(input: &str) -> IResult<&str, &str> {
    fenced_char('`', '`')(input)
}

fn single_dollar(input: &str) -> IResult<&str, &str> {
    fenced_char('$', '$')(input)
}

fn bold(input: &str) -> IResult<&str, Vec<Expression>> {
    alt((style("**"), style("__")))(input)
}

fn italic(input: &str) -> IResult<&str, Vec<Expression>> {
    alt((style_char('*'), style_char('_')))(input)
}

fn strikethrough(input: &str) -> IResult<&str, Vec<Expression>> {
    style("~~")(input)
}

fn highlight(input: &str) -> IResult<&str, Vec<Expression>> {
    style("==")(input)
}

/// Parses `![alt](url)`
fn remote_image(input: &str) -> IResult<&str, (&str, &str)> {
    preceded(char('!'), markdown_link)(input)
}

/// Parses `![alt](url)`
fn image(input: &str) -> IResult<&str, &str> {
    preceded(char('!'), internal_link)(input)
}

fn directive(input: &str) -> IResult<&str, Expression> {
    alt((
        map(single_dollar, Expression::InlineMath),
        map(internal_link, Expression::InternalLink),
        map(remote_image, |(alt, url)| Expression::ExternalImage {
            alt,
            url,
        }),
        map(triple_backtick, |(lang, contents)| Expression::CodeBlock {
            lang,
            contents,
        }),
        map(single_backtick, Expression::InlineCode),
        map(image, Expression::InternalImage),
        map(bold, Expression::Bold),
        map(italic, Expression::Italic),
        map(strikethrough, Expression::StrikeThrough),
        map(highlight, Expression::Highlight),
    ))(input)
}

/// Parse a line of text, counting anything that doesn't match a directive as plain text.
fn parse_inline(input: &str) -> IResult<&str, Vec<Expression>> {
    let mut output = Vec::with_capacity(4);

    let mut current_input = input;

    while !current_input.is_empty() {
        let mut found_directive = false;
        for (current_index, _) in current_input.char_indices() {
            //  println!("{} {}", current_index, current_input);
            match directive(&current_input[current_index..]) {
                Ok((remaining, parsed)) => {
                    // println!("Matched {:?} remaining {}", parsed, remaining);
                    let leading_text = &current_input[0..current_index];
                    if !leading_text.is_empty() {
                        output.push(Expression::Text(leading_text));
                    }
                    output.push(parsed);

                    current_input = remaining;
                    found_directive = true;
                    break;
                }
                Err(nom::Err::Error(_)) => {
                    // None of the parsers matched at the current position, so this character is just part of the text.
                    // The iterator will go to the next character so there's nothing to do here.
                }
                Err(e) => {
                    // On any other error, just return the error.
                    return Err(e);
                }
            }
        }

        if !found_directive {
            output.push(Expression::Text(current_input));
            break;
        }
    }

    Ok(("", output))
}

pub fn parse(input: &str) -> Result<Vec<Expression>, nom::Err<nom::error::Error<&str>>> {
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
        all_consuming(parse_inline),
    ))(input)
    .map(|(_, results)| results)
}

#[cfg(test)]
mod tests {
    use crate::ast::Expression;
    use crate::parser::{parse};
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
}
