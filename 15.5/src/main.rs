use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
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
    fn width(&self) -> usize {
        self.data.get(0).map(|row| row.len()).unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.data.len()
    }

    fn get(&self, coord: &Coord) -> u32 {
        let &(x, y) = coord;
        let x_local = x % self.width();
        let y_local = y % self.height();
        let x_dup = x / self.width();
        let y_dup = y / self.height();
        // 9 wraps back to 1, not 0!
        (self.data[y_local][x_local] + x_dup as u32 + y_dup as u32 - 1) % 9 + 1
    }

    fn get_next<T>(&self, coord: &Coord, visited: &HashMap<Coord, T>) -> Vec<Coord> {
        let &(x, y) = coord;
        // Cast so we can subtract safely
        let x = x as i32;
        let y = y as i32;
        vec![(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]
            .into_iter()
            .filter(|(x, y)| {
                0 <= *x
                    && *x < (self.width() * 5) as i32
                    && 0 <= *y
                    && *y < (self.height() * 5) as i32
            })
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|coord| !visited.contains_key(coord))
            .collect()
    }
}

fn main() -> Result<(), MainError> {
    let board = Board {
        data: read_lines("./input")?
            .flatten()
            .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect(),
    };
    let mut unvisited_node_to_distance = HashMap::new();
    let mut visited_node_to_distance = HashMap::new();
    unvisited_node_to_distance.insert((0, 0), 0);
    let mut current_node = (0, 0);
    loop {
        visited_node_to_distance.insert(
            current_node,
            unvisited_node_to_distance.remove(&current_node).unwrap(),
        );
        let next_neighbors: Vec<Coord> = board.get_next(&current_node, &visited_node_to_distance);
        next_neighbors.iter().for_each(|new_node| {
            let value = std::cmp::min(
                visited_node_to_distance.get(&current_node).unwrap() + board.get(new_node),
                *unvisited_node_to_distance
                    .get(new_node)
                    .unwrap_or(&u32::MAX),
            );
            unvisited_node_to_distance.insert(*new_node, value);
        });
        if let Some(min_node) = unvisited_node_to_distance
            .iter()
            .max_by(|kv1, kv2| kv2.1.cmp(&kv1.1))
            .map(|(k, _)| *k)
        {
            current_node = min_node;
        } else {
            break;
        }
    }
    println!(
        "{}",
        visited_node_to_distance
            .get(&(board.width() * 5 - 1, board.height() * 5 - 1))
            .unwrap()
    );
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
