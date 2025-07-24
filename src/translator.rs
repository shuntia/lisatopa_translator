use log::{info, warn};

use crate::parser::Line;

pub fn translate_lines<'a>(lines: Vec<Line<'a>>) -> String {
    let mut accumulator = String::new();
    let mut line_iter = lines.iter().peekable();
    let mut temp = String::new();
    let mut no_sc = false;
    while let Some(line) = line_iter.next() {
        no_sc = false;
        accumulator.push_str(&" ".repeat(line.whitespaces as usize));
        let mut word_iter = line.parts.iter().peekable();
        while let Some(i) = word_iter.next() {
            accumulator.push_str(match *i {
                "pilika" => match word_iter.peek() {
                    Some(s) => match **s {
                        // pilika palusta-->while
                        "palusta" => {
                            word_iter.next();
                            "while"
                        }
                        // pilika late-->for
                        "late" => {
                            word_iter.next();
                            "for"
                        }
                        // expects pilika or late
                        _ => {
                            warn!("\"palusta\" vista \"late\" koppisane.");
                            **s
                        }
                    },
                    None => "",
                },
                "katte" => "==",
                "kate" => "=",
                "vas" => "in",
                "kalaki" => "pub",
                "sati" => "fn",
                x if ";,[{}01234567890.".contains(x) => {
                    no_sc = true;
                    x
                }
                x if x.chars().last().is_some_and(|s| s == 'e' || s == '!') => {
                    let fn_name = match x {
                        "takute!" => "println!",
                        unk => unk,
                    };
                    temp.push_str(fn_name);
                    temp.push('(');
                    while let Some(s) = word_iter.next() {
                        temp.push_str(s);
                        if word_iter.peek().is_some() {
                            temp.push(',');
                        }
                    }
                    temp.push(')');
                    &temp
                }
                unknown => {
                    warn!("{unknown} kate takuta pasta jakipe.");
                    unknown
                }
            });
            if !no_sc {
                accumulator.push(' ');
            }
        }
        if !no_sc {
            accumulator.push(';');
        }
        accumulator.push('\n');
    }
    accumulator
}
