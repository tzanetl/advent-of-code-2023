use std::env;
use std::error::Error;

use itertools::izip;
use log::debug;
use regex::Regex;

use utils::{read_input, set_logging_level};

#[derive(Debug, Clone, Copy)]
struct RaceRecord {
    time: f64,
    distance: f64,
}

impl RaceRecord {
    fn calc_edges(&self) -> (f64, f64) {
        let low = (self.time - (self.time.powf(2.0) - 4.0 * self.distance).sqrt()) / 2.0;
        let high = (self.time + (self.time.powf(2.0) - 4.0 * self.distance).sqrt()) / 2.0;
        debug!("low: {}, high: {}", low, high);
        return ((low + 1.0).floor(), (high - 1.0).ceil());
    }

    fn calc_ways(&self) -> u64 {
        let (low, high) = self.calc_edges();
        debug!("low: {}, high: {}", low, high);
        (high - low + 1.0) as u64
    }

    fn combine(&self, other: &Self) -> Self {
        Self {
            time: format!("{}{}.0", self.time as u64, other.time as u64)
                .parse()
                .unwrap(),
            distance: format!("{}{}.0", self.distance as u64, other.distance as u64)
                .parse()
                .unwrap(),
        }
    }
}

fn part_1(records: &[RaceRecord]) -> u64 {
    records
        .iter()
        .map(|race| race.calc_ways())
        .fold(1, |acc, elem| acc * elem)
}

fn part_2(records: &[RaceRecord]) -> u64 {
    let combined: RaceRecord = records[1..]
        .iter()
        .fold(records[0], |acc, other| acc.combine(other));
    debug!("Combined: {:?}", combined);
    combined.calc_ways()
}

fn parse_inputs(input: &str) -> Vec<RaceRecord> {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut input_lines = input.lines();
    let time_line = input_lines.next().unwrap();
    let distance_line = input_lines.next().unwrap();

    let races: Vec<RaceRecord> = izip!(re.find_iter(time_line), re.find_iter(distance_line))
        .map(|(time, distance)| RaceRecord {
            time: time.as_str().parse::<f64>().unwrap(),
            distance: distance.as_str().parse::<f64>().unwrap(),
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

    log::set_max_level(log::LevelFilter::Debug);
    let part_2 = part_2(&race_records);
    println!("Part 2: {}", part_2);

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

    #[test]
    fn test_race_record_combine() {
        let race_1 = RaceRecord {
            time: 7.0,
            distance: 9.0,
        };
        let race_2 = RaceRecord {
            time: 15.0,
            distance: 40.0,
        };
        let race_3 = race_1.combine(&race_2);
        assert_eq!(race_3.time, 715.0);
        assert_eq!(race_3.distance, 940.0);
    }
}
