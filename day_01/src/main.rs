use std::env;

use log::debug;

use utils::{read_input, set_logging_level};

fn parse_digits(content: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut digits: Vec<i32> = vec![];
    for row in content.lines() {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);

    let digits = parse_digits(&input)?;
    debug!("{:?}", digits);
    let part_1_sum: i32 = digits.iter().sum();
    println!("Part 1: {:?}", part_1_sum);
    Ok(())
}
