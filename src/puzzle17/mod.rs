use std::ops::RangeInclusive;
use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;
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

    fn left_by(&self, by: i16) -> Pt { Pt { x: self.x - by, y: self.y } }
    fn left(&self) -> Pt { self.left_by(1) }
    fn right_by(&self, by: i16) -> Pt {
        Pt { x: self.x + by, y: self.y }
    }
    fn right(&self) -> Pt { self.right_by(1) }

    fn up_by(&self, by: i16) -> Pt { Pt { x: self.x, y: self.y - by } }
    fn up(&self) -> Pt { self.up_by(1) }

    fn down_by(&self, by: i16) -> Pt {
        Pt { x: self.x, y: self.y + by }
    }
    fn down(&self) -> Pt { self.down_by(1) }
}

// A tile that water went through
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Water {
    Flowing, // |
    Settled  // ~
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Soil {
    Sand(Option<Water>),
    Clay
}

impl Soil {
    fn blocks_flow(&self) -> bool {
        match self {
            Soil::Sand(None) | Soil::Sand(Some(Water::Flowing)) => false,
            _ => true
        }
    }
}

impl Display for Soil {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Ok(
            match self {
                Soil::Sand(None) => write!(f, ".")?,
                Soil::Sand(Some(Water::Flowing)) => write!(f, "|")?,
                Soil::Sand(Some(Water::Settled)) => write!(f, "~")?,
                Soil::Clay => write!(f, "#")?,
            }
        )
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum WaterFlow {
    Closed(RangeInclusive<Pt>),
    Opened(RangeInclusive<Pt>) // last Pt allows flowing downwards
}

#[derive(Debug, Clone)]
struct Ground {
    min_pos: Pt,
    max_pos: Pt,
    soil: HashMap<Pt, Soil>
}

impl Ground {
    fn new(clay: &Vec<ClayRange>) -> Self {

        let mut min_pos = Pt::max();
        let mut max_pos = Pt::min();
        let mut soil = HashMap::new();

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
                    soil.insert(pt, Soil::Clay);
                }
            }
        }

        Ground { min_pos, max_pos, soil }
    }

    fn with_flow(&self, water: HashMap<Pt, Water>) -> Self {
        let mut soil = self.soil.clone();
        for (pt,w) in water {
            soil.insert(pt, Soil::Sand(Some(w)));
        }
        Ground { min_pos: self.min_pos, max_pos: self.max_pos, soil }
    }

    fn with_flow_outcome(&self, outcome: &FlowOutcome) -> Self {
        let mut soil = self.soil.clone();
        match outcome {
            FlowOutcome::CannotSettle(pts) => {
                let x = pts.start().x;
                for y in pts.start().y..=pts.end().y {
                    soil.insert(Pt::new(x, y), Soil::Sand(Some(Water::Flowing)));
                }
                Ground { min_pos: self.min_pos, max_pos: self.max_pos, soil }
            },
            FlowOutcome::Settled(down, settled, flowing, _) => {
                for y in down.start().y..=down.end().y {
                    soil.insert(Pt::new(down.start().x, y), Soil::Sand(Some(Water::Flowing)));
                }
                for s in settled {
                    let y = s.start().y;
                    for x in s.start().x..=s.end().x {
                        soil.insert(Pt::new(x, y), Soil::Sand(Some(Water::Settled)));
                    }
                }
                for x in flowing.start().x..=flowing.end().x {
                    soil.insert(Pt::new(x, flowing.start().y), Soil::Sand(Some(Water::Flowing)));
                }
                Ground { min_pos: self.min_pos, max_pos: self.max_pos, soil }
            },
            FlowOutcome::Visited => self.clone()
        }
    }

    fn out_of_bounds(&self, pt: &Pt) -> bool {
        pt.y < self.min_pos.y || pt.y > self.max_pos.y
    }

    fn soil_at(&self, pos: &Pt) -> Soil {
        match self.soil.get(pos) {
            Some(soil) => *soil,
            None => Soil::Sand(None)
        }
    }

    fn flow_down(&self, start: &Pt) -> WaterFlow {
        let mut current = *start;
        loop {
            let down = current.down();
            if self.out_of_bounds(&down) {
                break WaterFlow::Opened(RangeInclusive::new(*start, current))
            } else if self.soil_at(&down).blocks_flow() {
                break WaterFlow::Closed(RangeInclusive::new(*start, current))
            } else {
                current = down;
            }
        }
    }

    fn flow_left_right<F>(&self, start: &Pt, f: F) -> WaterFlow
      where F: Fn(&Pt) -> Pt {
        let mut current = *start;
        loop {
            let down = current.down();
            if !self.soil_at(&down).blocks_flow() {
                break WaterFlow::Opened(RangeInclusive::new(*start, current));
            } else {
                let next = f(&current);
                if self.soil_at(&next).blocks_flow() {
                    break WaterFlow::Closed(RangeInclusive::new(*start, current))
                }
            }
            current = f(&current);
        }
    }

    fn flow_left(&self, start: &Pt) -> WaterFlow {
        self.flow_left_right(start, |current| current.left())
    }

    fn flow_right(&self, start: &Pt) -> WaterFlow {
        self.flow_left_right(start, |current| current.right())
    }

    fn wet_soil(&self) -> usize {
        self.soil
            .iter()
            .filter(|(pt, _)| {
                !self.out_of_bounds(pt)
            })
            .filter(|(_, soil)| {
                match soil {
                    Soil::Sand(Some(_)) => true,
                    _ => false
                }
            })
            .count()
    }

    fn retained(&self) -> usize {
        self.soil
            .iter()
            .filter(|(pt, _)| {
                !self.out_of_bounds(pt)
            })
            .filter(|(_, soil)| {
                match soil {
                    Soil::Sand(Some(Water::Settled)) => true,
                    _ => false
                }
            })
            .count()
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


// a flow outcome tells us which new tiles are wet and new flows to consider
#[derive(PartialEq, Eq, Debug, Clone)]
enum FlowOutcome {
    CannotSettle(RangeInclusive<Pt>),
    // Settled flow has several ranges of settled water, at most one range of flowing water and 1 or 2 new flows
    Settled(RangeInclusive<Pt>, Vec<RangeInclusive<Pt>>, RangeInclusive<Pt>, Vec<Pt>),
    Visited // some other flow has already computed this one
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Flow {
    origin: Pt
}

impl Flow {
    fn new(pt: &Pt) -> Self { Flow { origin: *pt } }
    fn solve(&self, ground: &Ground) -> FlowOutcome {
        let current = self.origin;
        if let Soil::Sand(Some(_)) = ground.soil_at(&self.origin.down()) {
            return FlowOutcome::Visited;
        }
        // Ground immediately below shouldn't already be clay or settled water.
        assert!(!ground.soil_at(&current.down()).blocks_flow(), format!("{} {:?}", ground, self.origin));

        match ground.flow_down(&self.origin) {
            WaterFlow::Opened(pts) => FlowOutcome::CannotSettle(pts),
            WaterFlow::Closed(range) => {

                // special case if we've hit some flowing water which means we've already visited this
                if let Soil::Sand(Some(Water::Flowing)) = ground.soil_at(range.end()) {
                    return FlowOutcome::Settled(range.clone(), Vec::new(), RangeInclusive::new(range.end().clone(), range.end().clone()), Vec::new());
                }

                let mut settled = Vec::new();
                let mut g = ground.clone();
                let mut end = *range.end();
                loop {
                    let left = g.flow_left(&end);
                    let right = g.flow_right(&end);
                    match (left.clone(), right.clone()) {
                        (WaterFlow::Opened(_), _) | (_, WaterFlow::Opened(_)) => {
                            let (left_range, left_pt) = match left {
                                WaterFlow::Closed(r) => (r.clone(), None),
                                WaterFlow::Opened(r) => (r.clone(), Some(r.end().clone()))
                            };
                            let (right_range, right_pt) = match right {
                                WaterFlow::Closed(r) => (r.clone(), None),
                                WaterFlow::Opened(r) => (r.clone(), Some(r.end().clone()))
                            };

                            let new_flows = left_pt.into_iter().chain(right_pt.into_iter()).collect::<Vec<_>>();

                            break FlowOutcome::Settled(range, settled.clone(), RangeInclusive::new(left_range.end().clone(), right_range.end().clone()), new_flows)
                        }

                        (WaterFlow::Closed(left_range), WaterFlow::Closed(right_range)) => {
                            let s = RangeInclusive::new(left_range.end().clone(), right_range.end().clone());
                            settled.push(s.clone());

                            let mut water = HashMap::new();
                            for x in s.start().x..=s.end().x {
                                water.insert(Pt::new(x, end.y), Water::Settled);
                            }

                            g = g.with_flow(water);

                            end = end.up();
                        }
                    }
                }
            }
        }
    }

    fn solve_r(&self, ground: &Ground) -> Ground {
        let outcome = self.solve(ground);
        match outcome.clone() {
            o@FlowOutcome::CannotSettle(_) => ground.with_flow_outcome(&o),
            FlowOutcome::Settled(_,_,_,flows) => {
                let     g = ground.with_flow_outcome(&outcome);
                flows.iter().fold(g, |gr, pt| {
                    Flow::new(pt).solve_r(&gr)
                })
            },
            FlowOutcome::Visited => ground.clone()
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
        let ground = Ground::new(&self.ranges);
        let flow = Flow::new(&Pt::new(500,ground.min_pos.y-1));
        let solved = flow.solve_r(&ground);
        solved.wet_soil().to_string()
    }

    fn part2(&self) -> String {
        let ground = Ground::new(&self.ranges);
        let flow = Flow::new(&Pt::new(500,ground.min_pos.y-1));
        let solved = flow.solve_r(&ground);
        solved.retained().to_string()
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
        assert_eq!(Soil::Sand(None), ground.soil_at(&Pt::new(1, 1)));
    }

    const EXPECTED: &str = r#"......+.......
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

    #[test]
    fn test_flow_down() {
        let ground = Ground::new(&parse(EXAMPLE));
        let flow = ground.flow_down(&Pt::new(500,0));
        assert_eq!(WaterFlow::Closed(RangeInclusive::new(Pt::new(500,0), Pt::new(500,6))), flow);
        let flow = ground.flow_down(&Pt::new(505,0));
        assert_eq!(WaterFlow::Opened(RangeInclusive::new(Pt::new(505,0), Pt::new(505,13))), flow);
    }

    #[test]
    fn test_flow_left() {
        let ground = Ground::new(&parse(EXAMPLE));
        let flow = ground.flow_left(&Pt::new(500,6));
        assert_eq!(WaterFlow::Closed(RangeInclusive::new(Pt::new(500,6), Pt::new(496,6))), flow);
        let flow = ground.flow_left(&Pt::new(496,6));
        assert_eq!(WaterFlow::Closed(RangeInclusive::new(Pt::new(496,6), Pt::new(496,6))), flow);
    }

    #[test]
    fn test_flow_right() {
        let ground = Ground::new(&parse(EXAMPLE));
        let flow = ground.flow_right(&Pt::new(500,6));
        assert_eq!(WaterFlow::Closed(RangeInclusive::new(Pt::new(500,6), Pt::new(500,6))), flow);
        let flow = ground.flow_right(&Pt::new(496,6));
        assert_eq!(WaterFlow::Closed(RangeInclusive::new(Pt::new(496,6), Pt::new(500,6))), flow);
    }

    // .+.    .+.
    // #.# => #|#
    // #.#    #|#
    #[test]
    fn test_flow1() {
        let ground = Ground::new(&parse("x=0, y=1..2\nx=2, y=1..2"));
        let flow = Flow { origin: Pt::new(1, 0) };
        let outcome = flow.solve(&ground);
        assert_eq!(FlowOutcome::CannotSettle(RangeInclusive::new(Pt::new(1,0), Pt::new(1,2))), outcome);
    }

    // ..+..    ..+..
    // #...# => #|||#
    // ..#..    ..#..
    #[test]
    fn test_flow2() {
        let ground = Ground::new(&parse("x=1, y=1..1\nx=3, y=2..2\nx=5, y=1..1"));
        let flow = Flow { origin: Pt::new(3, 0) };
        let outcome = flow.solve(&ground);
        let expected = FlowOutcome::Settled(
            RangeInclusive::new(Pt::new(3,0), Pt::new(3,1)),
            Vec::new(),
            RangeInclusive::new(Pt::new(2,1), Pt::new(4,1)),
            vec![Pt::new(2, 1), Pt::new(4, 1)]
        );
        assert_eq!(expected, outcome);
    }

    // ..+..    ..+..
    // .....    |||||
    // .#.#. => .#~#.
    // ..#..    ..#..
    #[test]
    fn test_flow3() {
        let ground = Ground::new(&parse("x=0, y=0..0\nx=2, y=2..2\nx=4, y=2..2\nx=3, y=3..3"));
        let flow = Flow { origin: Pt::new(3, 0) };
        let outcome = flow.solve(&ground);
        let expected = FlowOutcome::Settled(
            RangeInclusive::new(Pt::new(3,0), Pt::new(3,2)),
            vec![RangeInclusive::new(Pt::new(3,2), Pt::new(3,2))],
            RangeInclusive::new(Pt::new(1,1), Pt::new(5,1)),
            vec![Pt::new(1, 1), Pt::new(5, 1)]
        );
        assert_eq!(expected, outcome);
    }

    #[test]
    fn test_flow_example() {
        let ground = Ground::new(&parse(EXAMPLE));
        let flow = Flow { origin: Pt::new(500,0) };
        let outcome = flow.solve(&ground);
        println!("{}", ground.with_flow_outcome(&outcome));

        println!("{}", flow.solve_r(&ground));
    }
}