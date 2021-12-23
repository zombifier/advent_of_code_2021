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

fn map_add<T>(map: &mut HashMap<T, u64>, key: T, count: u64)
where
    T: std::hash::Hash + Eq + Copy,
{
    if !map.contains_key(&key) {
        map.insert(key, 0);
    }
    *map.get_mut(&key).unwrap() += count;
}

fn main() -> Result<(), MainError> {
    let mut lines = read_lines("./input")?.flatten();
    let polymer: Vec<char> = lines.next().ok_or("Parse Error")?.chars().collect();
    lines.next().ok_or("Parse Error")?;
    let rules: HashMap<(char, char), char> = lines
        .map(|line| {
            let (c1, c2, c3) = scan_fmt!(&line, "{/./}{/./} -> {/./}", char, char, char)
                .unwrap_or(('0', '0', '0'));
            ((c1, c2), c3)
        })
        .collect();
    let mut pair_count: HashMap<(char, char), u64> = HashMap::new();
    for slice in polymer.windows(2) {
        map_add(&mut pair_count, (slice[0], slice[1]), 1);
    }
    for _ in 0..40 {
        let mut new_pair_count: HashMap<(char, char), u64> = HashMap::new();
        for (&(c1, c2), val) in pair_count.iter() {
            let &c3 = rules.get(&(c1, c2)).unwrap();
            map_add(&mut new_pair_count, (c1, c3), *val);
            map_add(&mut new_pair_count, (c3, c2), *val);
        }
        pair_count = new_pair_count;
    }
    let mut char_count = HashMap::new();
    // Every char is the beginning of one pair except the last, so to count all instances of a
    // specific char in the polymer get the count of all pairs that begin with them.
    // Then we add the last char.
    for ((c1, _), count) in pair_count.iter() {
        map_add(&mut char_count, c1, *count);
    }
    map_add(&mut char_count, &polymer[polymer.len() - 1], 1);
    let mut char_count: Vec<u64> = char_count.iter().map(|(_, v)| *v).collect();
    char_count.sort();
    println!("{}", char_count[char_count.len() - 1] - char_count[0]);
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
