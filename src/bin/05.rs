use std::{
    cmp::{max, min},
    ops::Range,
};

use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
// use tracing::info;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Mapping {
    source: Range<u64>,
    destination: Range<u64>,
}

impl Mapping {
    fn project_range(&self, source_range: Range<u64>) -> Range<u64> {
        let start = self.destination.start + (source_range.start - self.source.start);
        let end = self.destination.start + (source_range.end - self.source.start);
        start..end
    }

    fn remainder(&self, b: &Range<u64>) -> Vec<Range<u64>> {
        let intersection_range = self.intersection(b);

        let mut ranges = Vec::new();
        if intersection_range.start > b.start {
            ranges.push(b.start..intersection_range.start)
        }
        if intersection_range.end < b.end {
            ranges.push(intersection_range.end..b.end)
        }
        ranges
    }

    fn intersection(&self, b: &Range<u64>) -> Range<u64> {
        max(self.source.start, b.start)..min(self.source.end, b.end)
    }

    fn is_intersecting(&self, b: &Range<u64>) -> bool {
        (b.start <= self.source.end) && (self.source.start <= b.end)
    }
}

#[derive(Debug)]
struct ResourceMap(Vec<Mapping>);

impl ResourceMap {
    fn translate_number(&self, source_num: u64) -> u64 {
        let translation = self
            .0
            .iter()
            .find(|mapping| mapping.source.contains(&source_num));

        match translation {
            Some(mapping) => mapping.destination.start + (source_num - mapping.source.start),
            None => source_num,
        }
    }

    fn translate_range(&self, source_range: Range<u64>) -> Vec<Range<u64>> {
        let bucket = self
            .0
            .iter()
            .fold(Bucket::new(source_range), |mut bucket, mapping| {
                let mut remaining = Vec::new();
                bucket.remaining.into_iter().for_each(|range| {
                    if mapping.is_intersecting(&range) {
                        bucket
                            .translated
                            .push(mapping.project_range(mapping.intersection(&range)));
                        remaining.append(&mut mapping.remainder(&range));
                    } else {
                        remaining.push(range);
                    }
                });
                bucket.remaining = remaining;
                bucket
            });

        bucket.concat()
    }
}

struct Bucket {
    translated: Vec<Range<u64>>,
    remaining: Vec<Range<u64>>,
}

impl Bucket {
    fn new(initial: Range<u64>) -> Self {
        Bucket {
            translated: Vec::new(),
            remaining: vec![initial],
        }
    }

    fn concat(&self) -> Vec<Range<u64>> {
        [self.translated.to_owned(), self.remaining.to_owned()].concat()
    }
}

fn line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (destination, source, size)) = tuple((
        complete::u64,
        complete::u64.preceded_by(tag(" ")),
        complete::u64.preceded_by(tag(" ")),
    ))(input)?;

    Ok((
        input,
        (source..source + size, destination..destination + size),
    ))
}

fn resource_map(input: &str) -> IResult<&str, ResourceMap> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(
            many1(
                line_ending
                    .precedes(line)
                    .map(|(source, destination)| Mapping {
                        source,
                        destination,
                    }),
            )
            .map(ResourceMap),
        )
        .parse(input)
}

fn parse_maps(input: &str) -> IResult<&str, Vec<ResourceMap>> {
    many1(resource_map)(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    tag("seeds: ")
        .precedes(separated_list1(space1, complete::u64))
        .parse(input)
}

fn parse_seed_ranges(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    tag("seeds: ")
        .precedes(separated_list1(
            space1,
            separated_pair(complete::u64, space1, complete::u64)
                .map(|(start, size)| start..start + size),
        ))
        .parse(input)
}

#[tracing::instrument(skip(input))]
pub fn part_one(input: &str) -> Option<u64> {
    let (input, seeds) = parse_seeds(input).expect("should be parsable");
    let (_, maps) = parse_maps(input).expect("should be parsable");
    // info!(?seeds);
    // info!(?maps);

    seeds
        .iter()
        .map(|seed| {
            maps.iter()
                .fold(*seed, |location, map| map.translate_number(location))
        })
        .min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (input, seed_ranges) = parse_seed_ranges(input).expect("should be parsable");
    let (_, maps) = parse_maps(input).expect("should be parsable");
    // info!(?seed_ranges);

    let mut location_ranges = maps.iter().fold(seed_ranges, |ranges, map| {
        ranges
            .into_iter()
            .flat_map(|range| map.translate_range(range))
            .collect()
    });

    location_ranges.sort_by(|a, b| a.start.cmp(&b.start));

    Some(location_ranges[0].start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test_log::test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
