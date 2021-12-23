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
    flashed: Vec<Vec<bool>>,
}

type Coord = (usize, usize);

impl Board {
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

    fn get_flashed(&self, coord: Coord) -> bool {
        let (x, y) = coord;
        self.flashed[y][x]
    }

    fn get_mut(&mut self, coord: Coord) -> &mut u32 {
        let (x, y) = coord;
        self.data.get_mut(y).unwrap().get_mut(x).unwrap()
    }

    fn set_flashed(&mut self, coord: Coord) {
        let (x, y) = coord;
        self.flashed[y][x] = true;
    }

    fn get_adjacent(&self, coord: Coord) -> Vec<Coord> {
        let (x, y) = coord;
        // To allow for negative without panicking
        let x = x as i8;
        let y = y as i8;
        vec![
            (x - 1, y),
            (x + 1, y),
            (x, y - 1),
            (x, y + 1),
            (x - 1, y - 1),
            (x - 1, y + 1),
            (x + 1, y - 1),
            (x + 1, y + 1),
        ]
        .iter()
        .filter(|(x, y)| 0 <= *x && *x < self.width() as i8 && 0 <= *y && *y < self.height() as i8)
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect()
    }

    fn energize(&mut self, coord: Coord) -> u32 {
        if self.get_flashed(coord) {
            return 0;
        }
        let mut flash_count = 0;
        let val = self.get_mut(coord);
        if *val != 9 {
            *val += 1;
        } else {
            *val = 0; // Flash!
            flash_count += 1;
            self.set_flashed(coord);
            flash_count += self
                .get_adjacent(coord)
                .iter()
                .map(|c| self.energize(*c))
                .sum::<u32>();
        }
        flash_count
    }

    /**
     * Steps the board by one cycle, returning the number of octopi who flashed.
     */
    fn step(&mut self) -> u32 {
        // Reset
        self.flashed = vec![vec![false; self.data[0].len()]; self.data.len()];
        let mut flash_count = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                flash_count += self.energize((x, y));
            }
        }
        flash_count
    }
}

fn main() -> Result<(), MainError> {
    let data: Vec<Vec<u32>> = read_lines("./input")?
        .flatten()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();
    let mut board = Board {
        flashed: vec![vec![false; data[0].len()]; data.len()],
        data,
    };
    let mut i = 0;
    loop {
        i += 1;
        let count = board.step();
        if count == (board.width() * board.height()) as u32 {
            println!("{}", i);
            break;
        }
        //println!("{:?}", board.data);
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
