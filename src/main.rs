use std::{
    fs::File,
    io::{BufRead, Read, stdin},
    path::PathBuf,
    str::FromStr,
};

use log::{info, warn};

use crate::translator::translate_lines;

pub mod parser;
pub mod translator;

fn main() {
    env_logger::init();
    let mut target = String::new();
    stdin().lock().read_line(&mut target).unwrap();
    target = target.trim().to_owned();
    let path = PathBuf::from_str(&target).unwrap();
    info!("{path:?}: exists? {}", path.exists());
    let mut file_handle = File::open(path).unwrap();
    let mut content = String::new();
    file_handle.read_to_string(&mut content);
    let res = parser::parse(&content);
    let (residual, lines) = res.unwrap();
    if !residual.is_empty() {
        warn!("Failed to parse fully. Residue: {residual}");
    }
    info!("{:?}", &lines);
    let result = translate_lines(lines);
    println!("{}", result);
}
