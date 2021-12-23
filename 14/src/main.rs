#[macro_use]
extern crate scan_fmt;

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

impl From<scan_fmt::parse::ScanError> for MainError {
    fn from(_: scan_fmt::parse::ScanError) -> Self {
        MainError { _msg: "ScanError" }
    }
}

fn main() -> Result<(), MainError> {
    let mut lines = read_lines("./input")?.flatten();
    let mut polymer: Vec<char> = lines.next().ok_or("Parse Error")?.chars().collect();
    lines.next().ok_or("Parse Error")?;
    let rules: HashMap<(char, char), char> = lines
        .map(|line| {
            let (c1, c2, c3) = scan_fmt!(&line, "{/./}{/./} -> {/./}", char, char, char)
                .unwrap_or(('0', '0', '0'));
            ((c1, c2), c3)
        })
        .collect();
    for _ in 0..10 {
        let new_elements: Vec<char> = polymer
            .iter()
            .enumerate()
            .map(|(i, _)| {
                if i == polymer.len() - 1 {
                    '0'
                } else {
                    *rules.get(&(polymer[i], polymer[i + 1])).unwrap()
                }
            })
            .collect();
        // When we zip two arrays, if one is longer than the other the extra elements are
        // discarded. That's why we pad new_elements with one extra char (0) above and pop it.
        polymer = polymer
            .into_iter()
            .zip(new_elements.into_iter())
            .flat_map(|(c1, c2)| vec![c1, c2])
            .collect();
        polymer.pop();
    }
    let mut count = HashMap::new();
    for c in polymer.iter() {
        if !count.contains_key(c) {
            count.insert(c, 0);
        }
        *count.get_mut(c).unwrap() += 1;
    }
    let mut count: Vec<u32> = count.iter().map(|(_, v)| *v).collect();
    count.sort();
    println!("{}", count[count.len() - 1] - count[0]);
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
