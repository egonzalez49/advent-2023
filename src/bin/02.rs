use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

advent_of_code::solution!(2);

struct Cube<'a> {
    color: &'a str,
    amount: u32,
}

struct Game<'a> {
    id: &'a str,
    hands: Vec<Vec<Cube<'a>>>,
}

impl<'a> Game<'a> {
    fn is_valid(&self, map: &BTreeMap<&str, u32>) -> bool {
        self.hands.iter().all(|hand| {
            hand.iter()
                .all(|c| c.amount <= *map.get(c.color).expect("should have max count"))
        })
    }

    fn parsed_id(&self) -> u32 {
        self.id.parse::<u32>().expect("should be a valid number")
    }

    fn minimum_cube_power(&self) -> u32 {
        let map: BTreeMap<&str, u32> = BTreeMap::new();
        self.hands
            .iter()
            .fold(map, |mut acc, hand| {
                for cube in hand.iter() {
                    acc.entry(cube.color)
                        .and_modify(|v| *v = (*v).max(cube.amount))
                        .or_insert(cube.amount);
                }
                acc
            })
            .values()
            .product()
    }
}

fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    Ok((input, Cube { color, amount }))
}

fn hand(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;
    Ok((input, cubes))
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), digit1)(input)?;
    let (input, hands) = preceded(tag(": "), separated_list1(tag("; "), hand))(input)?;
    Ok((input, Game { id, hands }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = BTreeMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let (_, games) = parse_games(input).expect("should be parsable");
    Some(
        games
            .iter()
            .filter(|game| game.is_valid(&map))
            .map(|game| game.parsed_id())
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, games) = parse_games(input).expect("should be parsable");
    Some(
        games
            .iter()
            .map(|game| game.minimum_cube_power())
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
