use nom::branch::alt;
use nom::bytes::complete::take_till;
use nom::character::complete::char;
use nom::combinator::map;
use nom::sequence::{pair, preceded};
use crate::ast::Expression;
use crate::parser::{NestedParseResult, ParseResult, StrParseResult};
use crate::parser::utils::{fenced, style};

fn internal_link(input: &str) -> StrParseResult {
    fenced("[[", "]]")(input)
}

fn markdown_link(input: &str) -> ParseResult<(&str, &str)> {
    pair(fenced("[", "]"), fenced("(", ")"))(input)
}

fn triple_backtick(input: &str) -> ParseResult<(&str, &str)> {
    let (remaining, inner) = fenced("```", "```")(input)?;

    let (contents, lang) = take_till(|c| c == '\n')(inner)?;
    // Remove \r if it exists
    Ok((
        remaining,
        (lang.trim_end_matches('\r'), contents.trim_end()),
    ))
}

fn single_backtick(input: &str) -> StrParseResult {
    fenced("`", "`")(input)
}

fn single_dollar(input: &str) -> StrParseResult {
    fenced("$", "$")(input)
}

fn bold(input: &str) -> NestedParseResult {
    alt((style("**"), style("__")))(input)
}

fn italic(input: &str) -> NestedParseResult {
    alt((style("*"), style("_")))(input)
}

fn strikethrough(input: &str) -> NestedParseResult {
    style("~~")(input)
}

fn highlight(input: &str) -> NestedParseResult {
    style("==")(input)
}

/// Parses `![alt](url)`
fn remote_image(input: &str) -> ParseResult<(&str, &str)> {
    preceded(char('!'), markdown_link)(input)
}

/// Parses `![alt](url)`
fn image(input: &str) -> StrParseResult {
    preceded(char('!'), internal_link)(input)
}

fn directive(input: &str) -> ParseResult<Expression> {
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
fn parse_inline(input: &str) -> NestedParseResult {
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
                        output.push(Expression::Text(leading_text.trim_start()));
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
            output.push(Expression::Text(current_input.trim_start()));
            break;
        }
    }

    Ok(("", output))
}