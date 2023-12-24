use std::collections::HashMap;
use std::env;
use std::error::Error;

use log::{debug, info};

use utils::{read_input, set_logging_level};

#[derive(Debug, PartialEq)]
enum PipeTile {
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Start,
}

#[derive(Debug)]
enum Movement {
    North,
    South,
    West,
    East,
}

impl PipeTile {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            '|' => Some(Self::Vertical),
            '-' => Some(Self::Horizontal),
            'L' => Some(Self::NorthToEast),
            'J' => Some(Self::NorthToWest),
            '7' => Some(Self::SouthToWest),
            'F' => Some(Self::SouthToEast),
            'S' => Some(Self::Start),
            _ => None,
        }
    }

    fn next_point(&self, from: &Point, current: &Point) -> Option<Point> {
        debug!("Tile: {:?}", self);
        debug!("From: {:?}", from);
        debug!("Current: {:?}", current);
        let movement = match self {
            PipeTile::Vertical => {
                if from.0 < current.0 {
                    Movement::South
                } else {
                    Movement::North
                }
            }
            PipeTile::Horizontal => {
                if from.1 < current.1 {
                    Movement::East
                } else {
                    Movement::West
                }
            }
            PipeTile::NorthToEast => {
                if from.0 == current.0 {
                    Movement::North
                } else {
                    Movement::East
                }
            }
            PipeTile::NorthToWest => {
                if from.0 == current.0 {
                    Movement::North
                } else {
                    Movement::West
                }
            }
            PipeTile::SouthToWest => {
                if from.0 == current.0 {
                    Movement::South
                } else {
                    Movement::West
                }
            }
            PipeTile::SouthToEast => {
                if from.0 == current.0 {
                    Movement::South
                } else {
                    Movement::East
                }
            }
            PipeTile::Start => return None,
        };
        debug!("Movemenet: {:?}", movement);
        let new_point = match movement {
            Movement::North => current.0.checked_sub(1).map(|i| (i, current.1)),
            Movement::South => Some((current.0 + 1, current.1)),
            Movement::West => current.1.checked_sub(1).map(|j| (current.0, j)),
            Movement::East => Some((current.0, current.1 + 1)),
        };
        debug!("Next point: {:?}", new_point);
        new_point
    }
}

#[derive(Debug)]
struct Path {
    from: Point,
    current: Point,
}

impl Path {
    fn move_next(&self, pipe: &PipeTile) -> Option<Self> {
        let next_point = match pipe.next_point(&self.from, &self.current) {
            Some(p) => p,
            None => return None,
        };
        Some(Self {
            from: self.current,
            current: next_point,
        })
    }
}

type Point = (usize, usize);
type PipeMap = HashMap<Point, PipeTile>;

fn parse_input(input: &str) -> (Point, PipeMap) {
    let mut start_point: Point = (0, 0);
    let mut map: PipeMap = PipeMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if let Some(pipe) = PipeTile::from_char(&c) {
                if pipe == PipeTile::Start {
                    start_point = (i, j);
                    info!("Start point: {:?}", start_point);
                }
                map.insert((i, j), pipe);
            }
        }
    }
    debug!("Map: {:?}", map);
    (start_point, map)
}

fn part_1(map: &PipeMap, start_point: &Point) -> usize {
    let paths = starting_paths(map, start_point);
    walk_paths(map, paths) / 2
}

fn walk_paths(map: &PipeMap, ini_paths: Vec<Path>) -> usize {
    let mut paths = ini_paths;
    let mut steps: usize = 1;
    loop {
        let mut new_paths: Vec<Path> = vec![];
        steps += 1;
        for p in paths.iter() {
            if let Some(new_path) = p.move_next(map.get(&p.current).unwrap()) {
                if map.get(&new_path.current) == Some(&PipeTile::Start) {
                    return steps;
                }
                new_paths.push(new_path);
            }
        }
        paths = new_paths;
    }
}

fn starting_paths(map: &PipeMap, start_point: &Point) -> Vec<Path> {
    let mut paths: Vec<Path> = vec![];
    let p_south = (start_point.0 + 1, start_point.1);
    if let Some(pipe) = map.get(&p_south) {
        if &pipe
            .next_point(&pipe.next_point(start_point, &p_south).unwrap(), &p_south)
            .unwrap()
            == start_point
        {
            paths.push(Path {
                from: *start_point,
                current: p_south,
            });
        }
    }
    let p_east = (start_point.0, start_point.1 + 1);
    if let Some(pipe) = map.get(&p_east) {
        if &pipe
            .next_point(&pipe.next_point(start_point, &p_east).unwrap(), &p_east)
            .unwrap()
            == start_point
        {
            paths.push(Path {
                from: *start_point,
                current: p_east,
            });
        }
    }
    let p_north: Point;
    if let Some(i) = start_point.0.checked_sub(1) {
        p_north = (i, start_point.1);
        if let Some(pipe) = map.get(&p_north) {
            if &pipe
                .next_point(&pipe.next_point(start_point, &p_north).unwrap(), &p_north)
                .unwrap()
                == start_point
            {
                paths.push(Path {
                    from: *start_point,
                    current: p_north,
                });
            }
        }
    }
    let p_west: Point;
    if let Some(j) = start_point.1.checked_sub(1) {
        p_west = (start_point.0, j);
        if let Some(pipe) = map.get(&p_west) {
            if &pipe
                .next_point(&pipe.next_point(start_point, &p_west).unwrap(), &p_west)
                .unwrap()
                == start_point
            {
                paths.push(Path {
                    from: *start_point,
                    current: p_west,
                });
            }
        }
    }
    info!("Initial points: {:?}", paths);
    paths
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);
    let (start_point, map) = parse_input(&input);

    let steps_p1 = part_1(&map, &start_point);
    println!("Part 1: {}", steps_p1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_tile_south_to_west() {
        let current: Point = (1, 1);
        let pipe = PipeTile::SouthToWest;
        assert_eq!(pipe.next_point(&(1, 0), &current), Some((2, 1)));
        assert_eq!(pipe.next_point(&(2, 1), &current), Some((1, 0)));
    }

    #[test]
    fn test_pipe_tile_south_to_east() {
        let current: Point = (1, 1);
        let pipe = PipeTile::SouthToEast;
        assert_eq!(pipe.next_point(&(1, 2), &current), Some((2, 1)));
        assert_eq!(pipe.next_point(&(2, 1), &current), Some((1, 2)));
    }

    #[test]
    fn test_pipe_tile_vertical() {
        let current: Point = (1, 1);
        let pipe = PipeTile::Vertical;
        assert_eq!(pipe.next_point(&(0, 1), &current), Some((2, 1)));
        assert_eq!(pipe.next_point(&(2, 1), &current), Some((0, 1)));
    }

    #[test]
    fn test_pipe_tile_horizonal() {
        let current: Point = (1, 1);
        let pipe = PipeTile::Horizontal;
        assert_eq!(pipe.next_point(&(1, 0), &current), Some((1, 2)));
        assert_eq!(pipe.next_point(&(1, 2), &current), Some((1, 0)));
    }

    #[test]
    fn test_pipe_tile_north_to_west() {
        let current: Point = (1, 1);
        let pipe = PipeTile::NorthToWest;
        assert_eq!(pipe.next_point(&(0, 1), &current), Some((1, 0)));
        assert_eq!(pipe.next_point(&(1, 0), &current), Some((0, 1)));
    }

    #[test]
    fn test_pipe_tile_north_to_east() {
        let current: Point = (1, 1);
        let pipe = PipeTile::NorthToEast;
        assert_eq!(pipe.next_point(&(0, 1), &current), Some((1, 2)));
        assert_eq!(pipe.next_point(&(1, 2), &current), Some((0, 1)));
    }
}
