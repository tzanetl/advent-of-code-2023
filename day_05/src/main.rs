use std::env;
use std::error::Error;

use log::debug;
use regex::Regex;

use utils::{read_input, set_logging_level};

#[derive(Debug)]
struct Path {
    destination: u32,
    source: u32,
    lenght: u32,
}

impl Path {
    fn to_destination(&self, value: &u32) -> Option<u32> {
        if !self.contains(value) {
            return None;
        }
        Some(value - self.source + self.destination)
    }

    fn contains(&self, value: &u32) -> bool {
        (self.source..(self.source + self.lenght)).contains(value)
    }
}

type Map = Vec<Path>;

trait Destination<Map> {
    fn to_destination(&self, value: u32) -> u32;
}

impl Destination<Map> for Map {
    fn to_destination(&self, value: u32) -> u32 {
        for p in self {
            if let Some(destination) = p.to_destination(&value) {
                return destination;
            }
        }
        return value;
    }
}

type Mappings = [Map; 7];

fn walk_index(mappings: &Mappings, index: usize, value: u32) -> u32 {
    let destination = mappings[index].to_destination(value);
    let new_index = index + 1;
    if new_index == mappings.len() {
        return destination;
    } else {
        return walk_index(mappings, new_index, destination);
    }
}

fn part_1(mappings: &Mappings, seeds: &[u32]) -> Vec<u32> {
    let locations: Vec<u32> = seeds
        .iter()
        .map(|seed| walk_index(mappings, 0, *seed))
        .collect();
    return locations;
}

fn parse_input(input: &str) -> Result<(Vec<u32>, Mappings), Box<dyn Error>> {
    let re_seeds: Regex = Regex::new(r"\d+")?;
    let (seed_str, map_str) = input.split_once("seed-to-soil map:").unwrap();

    let seeds: Vec<u32> = re_seeds
        .find_iter(seed_str)
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    let re_map: Regex = Regex::new(r"(\d+) (\d+) (\d+)")?;
    debug!("{map_str}");

    let mut mappings_v: Vec<Vec<Path>> = vec![];
    for block in map_str.split(":") {
        let mut block_maps: Vec<Path> = vec![];
        for line in block.lines() {
            debug!("line: {}", line);
            if let Some(m) = re_map.captures(line) {
                debug!("match {:?}", m);
                let map = Path {
                    destination: m.get(1).unwrap().as_str().parse::<u32>()?,
                    source: m.get(2).unwrap().as_str().parse::<u32>()?,
                    lenght: m.get(3).unwrap().as_str().parse::<u32>()?,
                };
                block_maps.push(map);
            }
        }
        mappings_v.push(block_maps);
        debug!("block done")
    }

    let mappings: Mappings = mappings_v
        .try_into()
        .map_err(|v: Vec<Vec<Path>>| format!("Failed to create Mappings from {} items", v.len()))?;
    Ok((seeds, mappings))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);

    let (seeds, mappings) = parse_input(&input)?;
    debug!("Seeds: {:?}", seeds);
    debug!("Mappings {:?}", mappings);

    let locations = part_1(&mappings, &seeds);
    debug!("Locations: {:?}", locations);
    let min_location_p1 = locations.iter().min().unwrap();
    println!("Part 1: {}", min_location_p1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_contains() {
        let path = Path {
            source: 98,
            destination: 50,
            lenght: 2,
        };
        assert!(!path.contains(&97));
        assert!(path.contains(&98));
        assert!(path.contains(&99));
        assert!(!path.contains(&100));
    }

    #[test]
    fn test_path_to_destination() {
        let path = Path {
            source: 98,
            destination: 50,
            lenght: 2,
        };

        assert_eq!(path.to_destination(&97), None);
        assert_eq!(path.to_destination(&98), Some(50));
        assert_eq!(path.to_destination(&99), Some(51));
        assert_eq!(path.to_destination(&100), None);
    }

    #[test]
    fn test_map_to_destination() {
        let map: Map = vec![
            Path {
                destination: 50,
                source: 98,
                lenght: 2,
            },
            Path {
                destination: 52,
                source: 50,
                lenght: 48,
            },
        ];

        assert_eq!(map.to_destination(10), 10);
        assert_eq!(map.to_destination(97), 99);
        assert_eq!(map.to_destination(98), 50);
    }
}
