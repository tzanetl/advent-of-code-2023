use std::fs;
use std::env;
use std::collections::HashMap;

use log::debug;
use regex::Regex;

use utils::{read_input, set_logging_level};

fn parse_digits(input: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut digits: Vec<i32> = vec![];
    for row in input.lines() {
        let mut row_digits: Vec<char> = vec![];
        for c in row.chars() {
            if c.is_ascii_digit() {
                row_digits.push(c);
            }
        }
        let d: i32 = format!("{}{}", row_digits[0], row_digits.last().unwrap()).parse()?;
        digits.push(d);
    }
    Ok(digits)
}

fn parse_digits_regex(input: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)")?;

    let number_mapping = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let mut digits: Vec<i32> = vec![];

    for row in input.lines() {
        let m = re.find(row).unwrap();
        let digit_s: &str = m.as_str();
        let first: char;
        if digit_s.len() > 1 {
            first = *number_mapping.get(digit_s).unwrap();
        } else {
            first = digit_s.chars().last().unwrap();
        }

        let mut last: char = 'a';
        for i in (0..row.len()).rev() {
            if let Some(m) = re.find(&row[i..]) {
                let digit_s: &str = m.as_str();
                if digit_s.len() > 1 {
                    last = *number_mapping.get(digit_s).unwrap();
                } else {
                    last = digit_s.chars().last().unwrap();
                }
                break;
            }
        }
        let d: i32 = format!("{}{}", first, last).parse()?;
        digits.push(d);
    }
    Ok(digits)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let mut input = read_input(&args);

    let digits_pt1 = parse_digits(&input)?;
    debug!("{:?}", digits_pt1);
    let part_1_sum: i32 = digits_pt1.iter().sum();
    println!("Part 1: {:?}", part_1_sum);

    if args.contains(&"--test".to_string()) {
        input = fs::read_to_string("test_pt2.txt")?;
    }

    let digits_pt2 = parse_digits_regex(&input)?;
    let part_2_sum: i32 = digits_pt2.iter().sum();
    println!("Part 2: {:?}", part_2_sum);

    Ok(())
}
