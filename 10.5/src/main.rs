use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct MainError {
    _msg: &'static str,
}

impl From<std::io::Error> for MainError {
    fn from(_: std::io::Error) -> Self {
        MainError { _msg: "IoError" }
    }
}

impl From<&'static str> for MainError {
    fn from(msg: &'static str) -> Self {
        MainError { _msg: msg }
    }
}

impl From<std::num::ParseIntError> for MainError {
    fn from(_: std::num::ParseIntError) -> Self {
        MainError {
            _msg: "ParseIntError",
        }
    }
}

fn main() -> Result<(), MainError> {
    // ideally this is const but Rust can't do this without a 3rd party crate yet.
    let bracket_map = HashMap::from([
        (')', ('(', 3)),
        (']', ('[', 57)),
        ('}', ('{', 1197)),
        ('>', ('<', 25137)),
    ]);
    let autoscore_map = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let lines = read_lines("./input")?.flatten();
    let mut scores = Vec::new();
    'outer: for line in lines {
        let mut stack = Vec::new();
        for c in line.chars() {
            if bracket_map.contains_key(&c) {
                if !stack
                    .pop()
                    .map_or(false, |top| top == bracket_map.get(&c).unwrap().0)
                {
                    continue 'outer;
                }
            } else {
                stack.push(c);
            }
        }
        let mut fill_score: u64 = 0;
        while let Some(c) = stack.pop() {
            fill_score = fill_score * 5 + autoscore_map.get(&c).unwrap_or(&0);
        }
        scores.push(fill_score);
    }
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
    Ok(())
}
// The output`FromIterator<&Vec<u64>>` is not implemented for `[&Vec<u64>]`
// is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
