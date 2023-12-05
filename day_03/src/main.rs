use std::collections::{HashMap, HashSet};
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
        let number: u32 = m.as_str().parse()?;
        match is_part_number(&continous, checkpoints) {
            NumberType::RANDOM => debug!("{} is not a part", number),
            NumberType::PART | NumberType::GEAR(_) => {
                part_numbers.push(number);
            }
        }
    }

    Ok(part_numbers)
}

fn parse_gear_ratios(input: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut gear_ratios: Vec<u32> = vec![];

    let row_len: usize = input.lines().next().unwrap().len();
    debug!("Row lenght: {0}", row_len);

    let continous: String = input.lines().into_iter().collect::<Vec<&str>>().join("");
    let total_len = continous.len();

    let mut gear_cache: HashMap<usize, u32> = HashMap::new();

    let re = Regex::new(r"\d+").unwrap();
    for m in re.find_iter(&continous) {
        debug!("{:?}", m);
        let checkpoints = get_checkpoints(m.start(), m.end(), &row_len, &total_len);
        debug!("{:?}", checkpoints);
        let number: u32 = m.as_str().parse()?;
        match is_part_number(&continous, checkpoints) {
            NumberType::RANDOM => debug!("{} is not a part", number),
            NumberType::PART => {}
            NumberType::GEAR(gear_position) => {
                if let Some(old_part) = gear_cache.remove(&gear_position) {
                    debug!("gear found at position {}", gear_position);
                    debug!("old part {}", old_part);
                    let ratio: u32 = old_part * number;
                    gear_ratios.push(ratio);
                } else {
                    gear_cache.insert(gear_position, number);
                }
                debug!("cache {:?}", gear_cache);
            }
        }
    }

    Ok(gear_ratios)
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

enum NumberType {
    RANDOM,
    PART,
    GEAR(usize),
}

fn is_part_number(input: &str, checkpoints: HashSet<usize>) -> NumberType {
    for ind in checkpoints {
        let c = &input[ind..ind + 1].chars().next().unwrap();

        if c == &'*' {
            return NumberType::GEAR(ind);
        } else if c.is_numeric() || c == &'.' {
            continue;
        }
        return NumberType::PART;
    }
    NumberType::RANDOM
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

    let gear_ratios = parse_gear_ratios(&input)?;
    debug!("{:?}", gear_ratios);
    let gear_ratio_sum: u32 = gear_ratios.iter().sum();
    println!("Part 2: {}", gear_ratio_sum);

    Ok(())
}
