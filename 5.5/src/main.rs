#![feature(int_abs_diff)]
#![feature(iter_zip)]
#[macro_use]
extern crate scan_fmt;

use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Square {
    count: u32,
}

struct Board {
    // Rows, then columns.
    data: Vec<Vec<Square>>,
}

impl Board {
    // Get an element. Automatically allocate extra space if needed.
    fn get(&mut self, x: usize, y: usize) -> &mut Square {
        if y >= self.data.len() {
            self.data.resize_with(y + 1, || Vec::new());
        }
        let row = &mut self.data[y];
        if x >= row.len() {
            row.resize_with(x + 1, || Square { count: 0 });
        }
        &mut row[x]
    }

    fn add_line(&mut self, start: (usize, usize), end: (usize, usize)) {
        if start.0 == end.0 {
            // Vertical line
            let iter = if start.1 < end.1 {
                start.1..end.1 + 1
            } else {
                end.1..start.1 + 1
            };
            iter.for_each(|row| self.get(start.0, row).count += 1);
        } else if start.1 == end.1 {
            // Horizontal line
            let iter = if start.0 < end.0 {
                start.0..end.0 + 1
            } else {
                end.0..start.0 + 1
            };
            iter.for_each(|col| self.get(col, start.1).count += 1);
        } else if start.0.abs_diff(end.0) == start.1.abs_diff(end.1) {
            // Diagonal line
            // Box'ing is necessary because Range and Rev are different types.
            let row_iter: Box<dyn std::iter::Iterator<Item = usize>> = if start.1 < end.1 {
                Box::new(start.1..end.1 + 1)
            } else {
                Box::new((end.1..start.1 + 1).rev())
            };
            let col_iter: Box<dyn std::iter::Iterator<Item = usize>> = if start.0 < end.0 {
                Box::new(start.0..end.0 + 1)
            } else {
                Box::new((end.0..start.0 + 1).rev())
            };
            std::iter::zip(col_iter, row_iter).for_each(|(col, row)| self.get(col, row).count += 1);
        }
    }

    fn count(&self) -> u32 {
        self.data
            .iter()
            .flatten()
            .filter(|square| square.count > 1)
            .count() as u32
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data {
            for col in row {
                write!(f, "{} ", col.count)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct MainError;

impl From<std::io::Error> for MainError {
    fn from(_: std::io::Error) -> Self {
        MainError {}
    }
}
impl From<scan_fmt::parse::ScanError> for MainError {
    fn from(_: scan_fmt::parse::ScanError) -> Self {
        MainError {}
    }
}

fn main() -> Result<(), MainError> {
    let lines = read_lines("./input")?;
    let mut board = Board { data: Vec::new() };

    for line in lines {
        let line = line?;
        let (x1, y1, x2, y2) = scan_fmt!(&line, "{},{} -> {},{}", usize, usize, usize, usize)?;
        board.add_line((x1, y1), (x2, y2));
    }
    println!("{}", board.count());
    Ok(())
}

// The output`FromIterator<&Vec<u32>>` is not implemented for `[&Vec<u32>]`
// is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
