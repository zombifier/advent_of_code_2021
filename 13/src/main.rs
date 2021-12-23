#![feature(int_abs_diff)]
#[macro_use]
extern crate scan_fmt;

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

type Coord = (usize, usize);

fn fold_x(mut coord: Coord, x_fold: usize) -> Coord {
    if coord.0 > x_fold {
        coord.0 = x_fold - (coord.0 - x_fold);
    }
    coord
}

fn fold_y(mut coord: Coord, y_fold: usize) -> Coord {
    if coord.1 > y_fold {
        coord.1 = y_fold - (coord.1 - y_fold);
    }
    coord
}

#[derive(Debug)]
enum FoldInstruction {
    X(usize),
    Y(usize),
}

fn main() -> Result<(), MainError> {
    let mut lines = read_lines("./input")?.flatten();
    let mut dots: Vec<Coord> = lines
        .by_ref()
        .take_while(|line| line.len() > 0)
        .filter_map(|line| scan_fmt!(&line, "{d},{d}", usize, usize).ok())
        .collect();
    let instructions: Vec<FoldInstruction> = lines
        .filter_map(|line| {
            if let Ok(x) = scan_fmt!(&line, "fold along x={d}", usize) {
                Some(FoldInstruction::X(x))
            } else if let Ok(y) = scan_fmt!(&line, "fold along y={d}", usize) {
                Some(FoldInstruction::Y(y))
            } else {
                None
            }
        })
        .collect();
    for instruction in instructions {
        if let FoldInstruction::X(x) = instruction {
            dots = dots.into_iter().map(|coord| fold_x(coord, x)).collect();
        } else if let FoldInstruction::Y(y) = instruction {
            dots = dots.into_iter().map(|coord| fold_y(coord, y)).collect();
        }
        dots.sort();
        dots.dedup();
        println!("{}", dots.len());
        break; // Cease after first fold as problem states
    }
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
