use std::str::FromStr;
use std::cmp::{min, max, Ordering};
use std::ops::Add;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Ord, Hash, Debug, Copy, Clone)]
struct Pt {
    left: u16,
    top: u16
}

impl Pt {

    fn max() -> Pt { Pt::new(std::u16::MAX, std::u16::MAX) }
    fn min() -> Pt { Pt::new(std::u16::MIN, std::u16::MIN) }

    fn new(left: u16, top: u16) -> Pt { Pt {left, top} }

    fn left(&self, by: u16) -> Pt {
        Pt { left: self.left - by, top: self.top }
    }
    fn right(&self, by: u16) -> Pt {
        Pt { left: self.left + by, top: self.top }
    }
    fn down(&self, by: u16) -> Pt {
        Pt { left: self.left, top: self.top + by }
    }
    fn up(&self, by: u16) -> Pt {
        Pt { left: self.left, top: self.top - by }
    }

    fn distance(&self, other: &Pt) -> u16 {
        ((self.left as i32 - other.left as i32).abs() + (self.top as i32 - other.top as i32).abs()) as u16
    }
}

impl PartialOrd for Pt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let origin = &Pt::new(0,0);
        let self_dist = self.distance(origin);
        let other_dist = other.distance(origin);

        if self_dist < other_dist { Some(Ordering::Less) }
        else if other_dist < self_dist { Some(Ordering::Greater) }
        else { Some(Ordering::Equal) }
    }
}

impl FromStr for Pt {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').map(|x| x.trim()).collect::<Vec<_>>();

        let left = u16::from_str(parts[0])?;
        let top = u16::from_str(parts[1])?;

        Ok(Pt { left, top })
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Named {
    name: char,
    coord: Pt
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Area {
    top_left: Pt,
    bottom_right: Pt,
    pts: Vec<Named>
}

impl Area {

    fn new(pts: &Vec<Pt>) -> Area {
        let mut names = Vec::new();
        let mut name = 'A';
        let mut top_left = Pt::max();
        let mut bottom_right = Pt::min();

        pts.iter().for_each(|pt| {
            top_left.top = min(top_left.top, pt.top);
            top_left.left = min(top_left.left, pt.left);
            bottom_right.top = max(bottom_right.top, pt.top);
            bottom_right.left = max(bottom_right.left, pt.left);
            names.push(Named { name, coord: *pt });
            name = (name as u8 + 1 as u8) as char;
        });

        Area { top_left, bottom_right, pts: names }
    }

    fn print(&self) -> String {
        let index = self.pts.iter()
            .map(|x| {
                (x.coord, x.name)
            })
            .collect::<HashMap<_, _>>();

        let mut surface = String::new();
        for top in 0..=self.bottom_right.top+1 {
            for left in 0..=self.bottom_right.left+1 {

                let pt = Pt { left, top };
                let name = index.get(&pt).unwrap_or(&'.');

                surface = surface.add(&name.to_string());
            }
            surface = surface.add("\n");
        }
        surface
    }

}

fn parse(input: String) -> Vec<Pt> {
    let mut pts = input.lines().map(|x| Pt::from_str(x).expect("invalid input")).collect::<Vec<_>>();
    pts.sort();
    pts
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle6 { coords: parse(input) })
}

struct Puzzle6 {
    coords: Vec<Pt>
}

impl crate::Puzzle for Puzzle6 {
    fn part1(&self) -> String {
        let area = Area::new(&self.coords);

        println!("{}", area.print());

        unimplemented!()
    }

    fn part2(&self) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(Pt { left: 1, top: 1}.distance(&Pt { left: 1, top: 1 }), 0);
        assert_eq!(Pt { left: 1, top: 1}.distance(&Pt { left: 1, top: 2 }), 1);
        assert_eq!(Pt { left: 1, top: 1}.distance(&Pt { left: 2, top: 2 }), 2);
        assert_eq!(Pt { left: 1, top: 1}.distance(&Pt { left: 3, top: 2 }), 3);
    }

    #[test]
    fn test_area() {
//        assert_eq!(Area::new(&Pt { left: 1, top: 1 }), Area { top_left: Pt { left: 1, top: 1}, bottom_right: Pt { left: 1, top: 1} });
//        assert_eq!(*Area::new(&Pt { left: 1, top: 1 }).add(&Pt { left: 3, top: 3}), Area { top_left: Pt { left: 1, top: 1}, bottom_right: Pt { left: 3, top: 3} });
//        assert_eq!(*Area::new(&Pt { left: 1, top: 1 }).add(&Pt { left: 3, top: 3}).add(&Pt { left: 2, top: 4}), Area { top_left: Pt { left: 1, top: 1}, bottom_right: Pt { left: 3, top: 4} });

        let mut area = Area::new(&vec![Pt::new(1,1), Pt::new(1,6), Pt::new(8,3), Pt::new(3,4), Pt::new(5,5), Pt::new(8,9)] );
        println!("{}", area.print());
    }
}