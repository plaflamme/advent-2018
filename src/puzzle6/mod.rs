use std::str::FromStr;
use std::cmp::{min, max, Ordering};
use std::collections::HashMap;
use std::fmt::Display;
use termion::color;

#[derive(PartialEq, Eq, Ord, Hash, Debug, Copy, Clone)]
struct Pt {
    left: u16,
    top: u16
}

impl Pt {

    fn max() -> Pt { Pt::new(std::u16::MAX, std::u16::MAX) }
    fn min() -> Pt { Pt::new(std::u16::MIN, std::u16::MIN) }

    fn new(left: u16, top: u16) -> Pt { Pt {left, top} }

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

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Named {
    name: char,
    coord: Pt
}


#[derive(PartialEq, Eq, Debug, Clone)]
struct Area {
    top_left: Pt,
    bottom_right: Pt,
    names: Vec<Named>
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

        Area { top_left, bottom_right, names }
    }

    fn analyze(&self, max_distance: u32) -> AreaAnalysis {
        let mut name_analysis= HashMap::new();
        for n in self.names.iter() {
            name_analysis.insert(n, NamedAnalysis { infinite: false, area: 0 });
        }
        let mut pt_analysis = HashMap::new();
        for top in self.top_left.top..=self.bottom_right.top {
            for left in self.top_left.left..=self.bottom_right.left {
                let coord = Pt { left, top };
                let is_frontier = top == self.top_left.top || top == self.bottom_right.top || left == self.bottom_right.left || left == self.top_left.left;
                let mut distances = HashMap::new();
                let mut shortest = std::u16::MAX;
                self.names.iter().for_each(|x| {
                    let distance = x.coord.distance(&coord);
                    distances.insert(x, distance);
                    shortest = min(shortest, distance);
                });
                let dominating = distances.iter().filter_map(|(name, dist)| {
                    if *dist == shortest { Some(*name) } else { None }
                }).collect::<Vec<_>>();

                let dominated = if dominating.len() != 1 { None } else {
                    let dom = *dominating.first().expect("cannot happen");
                    let a = name_analysis.get(dom).expect("cannot happen");
                    let na = NamedAnalysis { infinite: a.infinite || is_frontier, area: a.area + 1 };
                    name_analysis.insert(dom, na);
                    Some(dom)
                };

                let dist_sum: u32 = distances.values().map(|x| *x as u32).sum();
                let is_part2_region = dist_sum < max_distance;

                pt_analysis.insert(coord, PtAnalysis { is_frontier, is_part2_region, distances, dominated });
            }
        }
        AreaAnalysis { area: self, pt_analysis, name_analysis }
    }
}

#[derive(Debug)]
struct NamedAnalysis {
    infinite: bool,
    area: u32
}

struct PtAnalysis<'a> {
    is_frontier: bool,
    is_part2_region: bool,
    distances: HashMap<&'a Named, u16>,
    dominated: Option<&'a Named> // Some when the named origin dominates this coordinate
}
struct AreaAnalysis<'a> {
    area: &'a Area,
    pt_analysis: HashMap<Pt, PtAnalysis<'a>>,
    name_analysis: HashMap<&'a Named, NamedAnalysis>,
}

impl Display for AreaAnalysis<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for top in self.area.top_left.top..=self.area.bottom_right.top {
            for left in self.area.top_left.left..=self.area.bottom_right.left {
                let pt = Pt { left, top };
                let analysis = self.pt_analysis.get(&pt).expect(&format!("missing pt {:?} in analysis", pt).to_owned());

                let is_name = analysis.dominated.map(|x| {
                    let dist = *analysis.distances.get(x).expect("invalid area");
                    dist == 0
                }).unwrap_or(false);

                let symbol = match (analysis.dominated, analysis.is_part2_region) {
                    (None, true) =>'#',
                    (None, false) => '.',
                    (Some(p), true) => if is_name { p.name } else { '#' },
                    (Some(p), _) => p.name
                };

                match (analysis.is_frontier, is_name) {
                    (false, false) => write!(f, "{}{}", color::Fg(color::Cyan), symbol)?,
                    (false, true) => write!(f, "{}{}", color::Fg(color::LightCyan), symbol)?,
                    (true, false) => write!(f, "{}{}", color::Fg(color::Red), symbol)?,
                    (true, true) => write!(f, "{}{}", color::Fg(color::LightRed), symbol)?,
                }
            }
            write!(f, "\n{}", color::Fg(color::Reset))?;
        }
        Ok(())
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
        let analysis = area.analyze(10000);

        println!("{}", analysis);
        let mut finite_areas = analysis.name_analysis.iter()
            .filter(|(_, a)| !a.infinite)
            .collect::<Vec<_>>();

        finite_areas.sort_by_key(|(_, a)| 0-a.area as i32);


        finite_areas.iter().for_each(|(name, analysis)| {
            println!("{:?} -> {:?}", name, analysis)
        });

        let (_, largest) = finite_areas.first().expect("invalid solution");
        format!("{}", largest.area)
    }

    fn part2(&self) -> String {
        let area = Area::new(&self.coords);
        let analysis = area.analyze(10000);

        analysis.pt_analysis.values().filter(|x| x.is_part2_region).count().to_string()
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

        let area = Area::new(&vec![Pt::new(1,1), Pt::new(1,6), Pt::new(8,3), Pt::new(3,4), Pt::new(5,5), Pt::new(8,9)] );
        println!("{}", area.analyze(32));
    }
}