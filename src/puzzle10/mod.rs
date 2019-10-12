use std::str::FromStr;
use regex::Regex;
use std::cmp::{min, max};
use std::fmt::{Display, Formatter, Error};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pt {
    x: i32,
    y: i32
}

impl Pt {
    fn new(x: i32, y: i32) -> Pt {
        Pt{x,y}
    }
    fn max() -> Pt { Pt::new(std::i32::MAX, std::i32::MAX) }
    fn min() -> Pt { Pt::new(std::i32::MIN, std::i32::MIN) }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Velocity {
    x: i32,
    y: i32
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Char {
    pt: Pt,
    velocity: Velocity
}

impl Char {
    fn step(&mut self) {
        self.pt.x += self.velocity.x;
        self.pt.y += self.velocity.y;
    }
    fn unstep(&mut self) {
        self.pt.x -= self.velocity.x;
        self.pt.y -= self.velocity.y;
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

#[derive(Debug, PartialEq, Eq)]
struct Banner {
    top_left: Pt,
    bottom_right: Pt,
    chars: Vec<Char>
}

impl Banner {

    fn new(chars: &Vec<Char>) -> Banner {
        let mut top_left = Pt::max();
        let mut bottom_right = Pt::min();

        chars.iter().for_each(|c| {
            let pt = &c.pt;
            top_left.y = min(top_left.y, pt.y);
            top_left.x = min(top_left.x, pt.x);
            bottom_right.y = max(bottom_right.y, pt.y);
            bottom_right.x = max(bottom_right.x, pt.x);
        });
        Banner { top_left, bottom_right, chars: chars.clone()}
    }

    fn step(&mut self) {
        self.chars.iter_mut().for_each(|c| c.step());
        let mut top_left = Pt::max();
        let mut bottom_right = Pt::min();
        self.chars.iter().for_each(|c| {
            let pt = &c.pt;
            top_left.y = min(top_left.y, pt.y);
            top_left.x = min(top_left.x, pt.x);
            bottom_right.y = max(bottom_right.y, pt.y);
            bottom_right.x = max(bottom_right.x, pt.x);
        });
        self.top_left = top_left;
        self.bottom_right = bottom_right;
    }

    fn unstep(&mut self) {
        self.chars.iter_mut().for_each(|c| c.unstep());
        let mut top_left = Pt::max();
        let mut bottom_right = Pt::min();
        self.chars.iter().for_each(|c| {
            let pt = &c.pt;
            top_left.y = min(top_left.y, pt.y);
            top_left.x = min(top_left.x, pt.x);
            bottom_right.y = max(bottom_right.y, pt.y);
            bottom_right.x = max(bottom_right.x, pt.x);
        });
        self.top_left = top_left;
        self.bottom_right = bottom_right;
    }

    fn area(&self) -> u64 {
        ((self.top_left.x - self.bottom_right.x).abs() as u64 * (self.top_left.y - self.bottom_right.y).abs() as u64)
    }
}

impl Display for Banner {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let pt_index = self.chars.iter().map(|c| {
            &c.pt
        }).collect::<HashSet<_>>();

        for y in self.top_left.y..=self.bottom_right.y {
            for x in self.top_left.x..=self.bottom_right.x {
                let pt = Pt { x, y };
                let mut c = ".";
                if pt_index.contains(&pt) {
                    c = "#"
                }
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
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
        let mut banner = Banner::new(&self.chars);
        let mut area = banner.area();
        let mut new_area = area;
        while new_area <= area {
            banner.step();
            area = new_area;
            new_area = banner.area();
        }
        banner.unstep();

        format!("\n{}", banner)
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

    const HI: &'static str = "......................
......................
......................
......................
......#...#..###......
......#...#...#.......
......#...#...#.......
......#####...#.......
......#...#...#.......
......#...#...#.......
......#...#...#.......
......#...#..###......
......................
......................
......................
......................
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
    fn banner() {
        let chars = vec![Char{ pt: Pt{x:9,y:1}, velocity: Velocity{x:0,y:2}}, Char{ pt: Pt{x:-3,y:6}, velocity: Velocity{x:2,y:-1}}];
        let mut banner = Banner::new(&chars);
        banner.step();

        let mut moved = vec![Char{ pt: Pt{x:9,y:1}, velocity: Velocity{x:0,y:2}}, Char{ pt: Pt{x:-3,y:6}, velocity: Velocity{x:2,y:-1}}];
        moved.iter_mut().for_each(|c| c.step());
        let moved = Banner::new(&moved);

        assert_eq!(moved.chars, banner.chars);
    }

    #[test]
    fn part1() {
        let mut banner = Banner::new(&parse(EXAMPLE.to_string()).chars);
        banner.step();
        banner.step();
        banner.step();
        assert_eq!(HI, format!("{}", banner));
    }

    #[test]
    fn part2() {
        unimplemented!()
    }
}