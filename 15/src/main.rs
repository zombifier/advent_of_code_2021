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
        self.data[y][x]
    }

    fn get_next(&self, coord: &Coord) -> Vec<Coord> {
        let &(x, y) = coord;
        // Cast so we can subtract safely
        let x = x as i8;
        let y = y as i8;
        vec![(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]
            .into_iter()
            .filter(|(x, y)| {
                0 <= *x && *x < self.width() as i8 && 0 <= *y && *y < self.height() as i8
            })
            .map(|(x, y)| (x as usize, y as usize))
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
    let mut node_to_distance = HashMap::new();
    let mut visited_nodes = HashSet::new();
    node_to_distance.insert((0, 0), 0);
    let mut current_node = (0, 0);
    loop {
        visited_nodes.insert(current_node);
        let next_neighbors: Vec<Coord> = board.get_next(&current_node);
        next_neighbors.iter().for_each(|new_node| {
            let value = std::cmp::min(
                node_to_distance.get(&current_node).unwrap() + board.get(new_node),
                *node_to_distance.get(new_node).unwrap_or(&u32::MAX),
            );
            node_to_distance.insert(*new_node, value);
        });
        let mut sorted_node_to_distance = node_to_distance
            .iter()
            .filter(|(coord, _)| !visited_nodes.contains(coord))
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<(Coord, u32)>>();
        sorted_node_to_distance.sort_by(|val1, val2| val1.1.cmp(&val2.1));
        if sorted_node_to_distance.is_empty() {
            break;
        }
        current_node = sorted_node_to_distance[0].0;
    }
    println!(
        "{}",
        node_to_distance
            .get(&(board.width() - 1, board.height() - 1))
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
