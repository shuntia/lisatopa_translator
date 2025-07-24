use log::info;
use nom::{
    Err, IResult, Parser,
    branch::alt,
    bytes::{
        complete::{take, take_till1, take_while1},
        take_till, take_while,
    },
    character::{char, one_of},
    combinator::verify,
    multi::{many0, many0_count},
    sequence::delimited,
};

pub fn parse<'a>(target: &'a str) -> IResult<&'a str, Vec<Line<'a>>> {
    let mut ret = Vec::new();
    for i in target.split('\n') {
        let trimmed = i.trim_start();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        ret.push((interpret).parse(i)?.1);
    }
    Ok(("", ret))
}

fn interpret<'a>(target: &'a str) -> IResult<&'a str, Line<'a>> {
    if target.contains("//") {
        return Ok((
            "",
            Line {
                whitespaces: 0,
                parts: Vec::new(),
            },
        ));
    }
    let (mut residual, whitespaces) = many0_count(char(' ')).parse(target)?;
    let mut contents = Vec::new();
    while !residual.trim_start().is_empty() {
        let (res, _) = many0(one_of(" \n")).parse(residual)?;
        if res.is_empty() {
            break;
        }
        let (res, content) = alt((parse_str, parse_keychars, parse_word)).parse(res)?;
        residual = res;
        contents.push(content);
    }
    Ok((
        "",
        Line {
            whitespaces: whitespaces as u32,
            parts: contents,
        },
    ))
}

fn parse_str<'a>(target: &'a str) -> IResult<&'a str, &'a str> {
    alt((
        delimited(char('\"'), take_till(|c| c == '\"'), char('\"')),
        delimited(char('\''), take_till(|c| c == '\''), char('\'')),
    ))
    .parse(target)
}

fn parse_keychars<'a>(target: &'a str) -> IResult<&'a str, &'a str> {
    verify(take(1usize), move |s: &str| {
        "(){}[]!+-*/&^%;<>=".contains(s)
    })
    .parse(target)
}

fn parse_word<'a>(target: &'a str) -> IResult<&'a str, &'a str> {
    take_till1(|el: char| !el.is_alphanumeric()).parse(target)
}

#[derive(Debug)]
pub struct Line<'a> {
    pub whitespaces: u32,
    pub parts: Vec<&'a str>,
}
