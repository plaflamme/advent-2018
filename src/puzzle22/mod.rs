use std::str::FromStr;
use std::collections::HashMap;

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
        let y = u32::from_str(coords[1])?;
        Ok(Pt::new(x,y))
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Type {
    Rocky,
    Narrow,
    Wet
}

struct Analyzer {
    depth: u32,
    target: Pt,
    geologic_indices: HashMap<Pt, u32>
}

impl Analyzer {

    fn new(depth: u32, target: Pt) -> Self {
        Analyzer { depth, target, geologic_indices: HashMap::new() }
    }

    fn geologic_index(&mut self, pt: Pt) -> u32 {
        match self.geologic_indices.get(&pt) {
            Some(index) => *index,
            None => {
                match (pt.x, pt.y) {
                    (0,0) => 0,
                    (x,y) if x == self.target.x && y == self.target.y => 0,
                    (x, 0) => x * 16807,
                    (0,y) => y * 48271,
                    (x,y) => {
                        let index = self.erosion_level(Pt::new(x-1, y)) * self.erosion_level(Pt::new(x, y-1));
                        self.geologic_indices.insert(pt, index);
                        index
                    }
                }
            }
        }
    }

    fn erosion_level(&mut self, pt: Pt) -> u32 {
        (self.geologic_index(pt) + self.depth) % 20183
    }

    fn region_type(&mut self, pt: Pt) -> Type {
        match self.erosion_level(pt) % 3 {
            0 => Type::Rocky,
            1 => Type::Wet,
            2 => Type::Narrow,
            _ => panic!("modulo doesn't work!")
        }
    }

    fn risk(&mut self, pt: Pt) -> u32 {
        match self.region_type(pt) {
            Type::Rocky => 0,
            Type::Wet => 1,
            Type::Narrow => 2
        }
    }

    fn rect_risk(&mut self, from: Pt, to: Pt) -> u32 {
        let mut sum = 0;
        for x in from.x..=to.x {
            for y in from.y..=to.y {
                sum += self.risk(Pt::new(x,y));
            }
        }
        sum
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
        let mut analyzer = Analyzer::new(self.depth, self.target);
        analyzer.rect_risk(Pt::new(0,0), self.target).to_string()
    }

    fn part2(&self) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Puzzle;

    #[test]
    fn test_example() {
        let puzzle = Puzzle22 { depth: 510, target: Pt::new(10, 10) };

        assert_eq!("114", puzzle.part1());
    }
}