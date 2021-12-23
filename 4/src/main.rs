#![feature(try_trait)]

use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Square {
    value: u32,
    marked: bool,
}

struct Board {
    data: Vec<Vec<Square>>,
}

impl Board {
    fn new_row(&mut self, row: Vec<u32>) {
        self.data.push(
            row.into_iter()
                .map(|value| Square {
                    value,
                    marked: false,
                })
                .collect(),
        );
    }

    fn mark(&mut self, value: u32) {
        self.data
            .iter_mut()
            .flatten()
            .find(|square| square.value == value)
            .map(|square| square.marked = true);
    }

    fn bingo(&self) -> bool {
        // Check rows
        if self
            .data
            .iter()
            .any(|row| row.iter().all(|square| square.marked))
        {
            return true;
        }
        // Check columns
        for i in 0..self.data[0].len() {
            if self.data.iter().all(|row| row[i].marked) {
                return true;
            }
        }
        false
    }

    fn score(&self) -> u32 {
        self.data
            .iter()
            .flatten()
            .filter(|square| !square.marked)
            .map(|square| square.value)
            .sum()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data {
            for col in row {
                if col.marked {
                    write!(f, "!")?;
                }
                write!(f, "{} ", col.value)?;
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
impl From<std::option::NoneError> for MainError {
    fn from(_: std::option::NoneError) -> Self {
        MainError {}
    }
}

fn main() -> Result<(), MainError> {
    let mut lines = read_lines("./input")?;
    let mut boards = Vec::new();
    let moves: Vec<u32> = lines
        .next()??
        .split(",")
        .map(|value| value.parse().unwrap_or(0))
        .collect();
    for line in lines {
        // Safely unwrap line
        let line = line?;
        // Empty line, insert new board.
        if line.len() == 0 {
            boards.push(Board { data: Vec::new() });
        } else {
            let len = boards.len() - 1;
            let board = boards.get_mut(len)?;
            board.new_row(
                line.split(" ")
                    .filter(|substr| substr.len() > 0)
                    .map(|value| value.parse().unwrap_or(0))
                    .collect(),
            );
        }
    }
    for value in moves {
        for board in &mut boards {
            board.mark(value);
            if board.bingo() {
                println!("{}", board);
                println!("{}", board.score() * value);
                return Ok(());
            }
        }
    }
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
