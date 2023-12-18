use std::error::Error;
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

fn count_steps(steps: &[Side], nodes: &HashMap<&str, (&str, &str)>) -> Option<usize> {
    let mut destination: &str = &"AAA";
    for (i, step) in steps.iter().cycle().enumerate() {
        let options = nodes.get(destination).unwrap();
        destination = match step {
            Side::LEFT => options.0,
            Side::RIGHT => options.1,
        };
        if destination == "ZZZ" {
            return Some(i + 1);
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);

    let (steps, nodes) = parse_input(&input);
    let steps_p1 = count_steps(&steps, &nodes).unwrap();
    println!("Part 1: {}", steps_p1);

    Ok(())
}
