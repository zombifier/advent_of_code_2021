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

fn hex_char_to_bin(c: &char) -> Option<String> {
    c.to_digit(16).map(|v| format!("{:04b}", v))
}

fn bin_to_num(string: &str) -> Option<u64> {
    u64::from_str_radix(string, 2).ok()
}

// Advance the cursor (idx) by the specified count and return the substring.
fn advance<'a, 'b>(string: &'a str, count: usize, idx: &'b mut usize) -> &'a str {
    *idx += count;
    &string[*idx - count..*idx]
}

struct PacketParseResult {
    length: usize,
    payload: u64,
}

// Parse the packet.
fn parse_packet(string: &str) -> Result<PacketParseResult, MainError> {
    let mut idx = 0;
    let _version = bin_to_num(advance(&string, 3, &mut idx)).ok_or("Version parse error")?;
    let type_id = bin_to_num(advance(&string, 3, &mut idx)).ok_or("Type ID parse error")?;
    let payload = if type_id == 4 {
        let mut payload_string = String::new();
        loop {
            let subpacket_header = advance(&string, 1, &mut idx);
            payload_string += advance(&string, 4, &mut idx);
            if subpacket_header == "0" {
                break;
            }
        }
        bin_to_num(&payload_string).ok_or(format!("Format parse error: {}", payload_string))?
    } else {
        let mut sub_payloads = Vec::new();
        let length_type_id = advance(&string, 1, &mut idx);
        match length_type_id {
            "0" => {
                let payload_length = bin_to_num(advance(&string, 15, &mut idx))
                    .ok_or("Payload length parse error")?
                    as usize;
                let mut subpacket_length = 0;
                while subpacket_length < payload_length {
                    let result = parse_packet(&string[idx..])?;
                    idx += result.length;
                    subpacket_length += result.length;
                    sub_payloads.push(result.payload);
                }
            }
            "1" => {
                let payload_count = bin_to_num(advance(&string, 11, &mut idx))
                    .ok_or("Payload count parse error")?
                    as usize;
                for _ in 0..payload_count {
                    let result = parse_packet(&string[idx..])?;
                    idx += result.length;
                    sub_payloads.push(result.payload);
                }
            }
            _ => panic!("Inconceivable!"),
        }
        match type_id {
            0 => sub_payloads.iter().sum(),
            1 => sub_payloads.iter().product(),
            2 => *sub_payloads.iter().min().unwrap_or(&0),
            3 => *sub_payloads.iter().max().unwrap_or(&0),
            5 => {
                if sub_payloads[0] > sub_payloads[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                if sub_payloads[0] < sub_payloads[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                if sub_payloads[0] == sub_payloads[1] {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        }
    };
    Ok(PacketParseResult {
        length: idx,
        payload,
    })
}

fn main() -> Result<(), MainError> {
    let lines = read_lines("./input")?.flatten().map(|line| {
        line.chars()
            .filter_map(|c| hex_char_to_bin(&c))
            .collect::<String>()
    });
    for line in lines {
        println!("{}", parse_packet(&line)?.payload);
    }
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
