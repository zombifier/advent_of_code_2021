use std::collections::HashSet;
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

type Coord = (usize, usize);

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

    fn get(&self, coord: Coord) -> u32 {
        let (x, y) = coord;
        self.data[y][x]
    }

    // Returns all coords connected to this coord in a basin. Ignore all elements in the ignore list.
    fn basin_elements(&self, position: Coord, ignore: &mut HashSet<Coord>) -> HashSet<Coord> {
        let mut ret = HashSet::new();
        ret.insert(position);
        ignore.insert(position);
        let (x, y) = position;
        let mut coords_to_check = Vec::new();
        if x > 0 && !ignore.contains(&(x - 1, y)) {
            coords_to_check.push((x - 1, y));
        }
        if y > 0 && !ignore.contains(&(x, y - 1)) {
            coords_to_check.push((x, y - 1));
        }
        if x < self.width() - 1 && !ignore.contains(&(x + 1, y)) {
            coords_to_check.push((x + 1, y));
        }
        if y < self.height() - 1 && !ignore.contains(&(x, y + 1)) {
            coords_to_check.push((x, y + 1));
        }
        for coord in coords_to_check {
            if self.get(position) < self.get(coord) && self.get(coord) < 9 {
                ret.extend(self.basin_elements(coord, ignore));
            }
        }
        ret
    }
}

fn main() -> Result<(), MainError> {
    let board = Board {
        data: read_lines("./input")?
            .flatten()
            .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect(),
    };
    let mut basins = Vec::new();
    for y in 0..board.height() {
        for x in 0..board.width() {
            let val = board.get((x, y));
            if (y == 0 || val < board.get((x, y - 1)))
                && (y == board.height() - 1 || val < board.get((x, y + 1)))
                && (x == 0 || val < board.get((x - 1, y)))
                && (x == board.width() - 1 || val < board.get((x + 1, y)))
            {
                // Found basin bottom!
                basins.push(board.basin_elements((x, y), &mut HashSet::new()));
            }
        }
    }
    basins.sort_by(|basin1, basin2| basin2.len().cmp(&basin1.len()));
    let mut result = 1;
    for i in 0..3 {
        result *= basins[i].len();
    }
    println!("{}", result);
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
