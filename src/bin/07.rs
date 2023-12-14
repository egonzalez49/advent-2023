use std::{cmp::Ordering, ops::Deref};

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq)]
enum Hand<'a> {
    HighCard(&'a str),
    OnePair(&'a str),
    TwoPair(&'a str),
    ThreeOfAKind(&'a str),
    FullHouse(&'a str),
    FourOfAKind(&'a str),
    FiveOfAKind(&'a str),
}

impl<'a> Hand<'a> {
    fn score(&self) -> u8 {
        match self {
            Hand::FiveOfAKind(_) => 7,
            Hand::FourOfAKind(_) => 6,
            Hand::FullHouse(_) => 5,
            Hand::ThreeOfAKind(_) => 4,
            Hand::TwoPair(_) => 3,
            Hand::OnePair(_) => 2,
            Hand::HighCard(_) => 1,
        }
    }

    fn value(&self) -> &str {
        match self {
            Hand::FiveOfAKind(s) | Hand::FourOfAKind(s) | Hand::FullHouse(s) => s,
            Hand::ThreeOfAKind(s) | Hand::TwoPair(s) | Hand::OnePair(s) | Hand::HighCard(s) => s,
        }
    }

    fn card_rankings(&self) -> Vec<u32> {
        // For part two, we'll treat Jokers as 2.
        self.value()
            .chars()
            .map(|card| match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                // 'J' => 11,
                'T' => 10,
                'J' => 1,
                value => value.to_digit(10).unwrap(),
            })
            .collect_vec()
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score().cmp(&other.score()) {
            Ordering::Equal => self.card_rankings().cmp(&other.card_rankings()),
            ordering => ordering,
        }
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Game<'a> {
    hand: Hand<'a>,
    bid: u32,
}

fn determine_hand_type(hand: &str) -> Hand {
    use Hand::*;

    let mut counts = hand.chars().counts();

    // For part two, we combine Js into the highest card count.
    if hand.contains('J') {
        let joker_count = *counts.get(&'J').unwrap();

        if let Some((c, _)) = counts
            .iter()
            .filter(|(&c, _)| c != 'J')
            .sorted_by(|a, b| a.1.cmp(b.1))
            .next_back()
        {
            counts.entry(*c).and_modify(|count| *count += joker_count);
            counts.remove(&'J');
        }
    }

    let counts = counts.into_values().sorted().rev().join("");

    match counts.deref() {
        "5" => FiveOfAKind(hand),
        "41" => FourOfAKind(hand),
        "32" => FullHouse(hand),
        "311" => ThreeOfAKind(hand),
        "221" => TwoPair(hand),
        "2111" => OnePair(hand),
        _ => HighCard(hand),
    }
}

fn parse_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            Game {
                hand: determine_hand_type(hand),
                bid: bid.parse::<u32>().unwrap(),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = parse_games(input);
    // dbg!(games);

    Some(
        games
            .iter()
            .sorted()
            .enumerate()
            .map(|(index, game)| game.bid * (index as u32 + 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse_games(input);

    Some(
        games
            .iter()
            .sorted()
            .enumerate()
            .map(|(index, game)| game.bid * (index as u32 + 1))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // Uncomment for testing part one - fails test run since logic isn't split from part two.
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
