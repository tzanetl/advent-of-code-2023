use std::collections::HashSet;
use std::env;
use std::error::Error;

use log::debug;
use regex::Regex;

use utils::{read_input, set_logging_level};

fn parse_part_numbers(input: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut part_numbers: Vec<u32> = vec![];
    let row_len: usize = input.lines().next().unwrap().len();
    debug!("Row lenght: {0}", row_len);

    let continous: String = input.lines().into_iter().collect::<Vec<&str>>().join("");
    let total_len = continous.len();

    let re = Regex::new(r"\d+").unwrap();
    for m in re.find_iter(&continous) {
        debug!("{:?}", m);
        let checkpoints = get_checkpoints(m.start(), m.end(), &row_len, &total_len);
        debug!("{:?}", checkpoints);
        if is_part_number(&continous, checkpoints) {
            let number: u32 = m.as_str().parse()?;
            part_numbers.push(number);
        } else {
            debug!("{} is not a part number", m.as_str());
        }
    }

    Ok(part_numbers)
}

fn get_checkpoints(start: usize, end: usize, row_len: &usize, total_len: &usize) -> HashSet<usize> {
    let mut checkpoints: HashSet<usize> = HashSet::new();

    let left: usize;
    if let Some(left_) = start.checked_sub(1) {
        left = left_;
        checkpoints.insert(left);
    } else {
        left = start;
    }

    let right: usize;
    if (end + 1) < *total_len {
        right = end + 1;
        checkpoints.insert(end);
    } else {
        right = end;
    }

    for i in make_range(left, right) {
        if let Some(point_above) = i.checked_sub(*row_len) {
            checkpoints.insert(point_above);
        }

        let point_below: usize = i + row_len;
        if &point_below < total_len {
            checkpoints.insert(point_below);
        }
    }

    checkpoints
}

fn is_part_number(input: &str, checkpoints: HashSet<usize>) -> bool {
    for ind in checkpoints {
        let c = &input[ind..ind + 1].chars().next().unwrap();

        if c.is_numeric() || c == &'.' {
            continue;
        }
        return true;
    }
    false
}

fn make_range<T: std::cmp::PartialOrd>(left: T, right: T) -> Box<std::ops::Range<T>> {
    if left < right {
        Box::new(left..right)
    } else {
        Box::new(right..left)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);

    let part_numbers_p1 = parse_part_numbers(&input)?;
    debug!("{:?}", part_numbers_p1);
    let schematic_number_p1: u32 = part_numbers_p1.iter().sum();
    println!("Part 1: {}", schematic_number_p1);

    Ok(())
}
