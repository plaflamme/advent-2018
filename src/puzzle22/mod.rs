use std::str::FromStr;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
struct Pt {
    y: u32, // sort by y first
    x: u32
}

impl Pt {
    fn new(x: u32, y: u32) -> Self {
        Pt { y, x }
    }
}

impl FromStr for Pt {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s.split(',').collect::<Vec<_>>();
        let x = u32::from_str(coords[0])?;
        let y = u32::from_str(coords[0])?;
        Ok(Pt::new(x,y))
    }
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    let lines = input.lines().collect::<Vec<_>>();
    let depth = u32::from_str(lines[0].split_ascii_whitespace().last().unwrap()).unwrap();
    let target = Pt::from_str(lines[1].split_ascii_whitespace().last().unwrap()).unwrap();
    Box::new(Puzzle22 { depth, target })
}

struct Puzzle22 {
    depth: u32,
    target: Pt
}

impl crate::Puzzle for Puzzle22 {
    fn part1(&self) -> String {
        unimplemented!()
    }

    fn part2(&self) -> String {
        unimplemented!()
    }
}