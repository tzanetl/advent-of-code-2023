use std::env;
use std::error::Error;

use itertools::izip;
use log::debug;
use regex::Regex;

use utils::{read_input, set_logging_level};

#[derive(Debug)]
struct RaceRecord {
    time: f32,
    distance: f32,
}

impl RaceRecord {
    fn calc_edges(&self) -> (f32, f32) {
        let low = (self.time - (self.time.powf(2.0) - 4.0 * self.distance).sqrt()) / 2.0;
        let high = (self.time + (self.time.powf(2.0) - 4.0 * self.distance).sqrt()) / 2.0;
        debug!("low: {}, high: {}", low, high);
        return ((low + 1.0).floor(), (high - 1.0).ceil());
    }

    fn calc_ways(&self) -> u32 {
        let (low, high) = self.calc_edges();
        debug!("low: {}, high: {}", low, high);
        (high - low + 1.0) as u32
    }
}

fn part_1(records: &[RaceRecord]) -> u32 {
    records
        .iter()
        .map(|race| race.calc_ways())
        .fold(1, |acc, elem| acc * elem)
}

fn parse_inputs(input: &str) -> Vec<RaceRecord> {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut input_lines = input.lines();
    let time_line = input_lines.next().unwrap();
    let distance_line = input_lines.next().unwrap();

    let races: Vec<RaceRecord> = izip!(re.find_iter(time_line), re.find_iter(distance_line))
        .map(|(time, distance)| RaceRecord {
            time: time.as_str().parse::<f32>().unwrap(),
            distance: distance.as_str().parse::<f32>().unwrap(),
        })
        .collect();
    debug!("Races: {:?}", races);
    return races;
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);

    let race_records = parse_inputs(&input);
    let part_1 = part_1(&race_records);
    println!("Part 1: {}", part_1);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_record_calc_edges() {
        let race = RaceRecord {
            time: 7.0,
            distance: 9.0,
        };
        let (low, high) = race.calc_edges();
        assert_eq!(low, 2.0);
        assert_eq!(high, 5.0);
    }
}
