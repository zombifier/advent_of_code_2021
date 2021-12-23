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

struct Board {
    // Rows, then columns.
    data: Vec<Vec<u32>>,
}
impl Board {
    /*
    fn new_row(&mut self, row: Vec<u32>) {
        self.data.push(row.into_iter().map(|value| Square{value, marked: false}).collect());
    }
    */

    fn width(&self) -> usize {
        self.data.get(0).map(|row| row.len()).unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.data.len()
    }

    fn get(&self, x: usize, y: usize) -> u32 {
        self.data[y][x]
    }
}

fn main() -> Result<(), MainError> {
    let board = Board {
        data: read_lines("./input")?
            .flatten()
            .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect(),
    };
    let mut danger_val = 0;
    for y in 0..board.height() {
        for x in 0..board.width() {
            let val = board.get(x, y);
            if (y == 0 || val < board.get(x, y - 1))
                && (y == board.height() - 1 || val < board.get(x, y + 1))
                && (x == 0 || val < board.get(x - 1, y))
                && (x == board.width() - 1 || val < board.get(x + 1, y))
            {
                danger_val += val + 1;
            }
        }
    }
    println!("{}", danger_val);
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
