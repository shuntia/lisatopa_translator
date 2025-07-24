use log::{info, warn};

use crate::parser::Line;

pub fn translate_lines<'a>(lines: Vec<Line<'a>>) -> String {
    let mut accumulator = String::new();
    let mut line_iter = lines.iter().peekable();
    while let Some(line) = line_iter.next() {
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
                unknown => {
                    warn!("{unknown} kate takuta pasta jakipe.");
                    unknown
                }
            });
            accumulator.push(' ');
        }
        accumulator.push('\n');
    }
    accumulator
}
