use std::collections::BTreeMap;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, line_ending},
    multi::{fold_many1, many1},
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

advent_of_code::solution!(8);

type Edges<'a> = (&'a str, &'a str);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn node(input: &str) -> IResult<&str, (&str, Edges)> {
    separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ),
    )(input)
}

fn parse_nodes(input: &str) -> IResult<&str, BTreeMap<&str, Edges>> {
    fold_many1(
        preceded(many1(line_ending), node),
        BTreeMap::new,
        |mut acc, (node, edges)| {
            acc.insert(node, edges);
            acc
        },
    )(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(alt((
        complete::char('L').map(|_| Direction::Left),
        complete::char('R').map(|_| Direction::Right),
    )))(input)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (input, instructions) = parse_instructions(input).expect("should have directions");
    let (_, nodes) = parse_nodes(input).expect("should have nodes");

    let mut curr_node = "AAA";

    Some(
        instructions
            .iter()
            .cycle()
            .enumerate()
            .find_map(|(index, direction)| {
                let edges = nodes.get(curr_node).unwrap();

                curr_node = match direction {
                    Direction::Left => edges.0,
                    Direction::Right => edges.1,
                };

                if curr_node == "ZZZ" {
                    Some(index as u32 + 1)
                } else {
                    None
                }
            })
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (input, instructions) = parse_instructions(input).expect("should have directions");
    let (_, nodes) = parse_nodes(input).expect("should have nodes");

    let start_nodes: Vec<&str> = nodes
        .keys()
        .filter_map(|k| k.ends_with('A').then_some(*k))
        .collect();

    let lengths = start_nodes
        .into_iter()
        .map(|node| {
            let mut curr_node = node;

            instructions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(index, direction)| {
                    let edges = nodes.get(curr_node).unwrap();

                    curr_node = match direction {
                        Direction::Left => edges.0,
                        Direction::Right => edges.1,
                    };

                    if curr_node.ends_with('Z') {
                        Some(index as u64 + 1)
                    } else {
                        None
                    }
                })
                .expect("should reach ending")
        })
        .collect_vec();

    Some(lengths.into_iter().fold(1, lcm))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
