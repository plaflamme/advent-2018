use std::str::FromStr;
use regex::Regex;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Pt {
    x: i32,
    y: i32,
    z: i32
}

impl Pt {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Pt { x, y, z }
    }
}

impl FromStr for Pt {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(',').collect();
        Ok(
            Pt {
                x: i32::from_str(parts[0])?,
                y: i32::from_str(parts[1])?,
                z: i32::from_str(parts[2])?,
            }
        )
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Nanobot {
    pos: Pt,
    signal_radius: u32
}

impl FromStr for Nanobot {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new("^pos=<(.+)>, r=(\\d+)$").unwrap();
        let caps = re.captures(s).expect(&format!("unmatched input: {}", s));

        Ok(
            Nanobot {
                pos: Pt::from_str(&caps[1])?,
                signal_radius: u32::from_str(&caps[2])?,
            }
        )
    }
}

fn parse(input: &str) -> Vec<Nanobot> {
    input.lines().map(|line| Nanobot::from_str(line).unwrap() ).collect()
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle23 { bots: parse(&input) })
}

struct Puzzle23 {
    bots: Vec<Nanobot>
}

impl crate::Puzzle for Puzzle23 {
    fn part1(&self) -> String {
        unimplemented!()
    }

    fn part2(&self) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Puzzle;

    #[test]
    fn test_parse() {
        let bots = parse("pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1");

        assert_eq!(9, bots.len());
        assert_eq!(Nanobot { pos: Pt::new(0,0,0), signal_radius: 4 }, bots[0]);
        assert_eq!(Nanobot { pos: Pt::new(1,0,0), signal_radius: 1 }, bots[1]);
        assert_eq!(Nanobot { pos: Pt::new(1,3,1), signal_radius: 1 }, bots[bots.len()-1]);
    }

}