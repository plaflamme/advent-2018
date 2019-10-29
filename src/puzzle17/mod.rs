use std::ops::{Range, RangeInclusive};
use regex::Regex;
use std::str::FromStr;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, PartialEq, Eq, Clone)]
struct ClayRange { x: RangeInclusive<i16>, y: RangeInclusive<i16> }

impl FromStr for ClayRange {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(.)=(\d+), .=(\d+)\.\.(\d+)$").unwrap();
        let caps = re.captures(s).expect("invalid line");

        let not_range = i16::from_str(&caps[2])?;
        let first_range = not_range..=not_range;

        let start = i16::from_str(&caps[3])?;
        let end = i16::from_str(&caps[4])?;
        let range = start..=end;

        match &caps[1] {
            "x" => Ok(ClayRange { x: first_range, y: range }),
            "y" => Ok(ClayRange { x: range, y: first_range }),
            _ => panic!("invalid coord")
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy, Clone)]
struct Pt {
    x: i16,
    y: i16
}

impl Pt {

    fn new(x: i16, y: i16) -> Pt { Pt {x, y} }

    fn max() -> Pt { Pt::new(std::i16::MAX, std::i16::MAX) }
    fn min() -> Pt { Pt::new(std::i16::MIN, std::i16::MIN) }

    fn left(&self, by: i16) -> Pt { Pt { x: self.x - by, y: self.y } }
    fn right(&self, by: i16) -> Pt {
        Pt { x: self.x + by, y: self.y }
    }
    fn top(&self, by: i16) -> Pt {
        Pt { x: self.x, y: self.y - by }
    }
    fn down(&self, by: i16) -> Pt {
        Pt { x: self.x, y: self.y + by }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Soil {
    Sand,
    Clay
}

impl Display for Soil {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Ok(
            match self {
                Soil::Sand => write!(f, ".")?,
                Soil::Clay => write!(f, "#")?,
            }
        )
    }
}

struct Ground {
    min_pos: Pt,
    max_pos: Pt,
    clay_pos: HashSet<Pt>
}

impl Ground {
    fn new(clay: &Vec<ClayRange>) -> Self {

        let mut min_pos = Pt::max();
        let mut max_pos = Pt::min();
        let mut clay_pos = HashSet::new();

        for range in clay {
            for x in range.x.clone() {
                for y in range.y.clone() {
                    let pt = Pt::new(x,y);

                    if pt.x < min_pos.x {
                        min_pos.x = pt.x
                    }
                    if pt.y < min_pos.y {
                        min_pos.y = pt.y
                    }

                    if pt.x > max_pos.x {
                        max_pos.x = pt.x
                    }
                    if pt.y > max_pos.y {
                        max_pos.y = pt.y
                    }
                    clay_pos.insert(pt);
                }
            }
        }

        Ground { min_pos, max_pos, clay_pos }
    }

    fn soil_at(&self, pos: &Pt) -> Soil {
        if self.clay_pos.contains(pos) { Soil::Clay } else { Soil::Sand }
    }
}

impl Display for Ground {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let source = Pt::new(500,self.min_pos.y-1);
        for y in self.min_pos.y-1..=self.max_pos.y+1 {
            for x in self.min_pos.x-1..=self.max_pos.x+1 {
                let pt = Pt::new(x, y);
                if pt == source {
                    write!(f, "+")?;
                } else {
                    write!(f, "{}", self.soil_at(&pt))?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
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
        let ground = Ground::new(&self.ranges);
        println!("{}", ground);
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

    #[test]
    fn test_ground() {
        let ground = Ground::new(&parse(EXAMPLE));

        assert_eq!(Pt::new(495,1), ground.min_pos);
        assert_eq!(Pt::new(506,13), ground.max_pos);

        assert_eq!(Soil::Clay, ground.soil_at(&Pt::new(495, 7)));
        assert_eq!(Soil::Clay, ground.soil_at(&Pt::new(501, 3)));
        assert_eq!(Soil::Clay, ground.soil_at(&Pt::new(501, 7)));
        assert_eq!(Soil::Sand, ground.soil_at(&Pt::new(1, 1)));
    }

    const EXPECTED: &str = r#"..............
............#.
.#..#.......#.
.#..#..#......
.#..#..#......
.#.....#......
.#.....#......
.#######......
..............
..............
....#.....#...
....#.....#...
....#.....#...
....#######...
..............
"#;

    #[test]
    fn test_ground_display() {
        let ground = Ground::new(&parse(EXAMPLE));
        assert_eq!(EXPECTED, format!("{}", ground));
    }
}