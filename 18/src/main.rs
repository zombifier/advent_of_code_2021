use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::Chars;

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

#[derive(Debug)]
struct SnailfishNumber {
    left: SnailfishNumberValue,
    right: SnailfishNumberValue,
}

#[derive(Debug)]
enum SnailfishNumberValue {
    Single(u8),
    Pair(Box<SnailfishNumber>),
}

impl SnailfishNumber {
    // Check if the number is a simple pair (e.g. [1, 2]), return both values if so
    fn is_single_pair(&self) -> Option<(u8, u8)> {
        if let (SnailfishNumberValue::Single(x), SnailfishNumberValue::Single(y)) =
            (&self.left, &self.right)
        {
            return Some((*x, *y));
        }
        return None;
    }

    // Finds and explodes the leftmost pair. Returns value (since a pair can collapse into a single
    // value), a value to add to the first number in the left, a value to add to the first number in
    // the right, and whether there has been an explosion.
    fn explode(mut self, layer_count: u8) -> (SnailfishNumberValue, Option<u8>, Option<u8>, bool) {
        if let Some((x, y)) = self.is_single_pair() {
            if layer_count >= 4 {
                return (SnailfishNumberValue::Single(0), Some(x), Some(y), true);
            }
        }
        let (left_return, exploded) = if let SnailfishNumberValue::Pair(pair) = self.left {
            let (new_left, some_x_propagate, some_y_add, exploded) = pair.explode(layer_count + 1);
            self.left = new_left;
            if let Some(y) = some_y_add {
                self.right = self.right.add_to_leftmost(y);
            }
            (some_x_propagate, exploded)
        } else {
            (None, false)
        };
        if exploded {
            return (
                SnailfishNumberValue::Pair(Box::new(self)),
                left_return,
                None,
                true,
            );
        }
        let (right_return, exploded) = if let SnailfishNumberValue::Pair(pair) = self.right {
            let (new_right, some_x_add, some_y_propagate, exploded) = pair.explode(layer_count + 1);
            self.right = new_right;
            if let Some(x) = some_x_add {
                self.left = self.left.add_to_rightmost(x);
            }
            (some_y_propagate, exploded)
        } else {
            (None, false)
        };
        (
            SnailfishNumberValue::Pair(Box::new(self)),
            left_return,
            right_return,
            exploded,
        )
    }

    fn add_to_leftmost(mut self, i: u8) -> Self {
        self.left = self.left.add_to_leftmost(i);
        self
    }

    fn add_to_rightmost(mut self, i: u8) -> Self {
        self.right = self.right.add_to_rightmost(i);
        self
    }

    // Finds and splits the leftmost big number. Returns the new number and whether there has been
    // a split.
    fn split(self) -> (Self, bool) {
        let (left, split_left) = self.left.split();
        let (right, split_right) = if !split_left {
            self.right.split()
        } else {
            (self.right, false)
        };
        (SnailfishNumber { left, right }, split_left || split_right)
    }

    fn reduce(self) -> Self {
        let mut val = self;
        loop {
            let mut exploded;
            loop {
                let result = val.explode(0);
                val = result.0.unwrap_to_num().unwrap();
                exploded = result.3;
                if !exploded {
                    break;
                }
            }
            let result = val.split();
            val = result.0;
            let split = result.1;
            if !split && !exploded {
                break;
            }
        }
        val
    }

    fn magnitude(&self) -> u32 {
        self.left.value() * 3 + self.right.value() * 2
    }
}

impl std::ops::Add for SnailfishNumber {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        SnailfishNumber {
            left: SnailfishNumberValue::Pair(Box::new(self)),
            right: SnailfishNumberValue::Pair(Box::new(other)),
        }
        .reduce()
    }
}

impl fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.left.to_string(), self.right.to_string())
    }
}

impl SnailfishNumberValue {
    fn unwrap_to_num(self) -> Option<SnailfishNumber> {
        match self {
            SnailfishNumberValue::Pair(val) => Some(*val),
            _ => None,
        }
    }

    fn add_to_leftmost(self, i: u8) -> Self {
        match self {
            SnailfishNumberValue::Single(val) => SnailfishNumberValue::Single(val + i),
            SnailfishNumberValue::Pair(val) => {
                SnailfishNumberValue::Pair(Box::new(val.add_to_leftmost(i)))
            }
        }
    }

    fn add_to_rightmost(self, i: u8) -> Self {
        match self {
            SnailfishNumberValue::Single(val) => SnailfishNumberValue::Single(val + i),
            SnailfishNumberValue::Pair(val) => {
                SnailfishNumberValue::Pair(Box::new(val.add_to_rightmost(i)))
            }
        }
    }

    fn split(self) -> (Self, bool) {
        match self {
            SnailfishNumberValue::Single(val) => {
                if val >= 10 {
                    let left = SnailfishNumberValue::Single(val / 2);
                    let right = SnailfishNumberValue::Single((val - 1) / 2 + 1);
                    (
                        SnailfishNumberValue::Pair(Box::new(SnailfishNumber { left, right })),
                        true,
                    )
                } else {
                    (self, false)
                }
            }
            SnailfishNumberValue::Pair(val) => {
                let result = val.split();
                (SnailfishNumberValue::Pair(Box::new(result.0)), result.1)
            }
        }
    }

    fn value(&self) -> u32 {
        match self {
            SnailfishNumberValue::Single(val) => *val as u32,
            SnailfishNumberValue::Pair(val) => val.magnitude(),
        }
    }
}

impl fmt::Display for SnailfishNumberValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnailfishNumberValue::Single(val) => write!(f, "{}", val),
            SnailfishNumberValue::Pair(val) => write!(f, "{}", val),
        }
    }
}

fn parse(cursor: &mut Chars) -> Result<SnailfishNumberValue, MainError> {
    match cursor.next() {
        Some('[') => {
            let left = parse(cursor)?;
            let right = if let Some(',') = cursor.next() {
                parse(cursor)
            } else {
                Err(MainError {
                    _msg: "Error parsing string ".to_owned() + cursor.as_str(),
                })
            }?;
            if let Some(']') = cursor.next() {
                Ok(SnailfishNumberValue::Pair(Box::new(SnailfishNumber {
                    left,
                    right,
                })))
            } else {
                Err(MainError {
                    _msg: "Error parsing string ".to_owned() + cursor.as_str(),
                })
            }
        }
        Some(num) if num.is_digit(10) => {
            Ok(SnailfishNumberValue::Single(num.to_digit(10).unwrap() as u8))
        }
        _ => Err(MainError {
            _msg: "Error parsing string ".to_owned() + cursor.as_str(),
        }),
    }
}

fn main() -> Result<(), MainError> {
    /*
    let lines = read_lines("./input_test")?.flatten();
    for line in lines {
        let mut val = parse(&mut line.chars()).unwrap().unwrap_to_num().unwrap();
        println!("{}", val);
        val = val.explode(0).0.unwrap_to_num().unwrap();
        println!("{}", val);
        println!("");
    }
    */
    let mut lines = read_lines("./input_test")?.flatten();
    let mut sum = parse(&mut lines.next().unwrap().chars())
        .unwrap()
        .unwrap_to_num()
        .unwrap();
    for line in lines {
        let val = parse(&mut line.chars()).unwrap().unwrap_to_num().unwrap();
        sum = sum + val;
    }
    println!("{}", sum);
    println!("{}", sum.magnitude());
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
