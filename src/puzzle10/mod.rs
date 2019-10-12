use std::str::FromStr;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct Pt {
    x: i32,
    y: i32
}

#[derive(Debug, PartialEq, Eq)]
struct Velocity {
    x: i32,
    y: i32
}

#[derive(Debug, PartialEq, Eq)]
struct Char {
    pt: Pt,
    velocity: Velocity
}

impl Char {
    fn step(&mut self) {
        self.pt.x += self.velocity.x;
        self.pt.y += self.velocity.y;
    }
}

impl FromStr for Char {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>$").unwrap();
        let caps = re.captures(s).expect(&format!("invalid input {}", s));
        let pt = Pt { x: i32::from_str(&caps[1])?, y: i32::from_str(&caps[2])? };
        let velocity = Velocity { x: i32::from_str(&caps[3])?, y: i32::from_str(&caps[4])? };
        Ok(Char { pt, velocity })
    }
}

fn parse(input: String) -> Puzzle10 {
    let chars = input.lines().map(|line| Char::from_str(line).expect("invalid line")).collect::<Vec<_>>();
    Puzzle10 { chars }
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(parse(input) )
}

struct Puzzle10 {
    chars: Vec<Char>
}

impl crate::Puzzle for Puzzle10 {
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

    const EXAMPLE: &'static str = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
";

    #[test]
    fn parser() {
        let puzzle = parse(EXAMPLE.to_string());
        assert_eq!(31, puzzle.chars.len());
        assert_eq!(Some(&Char{ pt: Pt{x:9,y:1}, velocity: Velocity{x:0,y:2}}), puzzle.chars.iter().next());
        assert_eq!(Some(&Char{ pt: Pt{x:-3,y:6}, velocity: Velocity{x:2,y:-1}}), puzzle.chars.iter().rev().next());
    }

    #[test]
    fn step() {
        let mut c = Char{ pt: Pt{x:-3,y:6}, velocity: Velocity{x:2,y:-1}};
        c.step();
        assert_eq!(Char{ pt: Pt { x: -1 , y: 5 }, velocity: Velocity { x: 2, y: -1 } }, c);
    }

    #[test]
    fn part1() {
        unimplemented!()
    }

    #[test]
    fn part2() {
        unimplemented!()
    }
}