use std::collections::{HashMap, HashSet};

use glam::IVec2;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till1},
    character::complete::digit1,
    combinator::iterator,
    IResult, Parser,
};
use nom_locate::LocatedSpan;

advent_of_code::solution!(3);

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

#[derive(Debug, PartialEq)]
enum Value<'a> {
    Empty,
    Symbol(SpanIVec2<'a>),
    Number(SpanIVec2<'a>),
}

fn with_xy(span: Span) -> SpanIVec2 {
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    span.map_extra(|_| IVec2::new(x, y))
}

fn parse_grid(input: Span) -> IResult<Span, Vec<Value>> {
    let mut it = iterator(
        input,
        alt((
            digit1.map(|span| with_xy(span)).map(Value::Number),
            is_not(".\n0123456789")
                .map(|span| with_xy(span))
                .map(Value::Symbol),
            take_till1(|c: char| c.is_ascii_digit() || c != '.' && c != '\n').map(|_| Value::Empty),
        )),
    );

    let parsed = it
        .filter(|value| value != &Value::Empty)
        .collect::<Vec<Value>>();
    let res: IResult<_, _> = it.finish();

    res.map(|(input, _)| (input, parsed))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, objects) = parse_grid(Span::new(input)).unwrap();
    let symbol_set = objects
        .iter()
        .filter_map(|value| match value {
            Value::Empty => None,
            Value::Symbol(sym) => Some(sym.extra),
            Value::Number(_) => None,
        })
        .collect::<HashSet<IVec2>>();

    Some(
        objects
            .iter()
            .filter_map(|value| {
                let Value::Number(num) = value else {
                    return None;
                };

                let surrounding_pos = [
                    // East border
                    IVec2::new(num.fragment().len() as i32, 0),
                    // West border
                    IVec2::new(-1, 0),
                ]
                .into_iter()
                .chain(
                    // North border
                    (-1..=num.fragment().len() as i32).map(|x_offset| IVec2::new(x_offset, 1)),
                )
                .chain(
                    // South border
                    (-1..=num.fragment().len() as i32).map(|x_offset| IVec2::new(x_offset, -1)),
                )
                .map(|pos| pos + num.extra)
                .collect::<Vec<IVec2>>();

                surrounding_pos
                    .iter()
                    .any(|pos| symbol_set.contains(pos))
                    .then_some(
                        num.fragment()
                            .parse::<u32>()
                            .expect("should be a valid number"),
                    )
            })
            .sum(),
    )
}

fn parse_grid_asterisks(input: Span) -> IResult<Span, Vec<Value>> {
    let mut it = iterator(
        input,
        alt((
            digit1.map(|span| with_xy(span)).map(Value::Number),
            tag("*").map(|span| with_xy(span)).map(Value::Symbol),
            take_till1(|c: char| c.is_ascii_digit() || c == '*').map(|_| Value::Empty),
        )),
    );

    let parsed = it
        .filter(|value| value != &Value::Empty)
        .collect::<Vec<Value>>();
    let res: IResult<_, _> = it.finish();

    res.map(|(input, _)| (input, parsed))
}

const POSITIONS: [IVec2; 8] = [
    IVec2::new(0, 1),
    IVec2::new(1, 1),
    IVec2::new(1, 0),
    IVec2::new(1, -1),
    IVec2::new(0, -1),
    IVec2::new(-1, -1),
    IVec2::new(-1, 0),
    IVec2::new(-1, 1),
];

pub fn part_two(input: &str) -> Option<u32> {
    let (_, objects) = parse_grid_asterisks(Span::new(input)).unwrap();
    let num_map = objects
        .iter()
        .filter_map(|value| match value {
            Value::Empty => None,
            Value::Symbol(_) => None,
            Value::Number(num) => Some((num.extra, num.fragment(), num.location_offset())),
        })
        .flat_map(|(v, fragment, id)| {
            (v.x..(v.x + fragment.len() as i32)).map(move |x| (IVec2::new(x, v.y), (id, fragment)))
        })
        .collect::<HashMap<IVec2, (usize, &&str)>>();

    Some(
        objects
            .iter()
            .filter_map(|value| {
                let Value::Symbol(sym) = value else {
                    return None;
                };

                let matching_nums = POSITIONS
                    .iter()
                    .map(|pos| *pos + sym.extra)
                    .filter_map(|border| num_map.get(&border))
                    .unique()
                    .map(|(_, fragment)| fragment.parse::<u32>().expect("should be a valid number"))
                    .collect::<Vec<u32>>();

                (matching_nums.len() == 2).then_some(matching_nums.iter().product::<u32>())
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
