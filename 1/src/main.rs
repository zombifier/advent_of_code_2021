use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut first = true;
    let mut current_num = 0;
    let mut increase_count = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(Ok(num)) = line.map(|value| value.parse::<i32>()) {
                if first {
                    first = false;
                } else if num > current_num {
                    increase_count += 1;
                }
                current_num = num;
            }
        }
    }
    println!("{}", increase_count);
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
