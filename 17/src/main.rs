#[macro_use]
extern crate scan_fmt;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct MainError {
    _msg: String,
}

impl From<std::io::Error> for MainError {
    fn from(_: std::io::Error) -> Self {
        MainError {
            _msg: "IoError".to_owned(),
        }
    }
}

impl From<&'static str> for MainError {
    fn from(msg: &'static str) -> Self {
        MainError {
            _msg: msg.to_owned(),
        }
    }
}

impl From<String> for MainError {
    fn from(msg: String) -> Self {
        MainError { _msg: msg }
    }
}

impl From<std::num::ParseIntError> for MainError {
    fn from(_: std::num::ParseIntError) -> Self {
        MainError {
            _msg: "ParseIntError".to_owned(),
        }
    }
}

impl From<scan_fmt::parse::ScanError> for MainError {
    fn from(_: scan_fmt::parse::ScanError) -> Self {
        MainError {
            _msg: "ScanError".to_owned(),
        }
    }
}

struct ProbeConfig {
    x_range: (i32, i32),
    y_range: (i32, i32),
    x: i32,
    y: i32,
}

impl<'a> IntoIterator for &'a ProbeConfig {
    type Item = (i32, i32, bool, i32);
    type IntoIter = ProbeConfigIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ProbeConfigIterator {
            probe_config: self,
            x_velocity: self.x,
            y_velocity: self.y,
            x_position: 0,
            y_position: 0,
            ended: false,
            intersect: false,
            max_y: 0,
        }
    }
}

struct ProbeConfigIterator<'a> {
    probe_config: &'a ProbeConfig,
    x_velocity: i32,
    y_velocity: i32,
    x_position: i32,
    y_position: i32,
    ended: bool,
    intersect: bool,
    max_y: i32,
}

impl<'a> Iterator for ProbeConfigIterator<'a> {
    type Item = (i32, i32, bool, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            None
        } else {
            self.x_position += self.x_velocity;
            self.y_position += self.y_velocity;
            if self.y_position > self.max_y {
                self.max_y = self.y_position;
            }
            if self.x_velocity > 0 {
                self.x_velocity -= 1;
            } else if self.x_velocity < 0 {
                self.x_velocity += 1;
            }
            self.y_velocity -= 1;
            if self.x_position > self.probe_config.x_range.1 {
                self.ended = true;
            }
            if self.y_position < self.probe_config.y_range.0 {
                self.ended = true;
            }
            if self.probe_config.x_range.0 <= self.x_position
                && self.x_position <= self.probe_config.x_range.1
                && self.probe_config.y_range.0 <= self.y_position
                && self.y_position <= self.probe_config.y_range.1
            {
                self.ended = true;
                self.intersect = true;
            }
            Some((self.x_position, self.y_position, self.intersect, self.max_y))
        }
    }
}

fn main() -> Result<(), MainError> {
    let line = read_lines("./input")?.next().ok_or("Input Parse Error")??;
    let (x1, x2, y1, y2) = scan_fmt!(&line, "target area: x={}..{}, y={}..{}", i32, i32, i32, i32)?;
    let max_x_velocity = x2;
    let mut max_y_velocity = 2000;
    let mut max_height = 0;
    let mut count = 0;
    for x in 1..max_x_velocity + 1 {
        let mut y = -1000;
        while y < max_y_velocity {
            let config = ProbeConfig {
                x,
                y,
                x_range: (x1, x2),
                y_range: (y1, y2),
            };
            let config_iterator = config.into_iter();
            let (final_x, final_y, intersects, max_y) = config_iterator.last().unwrap();
            if intersects {
                if max_y > max_height {
                    max_height = max_y;
                }
                count += 1;
            } else {
                // Here lies my abandoned attempt at programatically finding a reasonable
                // max_y_velocity value. Now that I've found the solution by just brute forcing
                // everything I've dropped it harder than a Skrillex song. It's retained here for
                // posterity.
                /*
                if final_x > x2 {
                    max_y_velocity = config.y;
                }
                if final_x < x1 && final_y < y1 {
                    break;
                }
                */
                //            println!("({}, {}) does not intersect with max height {}", config.x, config.y, max_y);
            }
            y += 1;
        }
    }
    println!("{} {}", max_height, count);
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
