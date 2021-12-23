use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut line_count = 0;
    let mut one_bit_counts: Vec<i32> = Vec::new();
    let mut first = true;
    let mut gamma = 0;
    let mut epsilon = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(line_vec) = line.map(|value| value.as_bytes().to_owned()) {
                line_count += 1;
                if first {
                    one_bit_counts.resize(line_vec.len(), 0);
                    first = false;
                }
                for (i, x) in line_vec.iter().enumerate() {
                    if *x == '1' as u8 {
                        one_bit_counts[i] += 1;
                    }
                }
            }
        }
    }
    for (i, x) in one_bit_counts.iter().enumerate() {
        if *x > line_count / 2 {
            gamma += 1 << (one_bit_counts.len() - i - 1);
        } else {
            epsilon += 1 << (one_bit_counts.len() - i - 1);
        }
    }
    println!("{} * {} = {}", gamma, epsilon, gamma * epsilon);
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
