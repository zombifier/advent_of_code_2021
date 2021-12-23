use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
    let lines = read_lines("./input")?.flatten();
    let mut count_1478 = 0;
    for line in lines {
        let mut line_split = line.split("|");
        let _pattern = line_split.next().ok_or("ERROR")?;
        let output = line_split.next().ok_or("ERROR")?;
        count_1478 += output
            .split(" ")
            .filter(|digit| vec![2, 3, 4, 7].contains(&digit.len()))
            .count();
    }
    println!("{}", count_1478);
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
