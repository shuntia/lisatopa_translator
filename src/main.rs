use std::io::{BufRead, stdin};

use log::{info, warn};

use crate::translator::translate_lines;

pub mod parser;
pub mod translator;

fn main() {
    env_logger::init();
    let mut target = String::new();
    stdin().lock().read_line(&mut target).unwrap();
    let res = parser::parse(&target);
    let (residual, lines) = res.unwrap();
    if !residual.is_empty() {
        warn!("Failed to parse fully. Residue: {residual}");
    }
    info!("{:?}", &lines);
    let result = translate_lines(lines);
    println!("{}", result);
}
