use std::collections::HashSet;

use nom::{
    bytes::complete::{tag, take_until},
    multi::fold_many1,
    sequence::preceded,
    IResult, Parser,
};
use nom_locate::LocatedSpan;

advent_of_code::solution!(11);

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone)]
struct Grid {
    galaxies: HashSet<(usize, usize)>,
    galaxy_rows: HashSet<usize>,
    galaxy_cols: HashSet<usize>,
}

impl Grid {
    fn distances(&self, factor: usize) -> usize {
        let mut total = 0;
        for galaxy_a in &self.galaxies {
            for galaxy_b in &self.galaxies {
                //  Don't recalculate distance for pair
                if galaxy_a >= galaxy_b {
                    continue;
                }

                let rows = (galaxy_a.0 + 1..galaxy_b.0).chain(galaxy_b.0 + 1..galaxy_a.0);
                let cols = (galaxy_a.1 + 1..galaxy_b.1).chain(galaxy_b.1 + 1..galaxy_a.1);
                let starting_moves =
                    (galaxy_a.0 != galaxy_b.0) as usize + (galaxy_a.1 != galaxy_b.1) as usize;
                let distance: usize = starting_moves
                    + rows
                        .map(|row| {
                            if self.galaxy_rows.contains(&row) {
                                1
                            } else {
                                factor
                            }
                        })
                        .sum::<usize>()
                    + cols
                        .map(|col| {
                            if self.galaxy_cols.contains(&col) {
                                1
                            } else {
                                factor
                            }
                        })
                        .sum::<usize>();
                total += distance
            }
        }
        total
    }
}

fn as_xy(span: Span) -> (usize, usize) {
    // Positions are 1-indexed
    let x = span.get_column() - 1;
    let y = span.location_line() as usize - 1;
    (x, y)
}

fn parse_grid(input: Span) -> IResult<Span, Grid> {
    let (input, galaxies) = fold_many1(
        preceded(take_until("#"), tag("#").map(|span| as_xy(span))),
        HashSet::new,
        |mut acc, galaxy| {
            acc.insert(galaxy);
            acc
        },
    )(input)?;

    let galaxy_rows = galaxies.iter().map(|galaxy| galaxy.0).collect();
    let galaxy_cols = galaxies.iter().map(|galaxy| galaxy.1).collect();

    Ok((
        input,
        Grid {
            galaxies,
            galaxy_rows,
            galaxy_cols,
        },
    ))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, grid) = parse_grid(Span::new(input)).expect("should be parsable");
    Some(grid.distances(2))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, grid) = parse_grid(Span::new(input)).expect("should be parsable");
    Some(grid.distances(1_000_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82210));
    }
}
