use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct MainError {
    _msg: &'static str,
}

/**
 * Visual representation of each segments by how many times they light up when
 * cycling through all 10 digits:
 *  8
 * 6 8
 *  7
 * 4 9
 *  7
 */
#[derive(PartialEq, Eq, Debug)]
enum Segment {
    Top,
    TopLeft,
    TopRight,
    Middle,
    BottomLeft,
    BottomRight,
    Bottom,
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

fn main() -> Result<(), MainError> {
    let lines = read_lines("./input")?.flatten();
    let mut sum = 0;
    for line in lines {
        // Map of digit to their segments.
        let mut digit_to_segment = HashMap::new();
        // Map of digit to their frequency within the 10-num cycle.
        let mut digit_to_freq = HashMap::new();
        // We use 1 and 4 to disambiguate the segments with the same frequency.
        let mut digit_in_one = HashSet::new();
        let mut digit_in_four = HashSet::new();
        let mut line_split = line.split("|");
        let patterns = line_split.next().ok_or("ERROR")?.split(" ");
        for pattern in patterns {
            let length = pattern.len();
            for c in pattern.chars() {
                if !digit_to_freq.contains_key(&c) {
                    digit_to_freq.insert(c, 0);
                }
                *digit_to_freq.get_mut(&c).unwrap() += 1;
                if length == 2 {
                    digit_in_one.insert(c);
                } else if length == 4 {
                    digit_in_four.insert(c);
                }
            }
        }
        // Now we assign digit to segments
        digit_to_freq.iter().for_each(|(digit, frequency)| {
            match frequency {
                4 => digit_to_segment.insert(*digit, Segment::BottomLeft),
                6 => digit_to_segment.insert(*digit, Segment::TopLeft),
                7 => {
                    if digit_in_four.contains(digit) {
                        digit_to_segment.insert(*digit, Segment::Middle)
                    } else {
                        digit_to_segment.insert(*digit, Segment::Bottom)
                    }
                }
                8 => {
                    if digit_in_one.contains(digit) {
                        digit_to_segment.insert(*digit, Segment::TopRight)
                    } else {
                        digit_to_segment.insert(*digit, Segment::Top)
                    }
                }
                9 => digit_to_segment.insert(*digit, Segment::BottomRight),
                _ => None,
            };
        });
        let outputs = line_split.next().ok_or("ERROR")?.split(" ");
        let mut char_arr = Vec::new();
        for output in outputs {
            if output.len() == 0 {
                continue;
            }
            match output.len() {
                2 => char_arr.push('1'),
                3 => char_arr.push('7'),
                4 => char_arr.push('4'),
                5 => {
                    if output
                        .chars()
                        .any(|digit| *digit_to_segment.get(&digit).unwrap() == Segment::TopLeft)
                    {
                        char_arr.push('5');
                    } else if output
                        .chars()
                        .any(|digit| *digit_to_segment.get(&digit).unwrap() == Segment::BottomLeft)
                    {
                        char_arr.push('2');
                    } else {
                        char_arr.push('3');
                    }
                }
                6 => {
                    if output
                        .chars()
                        .all(|digit| *digit_to_segment.get(&digit).unwrap() != Segment::Middle)
                    {
                        char_arr.push('0');
                    } else if output
                        .chars()
                        .any(|digit| *digit_to_segment.get(&digit).unwrap() == Segment::BottomLeft)
                    {
                        char_arr.push('6');
                    } else {
                        char_arr.push('9');
                    }
                }
                7 => char_arr.push('8'),
                _ => (),
            };
        }
        sum += char_arr.iter().collect::<String>().parse::<u32>()?;
    }
    println!("{}", sum);
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
