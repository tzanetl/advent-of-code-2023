use std::collections::{HashSet, VecDeque};
use std::env;
use std::error::Error;

use log::debug;
use regex::Regex;

use utils::{read_input, set_logging_level};

fn parse_scores(input: &str) -> Vec<usize> {
    let mut scores: Vec<usize> = vec![];

    let re = Regex::new(r"\d+").unwrap();

    for row in input.lines() {
        let (_, stripped) = row.split_once(":").unwrap();
        let (left_str, right_str) = stripped.split_once("|").unwrap();
        let left_numbers: HashSet<u32> = re
            .find_iter(left_str)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        debug!("Left {:?}", left_numbers);
        let right_numbers: HashSet<u32> = re
            .find_iter(right_str)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        debug!("Right {:?}", right_numbers);
        let count = right_numbers.intersection(&left_numbers).count();
        debug!("count {}", count);
        let score: usize;
        if count > 0 {
            score = 2_usize.pow((count - 1) as u32);
        } else {
            score = 0;
        }
        debug!("score {}", score);
        scores.push(score);
    }
    scores
}

#[derive(Debug)]
struct Card {
    points: usize,
    copies: usize,
}

fn parse_with_winnings(input: &str) -> usize {
    let mut card_stack: VecDeque<Card> = VecDeque::new();

    let re = Regex::new(r"\d+").unwrap();

    for row in input.lines() {
        let (_, stripped) = row.split_once(":").unwrap();
        let (left_str, right_str) = stripped.split_once("|").unwrap();
        let left_numbers: HashSet<u32> = re
            .find_iter(left_str)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        debug!("Left {:?}", left_numbers);
        let right_numbers: HashSet<u32> = re
            .find_iter(right_str)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        debug!("Right {:?}", right_numbers);
        let points = right_numbers.intersection(&left_numbers).count();
        debug!("points {}", points);
        card_stack.push_back(Card { points, copies: 1 })
    }
    debug!("initial card stack \n{:?}", card_stack);

    let mut processed_cards: Vec<Card> = Vec::new();
    while let Some(card) = card_stack.pop_front() {
        debug!("{:?}", card);
        for i in 0..card.points {
            card_stack[i].copies += card.copies;
        }
        processed_cards.push(card);
    }
    processed_cards
        .iter()
        .fold(0, |acc, card| acc + card.copies)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);

    let scores_p1 = parse_scores(&input);
    let sum_p1: usize = scores_p1.iter().sum();
    println!("Part 1: {}", sum_p1);
    let total_cards = parse_with_winnings(&input);
    println!("Part 2: {}", total_cards);

    Ok(())
}
