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

// Ideally we'd do this with a graph, but implementing a graph type in Rust is a pathway to
// many abilities some consider unnatural; as such I refrain from practicing such dark magicks
// for now.
type Route = Vec<String>;
type Connection = (String, String);

// Checks if the route is one that contains the cave, returns the other cave if true.
fn is_connection<'a, 'b>(node: &'a str, connection: &'b Connection) -> Option<&'b str> {
    if connection.0 == node {
        Some(&connection.1)
    } else if connection.1 == node {
        Some(&connection.0)
    } else {
        None
    }
}

// true if cave is small and visited, false otherwise
fn visited(node: &str, visited_smalls: &HashSet<String>) -> bool {
    node.to_lowercase() == node && visited_smalls.contains(node)
}

fn find_routes(
    current_node: &str,
    map: &Vec<Connection>,
    mut visited_smalls: HashSet<String>,
) -> Vec<Route> {
    if current_node.to_lowercase() == current_node {
        visited_smalls.insert(current_node.to_owned());
    }
    let mut result = Vec::new();
    if current_node == "end" {
        return vec![vec!["end".to_owned()]];
    }
    for next_node in map
        .iter()
        .filter_map(|c| is_connection(&current_node, c))
        .filter(|n| !visited(n, &visited_smalls))
        .collect::<Vec<&str>>()
    {
        let mut routes = find_routes(next_node, map, visited_smalls.clone());
        routes
            .iter_mut()
            .for_each(|route| route.insert(0, current_node.to_owned()));
        result.extend(routes);
    }
    result
}

fn main() -> Result<(), MainError> {
    let map: Vec<Connection> = read_lines("./input")?
        .flatten()
        .map(|line| line.split('-').map(|s| s.to_owned()).collect())
        .map(|mut list: Vec<String>| (list.swap_remove(0), list.swap_remove(0)))
        .collect();
    let result = find_routes("start", &map, HashSet::new());
    println!("{}", result.len());
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
