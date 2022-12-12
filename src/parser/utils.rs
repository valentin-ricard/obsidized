use nom::branch::alt;
use crate::ast::Expression;
use crate::parser::parse_inline;
use nom::bytes::complete::{tag, take_till, take_until, take_while1};
use nom::character::complete::char;
use nom::character::{is_newline, is_space};
use nom::combinator::{map, map_parser};
use nom::sequence::tuple;
use nom::IResult;

/// Checks if the provided character is not a whitespace or a new line.
pub fn non_whitespace(c: char) -> bool {
    !is_space(c as u8) && !is_newline(c as u8)
}

/// Parses a single word (characters delimited with spaces or newlines)
pub fn word(input: &str) -> IResult<&str, &str> {
    take_while1(non_whitespace)(input)
}

pub fn new_line<'a>(input: &'a str) -> IResult<&'a str, &'a str>{
    alt((tag("\r\n"), tag("\n")))(input)
}




pub fn fenced<'a, 'b: 'a>(
    start: &'b str,
    end: &'b str,
) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    map(tuple((tag(start), take_until(end), tag(end))), |x| x.1)
}

pub fn fenced_char<'a>(start: char, end: char) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    map(
        tuple((char(start), take_till(move |c| c == end), char(end))),
        |x| x.1,
    )
}

pub fn style<'a, 'b: 'a>(
    boundary: &'b str,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Expression<'a>>> {
    map_parser(fenced(boundary, boundary), parse_inline)
}

pub fn style_char<'a>(
    boundary: char,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Expression<'a>>> {
    map_parser(fenced_char(boundary, boundary), parse_inline)
}
