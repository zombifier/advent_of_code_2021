use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Command {
    FORWARD,
    UP,
    DOWN,
}

fn main() {
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(Some((command, num))) = line.map(|value| parse_str(&value)) {
                match command {
                    Command::FORWARD => {
                        distance += num;
                        depth += aim * num;
                    }
                    Command::UP => {
                        aim -= num;
                    }
                    Command::DOWN => {
                        aim += num;
                    }
                }
            }
        }
    }
    println!("{}", depth * distance);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_str(str: &str) -> Option<(Command, i32)> {
    let split: Vec<&str> = str.split(" ").collect();
    if let (Some(command), Some(num)) = (
        split.get(0).and_then(|value| map_command(value)),
        split.get(1).and_then(|value| value.parse::<i32>().ok()),
    ) {
        return Some((command, num));
    }
    return None;
}

fn map_command(cmd_str: &str) -> Option<Command> {
    match cmd_str {
        "forward" => Some(Command::FORWARD),
        "up" => Some(Command::UP),
        "down" => Some(Command::DOWN),
        _ => None,
    }
}
