advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let total = input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|character| character.to_digit(10));

            let first = digits.next().expect("should be a number");

            match digits.last() {
                Some(digit) => {
                    format!("{first}{digit}")
                }
                None => {
                    format!("{first}{first}")
                }
            }
            .parse::<u32>()
            .expect("should be a number")
        })
        .sum::<u32>();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total = input
        .lines()
        .map(|line| {
            let mut digits = (0..line.len()).filter_map(|index| {
                let substring = &line[index..];
                let value = if substring.starts_with("one") {
                    '1'
                } else if substring.starts_with("two") {
                    '2'
                } else if substring.starts_with("three") {
                    '3'
                } else if substring.starts_with("four") {
                    '4'
                } else if substring.starts_with("five") {
                    '5'
                } else if substring.starts_with("six") {
                    '6'
                } else if substring.starts_with("seven") {
                    '7'
                } else if substring.starts_with("eight") {
                    '8'
                } else if substring.starts_with("nine") {
                    '9'
                } else {
                    substring.chars().next().unwrap()
                };

                value.to_digit(10)
            });

            let first = digits.next().expect("should be a number");

            match digits.last() {
                Some(digit) => {
                    format!("{first}{digit}")
                }
                None => {
                    format!("{first}{first}")
                }
            }
            .parse::<u32>()
            .expect("should be a number")
        })
        .sum::<u32>();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(360));
    }
}
