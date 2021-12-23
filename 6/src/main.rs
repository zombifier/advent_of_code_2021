use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Fish {
    timer: u64,
}

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
    let mut fishies: Vec<Fish> = read_lines("./input")?
        .next()
        .ok_or("No line")?? // Get the singular line from input
        .split(",")
        .map(|value| Fish {
            timer: value.parse::<u64>().unwrap_or(0),
        })
        .collect();
    for _ in 0..80 {
        let mut new_fishies = 0;
        fishies.iter_mut().for_each(|fish| {
            if fish.timer == 0 {
                fish.timer = 6;
                new_fishies += 1;
            } else {
                fish.timer -= 1;
            }
        });
        fishies.resize_with(fishies.len() + new_fishies, || Fish { timer: 8 });
    }
    println!("{}", fishies.len());
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
