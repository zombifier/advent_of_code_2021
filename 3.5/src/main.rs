use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let lines: Vec<Vec<u8>> = read_lines("./input")?
        .into_iter()
        .filter_map(|line| line.ok().map(|value| value.as_bytes().to_owned()))
        .collect();
    let oxygen = line_to_num(find(lines.iter().collect(), true));
    let co2 = line_to_num(find(lines.iter().collect(), false));
    println!("{} * {} = {}", oxygen, co2, oxygen * co2);
    Ok(())
}

// The output`FromIterator<&Vec<u8>>` is not implemented for `[&Vec<u8>]`
// is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn line_to_num(line: &[u8]) -> u32 {
    let mut result = 0;
    for (i, x) in line.iter().enumerate() {
        if *x == '1' as u8 {
            result += 1 << (line.len() - i - 1);
        }
    }
    result
}

fn filter_bit_criteria(lines: Vec<&Vec<u8>>, index: usize) -> (Vec<&Vec<u8>>, Vec<&Vec<u8>>) {
    let mut result_more: Vec<&Vec<u8>> = Vec::new();
    let mut result_less: Vec<&Vec<u8>> = Vec::new();
    let mut one_count = 0;
    for line in &lines {
        if line[index] == '1' as u8 {
            one_count += 1;
        }
    }
    let more_common_bit = {
        if one_count < (lines.len() - one_count) {
            '0'
        } else {
            '1'
        }
    };
    for line in lines {
        if line[index] == more_common_bit as u8 {
            result_more.push(line);
        } else {
            result_less.push(line);
        }
    }
    (result_more, result_less)
}

fn find(lines: Vec<&Vec<u8>>, is_oxygen: bool) -> &Vec<u8> {
    let mut result = lines;
    let mut index = 0;
    let max_index = result[0].len();
    loop {
        result = if is_oxygen {
            filter_bit_criteria(result, index).0
        } else {
            filter_bit_criteria(result, index).1
        };
        index += 1;
        if index == max_index {
            index = 0;
        }
        if result.len() == 1 {
            return result[0];
        }
    }
}
