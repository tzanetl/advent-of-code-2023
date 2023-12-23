use std::env;
use std::error::Error;

use log::debug;

use utils::{read_input, set_logging_level};

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|ns| ns.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn walk_history(history: &[i64]) -> i64 {
    let mut new_line: Vec<i64> = vec![];
    let mut i: usize = 1;
    let mut all_zeros = true;
    while i < history.len() {
        let value = history[i] - history[i - 1];
        if value != 0 {
            all_zeros = false;
        }
        new_line.push(value);
        i += 1;
    }
    if !all_zeros {
        new_line.push(walk_history(&new_line));
    } else {
        new_line.push(0);
    }
    let result = history.last().unwrap() + new_line.last().unwrap();
    debug!("History: {:?}", history);
    debug!("New history: {:?}", new_line);
    debug!("All zeros: {}", all_zeros);
    debug!("New value: {}", result);
    return history.last().unwrap() + new_line.last().unwrap();
}

fn part_1(histories: &[Vec<i64>]) -> i64 {
    return histories.iter().map(|hist| walk_history(&hist[..])).sum();
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);

    let histories = parse_input(&input);
    let explorations = part_1(&histories);
    println!("Part 1: {}", explorations);

    Ok(())
}
