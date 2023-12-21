use std::error::Error;
use std::fs;
use std::{collections::HashMap, env};

use log::debug;
use regex::Regex;

use utils::{read_input, set_logging_level};

#[derive(Debug)]
enum Side {
    LEFT,
    RIGHT,
}

impl Side {
    fn from_char(c: &char) -> Option<Self> {
        debug!("Char: {}", c);
        match c {
            &'L' => Some(Self::LEFT),
            &'R' => Some(Self::RIGHT),
            _ => None,
        }
    }
}

fn parse_input(input: &str) -> (Vec<Side>, HashMap<&str, (&str, &str)>) {
    let (steps_str, node_str) = input.split_once("\r\n\r\n").unwrap();
    debug!("Steps str: {}", steps_str);
    let steps: Vec<Side> = steps_str
        .chars()
        .map(|c| Side::from_char(&c).unwrap())
        .collect();
    debug!("Steps: {:?}", steps);

    let re = Regex::new(r"(\w\w\w) = \((\w\w\w), (\w\w\w)\)").unwrap();
    let nodes: HashMap<&str, (&str, &str)> = re
        .captures_iter(node_str)
        .map(|m| {
            (
                m.get(1).unwrap().as_str(),
                (m.get(2).unwrap().as_str(), m.get(3).unwrap().as_str()),
            )
        })
        .collect();
    debug!("Nodes: {:?}", nodes);
    (steps, nodes)
}

fn count_steps(
    steps: &[Side],
    nodes: &HashMap<&str, (&str, &str)>,
    start: &str,
    end: &str,
) -> Option<usize> {
    let mut destination: &str = start;
    for (i, step) in steps.iter().cycle().enumerate() {
        let options = nodes.get(destination).unwrap();
        destination = match step {
            Side::LEFT => options.0,
            Side::RIGHT => options.1,
        };
        if destination.ends_with(end) {
            return Some(i + 1);
        }
    }
    None
}

fn count_steps_simul(steps: &[Side], nodes: &HashMap<&str, (&str, &str)>) -> usize {
    let destinations: Vec<&str> = nodes
        .keys()
        .filter(|n| n.ends_with("A"))
        .map(|n| *n)
        .collect();
    debug!("Initial destinations: {:?}", destinations);
    let first_goals: Vec<usize> = destinations
        .iter()
        .map(|start| count_steps(steps, nodes, start, &"Z").unwrap())
        .collect();
    debug!("First goals: {:?}", first_goals);

    return lcm(&first_goals).unwrap();
}

// https://en.wikipedia.org/wiki/Least_common_multiple#Using_the_greatest_common_divisor
fn lcm(nums: &[usize]) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }
    let a = nums[0];
    if nums.len() == 1 {
        return Some(a);
    }
    let b = lcm(&nums[1..]).unwrap();
    return Some((a * b) / gcd(a, b));
}

// https://en.wikipedia.org/wiki/Greatest_common_divisor#Euclidean_algorithm
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let mut input = read_input(&args);

    let (mut steps, mut nodes) = parse_input(&input);

    let steps_p1 = count_steps(&steps, &nodes, &"AAA", &"ZZZ").unwrap();
    println!("Part 1: {}", steps_p1);

    if args.contains(&"--test".to_string()) {
        input = fs::read_to_string("test_pt2.txt")?;
        (steps, nodes) = parse_input(&input);
    }

    let steps_p2 = count_steps_simul(&steps, &nodes);
    println!("Part 2: {}", steps_p2);

    Ok(())
}
