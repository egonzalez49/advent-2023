use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::multispace0,
    combinator::iterator,
    sequence::terminated,
    IResult, Parser,
};
use nom_locate::LocatedSpan;

advent_of_code::solution!(10);

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

#[derive(Clone, Debug, PartialEq)]
enum Pipe {
    Empty,
    Start,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Clone, Debug)]
struct Square<'a> {
    item: Pipe,
    data: SpanIVec2<'a>,
}

impl<'a> Square<'a> {
    fn fragment(&self) -> &str {
        self.data.fragment()
    }

    fn position(&self) -> IVec2 {
        self.data.extra
    }

    fn is_boundary(&self, loop_squares: &HashSet<IVec2>) -> bool {
        loop_squares.contains(&self.position())
            && matches!(
                self.item,
                Pipe::NorthSouth | Pipe::NorthEast | Pipe::NorthWest
            )
    }

    fn surrounding_pos(&self) -> Vec<IVec2> {
        let positions = match self.item {
            Pipe::Empty => vec![],
            Pipe::NorthSouth => vec![IVec2::new(0, -1), IVec2::new(0, 1)],
            Pipe::EastWest => vec![IVec2::new(1, 0), IVec2::new(-1, 0)],
            Pipe::NorthEast => vec![IVec2::new(0, -1), IVec2::new(1, 0)],
            Pipe::NorthWest => vec![IVec2::new(0, -1), IVec2::new(-1, 0)],
            Pipe::SouthEast => vec![IVec2::new(0, 1), IVec2::new(1, 0)],
            Pipe::SouthWest => vec![IVec2::new(0, 1), IVec2::new(-1, 0)],
            _ => vec![
                IVec2::new(-1, 0),
                IVec2::new(0, 1),
                IVec2::new(1, 0),
                IVec2::new(-1, 0),
            ],
        };

        positions
            .into_iter()
            .map(|pos| pos + self.position())
            .collect::<Vec<IVec2>>()
    }
}

fn determine_pipe(c: &str) -> Pipe {
    match c {
        "|" => Pipe::NorthSouth,
        "-" => Pipe::EastWest,
        "L" => Pipe::NorthEast,
        "J" => Pipe::NorthWest,
        "F" => Pipe::SouthEast,
        "7" => Pipe::SouthWest,
        _ => panic!("invalid character!"),
    }
}

fn with_xy(span: Span) -> SpanIVec2 {
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    span.map_extra(|_| IVec2::new(x, y))
}

fn parse_grid(input: Span) -> IResult<Span, HashMap<IVec2, Square>> {
    let mut it = iterator(
        input,
        terminated(
            alt((
                tag("S").map(|span| with_xy(span)).map(|span| Square {
                    item: Pipe::Start,
                    data: span,
                }),
                take_while_m_n(1, 1, |c: char| ".\nS".chars().all(|other| other != c))
                    .map(|span| with_xy(span))
                    .map(|span| Square {
                        item: determine_pipe(span.fragment()),
                        data: span,
                    }),
                tag(".").map(|span| with_xy(span)).map(|span| Square {
                    item: Pipe::Empty,
                    data: span,
                }),
            )),
            multispace0,
        ),
    );

    let parsed: HashMap<IVec2, Square> = it.fold(HashMap::new(), |mut acc, square| {
        acc.insert(square.data.extra, square);
        acc
    });

    let res: IResult<_, _> = it.finish();

    res.map(|(input, _)| (input, parsed))
}

fn loop_squares(grid: &HashMap<IVec2, Square>) -> HashSet<IVec2> {
    let start_square = grid
        .values()
        .find(|square| matches!(square.item, Pipe::Start))
        .expect("should have start square");

    let mut curr_square = start_square
        .surrounding_pos()
        .into_iter()
        .filter_map(|pos| grid.get(&pos))
        .find(|square| square.surrounding_pos().contains(&start_square.position()))
        .expect("should have a compatible pipe");

    let mut prev_pos = start_square.position();

    let mut set = HashSet::from([start_square.position()]);

    loop {
        let placeholder = curr_square.position();

        curr_square = curr_square
            .surrounding_pos()
            .into_iter()
            .filter(|pos| *pos != prev_pos)
            .filter_map(|pos| grid.get(&pos))
            .next()
            .expect("should have compatible pipe");

        prev_pos = placeholder;
        set.insert(placeholder);

        if curr_square.fragment() == "S" {
            break;
        }
    }

    set
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, grid) = parse_grid(Span::new(input)).expect("should be parsable");

    let set = loop_squares(&grid);

    Some(set.len() as u32 / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, grid) = parse_grid(Span::new(input)).expect("should be parsable");

    let max_dims = grid
        .keys()
        .max_by(|a, b| match a.x.cmp(&b.x) {
            Ordering::Equal => a.y.cmp(&b.y),
            ordering => ordering,
        })
        .expect("should have max dimensions");

    let length = max_dims.x;
    let height = max_dims.y;

    let set = loop_squares(&grid);

    let mut inside_ground = 0;

    (0..=height).for_each(|y| {
        let mut parity = 0;
        (0..=length).for_each(|x| {
            let pos = IVec2::from_array([x, y]);

            let square = grid.get(&pos).expect("should be valid position");
            if square.is_boundary(&set) {
                parity += 1;
            } else if !set.contains(&square.position()) && parity % 2 == 1 {
                inside_ground += 1;
            }
        })
    });

    Some(inside_ground)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10));
    }
}
