use std::collections::{BTreeMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, space0, space1},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult, Parser,
};

advent_of_code::solution!(4);

struct Card {
    nums: HashSet<u32>,
    winning_nums: HashSet<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let power = self.matches();
        if power == 0 {
            0
        } else {
            2u32.pow(power - 1)
        }
    }

    fn matches(&self) -> u32 {
        self.winning_nums.intersection(&self.nums).count() as u32
    }
}

fn set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut acc, num| {
            acc.insert(num);
            acc
        },
    )(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, _) = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space1)),
    )(input)?;
    separated_pair(set, tuple((tag("|"), space1)), set)
        .map(|(winning_nums, nums)| Card { winning_nums, nums })
        .parse(input)
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, card)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, cards) = parse_cards(input).expect("should be parsable");
    Some(cards.iter().map(|card| card.score()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, cards) = parse_cards(input).expect("should be parsable");
    let scores = cards.iter().map(|card| card.matches()).collect::<Vec<_>>();
    Some(
        scores
            .iter()
            .enumerate()
            .fold(
                (0..cards.len())
                    .map(|index| (index, 1))
                    .collect::<BTreeMap<usize, u32>>(),
                |mut acc, (index, score)| {
                    for offset in (0..=*score).skip(1) {
                        let copies = acc[&index];
                        acc.entry(index + offset as usize)
                            .and_modify(|c| *c += copies);
                    }
                    acc
                },
            )
            .values()
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
