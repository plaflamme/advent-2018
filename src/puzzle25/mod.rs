use std::str::FromStr;
use std::num::ParseIntError;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pt {
    x: i32,
    y: i32,
    z: i32,
    t: i32
}

impl Pt {
    fn new(x: i32, y: i32, z: i32, t: i32) -> Self {
        Pt {x,y,z,t}
    }

    fn distance(&self, other: &Pt) -> u32 {
        ((self.x - other.x).abs() +
            (self.y - other.y).abs() +
            (self.z - other.z).abs() +
            (self.t - other.t).abs()) as u32
    }
}

impl FromStr for Pt {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<_>>();
        Ok(
            Pt::new(
                i32::from_str(parts[0])?,
                i32::from_str(parts[1])?,
                i32::from_str(parts[2])?,
                i32::from_str(parts[3])?,
            )
        )
    }
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle25::from_str(&input).expect("invalid input"))
}

struct Puzzle25 {
    pts: Vec<Pt>
}

impl FromStr for Puzzle25 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            Puzzle25 {
                pts: Result::from(s.lines().map(|line| Pt::from_str(line)).collect())?
            }
        )
    }

}

impl crate::Puzzle for Puzzle25 {
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

    const EX1: &str = "0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0";

    #[test]
    fn test_parse() {
        let pzl = Puzzle25::from_str(EX1).unwrap();
        assert_eq!(8, pzl.pts.len());
        assert_eq!(Some(&Pt::new(0,0,0,0)), pzl.pts.first());
        assert_eq!(Some(&Pt::new(12,0,0,0)), pzl.pts.last());
    }
}