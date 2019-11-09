use std::str::FromStr;
use std::collections::HashMap;
use pathfinding::directed::dijkstra;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
struct Pt {
    y: u32, // sort by y first
    x: u32
}

impl Pt {
    fn new(x: u32, y: u32) -> Self {
        Pt { y, x }
    }

    fn neighbours(&self) -> Vec<Pt> {
        let one = if self.x > 0 { Some(Pt::new(self.x - 1, self.y)) } else { None };
        let two = if self.y > 0 { Some(Pt::new(self.x, self.y-1)) } else { None };

        one.iter()
            .chain(two.iter())
            .cloned()
            .chain(vec![Pt::new(self.x + 1, self.y), Pt::new(self.x, self.y + 1)])
            .collect()
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

impl Type {
    fn accepts(&self, tool: &Option<Tool>) -> bool {
        match (self, tool) {
            (Type::Rocky, Some(_)) => true,
            (Type::Rocky, None) => false,

            (Type::Wet, Some(Tool::Gear)) => true,
            (Type::Wet, None) => true,
            (Type::Wet, Some(Tool::Torch)) => false,


            (Type::Narrow, Some(Tool::Torch)) => true,
            (Type::Narrow, None) => true,
            (Type::Narrow, Some(Tool::Gear)) => false,
        }
    }
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

#[derive(PartialEq, Eq, Hash, Clone)]
enum Tool {
    Gear,
    Torch
}

impl Tool {
    fn all() -> Vec<Option<Tool>> {
        vec![
            None,
            Some(Tool::Gear),
            Some(Tool::Torch)
        ]
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    at: Pt,
    holding: Option<Tool>
}

impl State {

    fn new() -> Self {
        State { at: Pt::new(0,0), holding: Some(Tool::Torch) }
    }

    fn solve(analyzer: &mut Analyzer) -> u32 {
        let target = State { at: analyzer.target, holding: Some(Tool::Torch) };
        dijkstra::dijkstra(
            &State::new(),
            |state| { state.neighbours(analyzer) },
            |state| { state == &target }
        ).map(|(_, cost)| cost).unwrap()
    }

    fn neighbours(&self, analyzer: &mut Analyzer) -> Vec<(State, u32)> {
        // all neighbours that accept what we're holding (cost 1 minute)
        //   as well as this same pt but using a different tool (cost 7 minutes)
        self.at.neighbours()
            .iter()
            .filter_map(|other| {
                let other_type = &analyzer.region_type(*other);
                if !other_type.accepts(&self.holding) { None } else {
                    Some( (State { at: *other, holding: self.holding.clone() }, 1) )
                }
            })
            .chain(
                Tool::all()
                    .iter()
                    .filter(|&tool| tool != &self.holding)
                    .map(|tool| (State { at: self.at, holding: tool.clone() }, 7))
                    .collect::<Vec<_>>()
            )
            .collect()
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
        let mut analyzer = Analyzer::new(self.depth, self.target);
        State::solve(&mut analyzer).to_string()
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