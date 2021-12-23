#![feature(int_abs_diff)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Fish {
    timer: u64,
}

#[derive(Debug, Clone)]
struct MainError;

impl From<std::io::Error> for MainError {
    fn from(_: std::io::Error) -> Self {
        MainError {}
    }
}

impl From<&str> for MainError {
    fn from(_: &str) -> Self {
        MainError {}
    }
}

fn main() -> Result<(), MainError> {
    let line = read_lines("./input")?.next().ok_or("No line")??; // Get the singular line from input
    let mut values: Vec<u32> = line
        .split(",")
        .map(|value| value.parse::<u32>().unwrap_or(0))
        .collect();
    let mut min_fuel = u32::MAX;
    for position in 0..*values.iter().max().ok_or("ERROR")? {
        let fuel = values
            .iter()
            .map(|value| {
                let diff = value.abs_diff(position);
                diff * (diff + 1) / 2
            })
            .sum();
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    println!("{}", min_fuel);
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
