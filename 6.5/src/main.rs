use std::collections::VecDeque;
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
    const VETERAN_DAYS: usize = 7;
    const NEWBIE_DAYS: usize = 9;
    let mut fishies_queue: VecDeque<u64> = VecDeque::new();
    fishies_queue.resize(VETERAN_DAYS, 0);
    let mut new_fishies_queue: VecDeque<u64> = VecDeque::new();
    new_fishies_queue.resize(NEWBIE_DAYS - VETERAN_DAYS, 0);
    read_lines("./input")?
        .next()
        .ok_or("No line")?? // Get the singular line from input
        .split(",")
        .map(|value| value.parse::<usize>().unwrap_or(0))
        .for_each(|value| fishies_queue[value] += 1);
    for _ in 0..256 {
        let new_fishies = fishies_queue.pop_front().ok_or("FATAL")?;
        new_fishies_queue.push_back(new_fishies);
        let vet_fishies = new_fishies_queue.pop_front().ok_or("FATAL")?;
        fishies_queue.push_back(new_fishies + vet_fishies); // new_fishies in this case means fish that just gave birth
    }
    println!(
        "{}",
        fishies_queue.iter().sum::<u64>() + new_fishies_queue.iter().sum::<u64>()
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
