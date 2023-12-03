use std::env;
use std::error::Error;

use log::debug;
use regex::Regex;

use utils::{read_input, set_logging_level};

const LIMIT_RED: u32 = 12;
const LIMIT_GREEN: u32 = 13;
const LIMIT_BLUE: u32 = 14;

fn parse_valid_games(input: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let re = Regex::new(r"Game ([\d]+):")?;

    let mut valid_games: Vec<u32> = vec![];
    for row in input.lines() {
        let m = re.captures(row).unwrap().get(1).unwrap();
        let game_id: u32 = m.as_str().parse()?;
        debug!("Game id: {:?}", game_id);
        let (red, green, blue) = count_cubes(&row)?;
        debug!("R: {:>2}, G: {:>2}, B: {:>2}", red, green, blue);

        if red <= LIMIT_RED && green <= LIMIT_GREEN && blue <= LIMIT_BLUE {
            valid_games.push(game_id);
        }
    }

    Ok(valid_games)
}

#[derive(Debug, Clone)]
struct RGBError {
    color: String,
}

impl std::fmt::Display for RGBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown color: {0}", self.color)
    }
}

impl Error for RGBError {}

type RGB = (u32, u32, u32);

fn count_cubes(input: &str) -> Result<RGB, Box<dyn Error>> {
    let re = Regex::new(r"([\d]+) (red|green|blue)").unwrap();

    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    for caps in re.captures_iter(input) {
        let count: u32 = caps.get(1).unwrap().as_str().parse()?;

        let color = caps.get(2).unwrap().as_str();

        match color {
            "red" => red = std::cmp::max(red, count),
            "green" => green = std::cmp::max(green, count),
            "blue" => blue = std::cmp::max(blue, count),
            &_ => {
                return Err(Box::new(RGBError {
                    color: color.to_string(),
                }))
            }
        }
    }

    Ok((red, green, blue))
}

fn count_game_powers(input: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut game_powers: Vec<u32> = vec![];

    for row in input.lines() {
        let (red, green, blue) = count_cubes(&row)?;
        let power = red * green * blue;
        debug!("Power: {}", power);
        game_powers.push(power);
    }
    Ok(game_powers)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);
    let valid_games_pt1 = parse_valid_games(&input)?;
    let sum_pt1: u32 = valid_games_pt1.iter().sum();
    println!("Part 1: {}", sum_pt1);
    let game_powers = count_game_powers(&input)?;
    let sum_pt2: u32 = game_powers.iter().sum();
    println!("Part 2: {}", sum_pt2);

    Ok(())
}
