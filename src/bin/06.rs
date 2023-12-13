use nom::{
    bytes::complete::is_not,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::ParserExt;

advent_of_code::solution!(6);

fn nums(input: &str) -> IResult<&str, Vec<u64>> {
    is_not("0123456789")
        .precedes(separated_list1(space1, complete::u64))
        .parse(input)
}

fn parse_numbers(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    separated_pair(nums, line_ending, nums).parse(input)
}

fn join_nums(a: Vec<u64>) -> u64 {
    a.into_iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .expect("should be parsable")
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (times, distances)) = parse_numbers(input).expect("should be parsable");

    Some(
        times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| {
                (1..time).filter(|ms| (time - ms) * ms > distance).count() as u32
            })
            .product::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (times, distances)) = parse_numbers(input).expect("should be parsable");

    let time = join_nums(times);
    let distance = join_nums(distances);

    Some(
        (1..time)
            .map(|ms| if (time - ms) * ms > distance { 1 } else { 0 })
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
