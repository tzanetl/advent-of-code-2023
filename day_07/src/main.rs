use std::collections::HashMap;
use std::env;
use std::error::Error;

use log::debug;
use phf::phf_map;

use utils::{read_input, set_logging_level};

static CARD_STREGTH: phf::Map<char, u32> = phf_map! {
    '2' => 0,
    '3' => 1,
    '4' => 2,
    '5' => 3,
    '6' => 4,
    '7' => 5,
    '8' => 6,
    '9' => 7,
    'T' => 8,
    'J' => 9,
    'Q' => 10,
    'K' => 11,
    'A' => 12,
};

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPairs = 2,
    Threes = 3,
    FullHouse = 4,
    Fours = 5,
    Fives = 6,
}

type Powers = [u32; 5];

trait FromCards {
    fn from_cards(cards: &str) -> Self;
}

impl FromCards for Powers {
    fn from_cards(s: &str) -> Self {
        let powers_v: Vec<u32> = s.chars().map(|c| *CARD_STREGTH.get(&c).unwrap()).collect();
        Self::try_from(powers_v).unwrap()
    }
}

impl FromCards for HandType {
    fn from_cards(cards: &str) -> Self {
        let counts = cards.chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
        debug!("Card counts: {:?}", counts);

        // Fives
        if counts.len() == 1 {
            return Self::Fives;
        }
        // Fours
        if counts.len() == 2 && counts.values().max().unwrap() == &4 {
            return Self::Fours;
        }
        // Threes
        if counts.len() == 3 && counts.values().max().unwrap() == &3 {
            return Self::Threes;
        }
        // TwoPairs
        if counts.len() == 3 && counts.values().max().unwrap() == &2 {
            return Self::TwoPairs;
        }
        // OnePair
        if counts.len() == 4 {
            return Self::OnePair;
        }
        // HighCard
        if counts.len() == 5 {
            return Self::HighCard;
        }
        // FullHouse
        return Self::FullHouse;
    }
}

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: u64,
}

fn parse_hands(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            Hand {
                cards: cards,
                bid: bid.parse().unwrap(),
            }
        })
        .collect::<Vec<Hand>>()
}

#[derive(Debug)]
struct TypedHand {
    hand_type: HandType,
    powers: Powers,
    bid: u64,
}

impl TypedHand {
    fn from_hand(hand: &Hand) -> Self {
        Self {
            hand_type: HandType::from_cards(hand.cards),
            powers: Powers::from_cards(hand.cards),
            bid: hand.bid,
        }
    }
}

impl PartialOrd for TypedHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.powers.partial_cmp(&other.powers) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        Some(core::cmp::Ordering::Equal)
    }
}

impl Ord for TypedHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for TypedHand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.powers == other.powers
    }
}

impl Eq for TypedHand {}

fn part_1(typed_hands: &[TypedHand]) -> u64 {
    let mut sorted_hands: Vec<&TypedHand> = typed_hands.iter().map(|h| h).collect();
    sorted_hands.sort();
    debug!("Sorted hands: {:?}", sorted_hands);
    sorted_hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i as u64 + 1) * hand.bid)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);

    let hands = parse_hands(&input);
    debug!("Hands: {:?}", hands);
    let typed_hands: Vec<TypedHand> = hands.iter().map(|h| TypedHand::from_hand(h)).collect();
    debug!("Typed hands: {:?}", typed_hands);
    let winnigs = part_1(&typed_hands);
    println!("Part 1: {}", winnigs);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_hand_type() {
        assert!(HandType::Fives > HandType::Fours);
    }
}
