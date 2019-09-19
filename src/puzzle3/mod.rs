use std::str::FromStr;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Pt {
    left: u16,
    top: u16
}

impl Pt {
    fn right(&self, by: u16) -> Pt {
        Pt { left: self.left + by, top: self.top }
    }
    fn down(&self, by: u16) -> Pt {
        Pt { left: self.left, top: self.top + by }
    }
}

impl FromStr for Pt {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<_>>();

        let left = u16::from_str(parts[0])?;
        let top = u16::from_str(parts[1])?;

        Ok(Pt { left, top })
    }
}

#[derive(Debug)]
struct Claim {
    id: String,
    orig: Pt,
    width: u16,
    height: u16
}

impl Claim {
    fn surface(&self) -> HashSet<Pt> {
        (0..self.width)
            .flat_map(|right| {
                let moved = self.orig.right(right);
                (0..self.height).map(move |down| {
                    moved.down(down)
                })
            })
            .collect::<HashSet<Pt>>()

    }
}

impl FromStr for Claim {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();

        let id = {
            let id_part = parts[0].to_string();
            id_part[1..].to_owned()
        };

        let orig = {
            let part = parts[2].to_string();
            Pt::from_str(&part[0..part.len()-1])?
        };

        let (width, height) = {
            let part = parts[3].to_string();
            let wh = part.split('x').collect::<Vec<_>>();

            (u16::from_str(wh[0])?, u16::from_str(wh[1])?)
        };

        Ok(Claim { id, orig, width, height })
    }
}

fn parse() -> Vec<Claim> {
    let content = std::fs::read_to_string("src/puzzle3/input.txt").expect("cannot read puzzle input.");
    content.lines()
        .map(|x| Claim::from_str(x).unwrap_or_else(|_| panic!("invalid line {}", x)))
        .collect::<Vec<_>>()
}

fn intersecting() -> HashSet<Pt> {
    let surfaces = &parse()
        .iter()
        .map(|x| {
            x.surface()
        })
        .collect::<Vec<_>>();

    // TODO: this is super slow
    let mut intersecting = HashSet::new();
    for a in surfaces {
        for b in surfaces {
            if a == b { continue };
            a.intersection(&b).for_each(|x| {
                intersecting.insert(*x);
            });
        }
    }

    intersecting
}

pub struct Puzzle3;

impl crate::Puzzle for Puzzle3 {

    fn part1(&self) -> String {
        intersecting().len().to_string()
    }
    fn part2(&self) -> String {
        let pts = intersecting();
        parse()
            .iter()
            .find_map(|claim| {
                if claim.surface().intersection(&pts).collect::<HashSet<_>>().is_empty() { Some(claim.id.to_owned()) } else { None }
            })
            .expect("no claim is not intersecting")
    }
}