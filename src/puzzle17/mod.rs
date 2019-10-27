use std::ops::{Range, RangeInclusive};
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct ClayRange { x: RangeInclusive<u16>, y: RangeInclusive<u16> }

impl FromStr for ClayRange {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(.)=(\d+), .=(\d+)\.\.(\d+)$").unwrap();
        let caps = re.captures(s).expect("invalid line");

        let not_range = u16::from_str(&caps[2])?;
        let first_range = not_range..=not_range;

        let start = u16::from_str(&caps[3])?;
        let end = u16::from_str(&caps[4])?;
        let range = start..=end;

        match &caps[1] {
            "x" => Ok(ClayRange { x: first_range, y: range }),
            "y" => Ok(ClayRange { x: range, y: first_range }),
            _ => panic!("invalid coord")
        }
    }
}

fn parse(input: &str) -> Vec<ClayRange> {
    input.lines()
        .map(|line| {
            ClayRange::from_str(line).expect(&format!("Unparseable line {}", line))
        })
        .collect()
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle17 { ranges: parse(&input) })
}

struct Puzzle17 {
    ranges: Vec<ClayRange>
}

impl crate::Puzzle for Puzzle17 {
    fn part1(&self) -> String {
        unimplemented!()
    }

    fn part2(&self) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"#;

    #[test]
    fn test_parse() {
        let parsed = parse(EXAMPLE);
        let expected = vec![
            ClayRange { x: 495..=495, y: 2..=7 },
            ClayRange { x: 495..=501, y: 7..=7 },
            ClayRange { x: 501..=501, y: 3..=7 },
            ClayRange { x: 498..=498, y: 2..=4 },
            ClayRange { x: 506..=506, y: 1..=2 },
            ClayRange { x: 498..=498, y: 10..=13 },
            ClayRange { x: 504..=504, y: 10..=13 },
            ClayRange { x: 498..=504, y: 13..=13 },
        ];
        assert_eq!(expected, parsed);
    }
}