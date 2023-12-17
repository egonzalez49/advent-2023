use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(9);

fn parse_sequences(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(line_ending, separated_list1(space1, complete::i32))(input)
}

fn interpolate_forward(seq: Vec<i32>) -> i32 {
    if seq.iter().all(|n| *n == 0) {
        return 0;
    }

    seq.iter().copied().last().unwrap()
        + interpolate_forward(seq.windows(2).map(|w| w[1] - w[0]).collect_vec())
}

fn interpolate_backward(seq: Vec<i32>) -> i32 {
    if seq.iter().all(|n| *n == 0) {
        return 0;
    }

    seq.iter().copied().next().unwrap()
        - interpolate_backward(seq.windows(2).map(|w| w[1] - w[0]).collect_vec())
}

pub fn part_one(input: &str) -> Option<i32> {
    let (_, sequences) = parse_sequences(input).expect("should be parsable");

    Some(sequences.into_iter().map(interpolate_forward).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let (_, sequences) = parse_sequences(input).expect("should be parsable");

    Some(sequences.into_iter().map(interpolate_backward).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(107));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(17));
    }
}
