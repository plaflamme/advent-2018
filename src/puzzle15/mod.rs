use std::collections::{HashMap, HashSet, BinaryHeap};
use std::fmt::{Display, Error, Formatter};
use std::cmp::Ordering;
use std::iter;

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
struct Pt { top: u16, left: u16 }
impl Pt {
    fn new(top: u16, left: u16) -> Self {
        Pt { top, left }
    }

    fn distance(&self, other: &Pt) -> u16 {
        ((self.left as i32 - other.left as i32).abs() + (self.top as i32 - other.top as i32).abs()) as u16
    }

    fn pt_left(&self) -> Option<Self> {
        if self.left > 0 { Some(Pt { top: self.top, left: self.left - 1 }) } else { None }
    }
    fn pt_right(&self) -> Self {
        Pt { top: self.top, left: self.left + 1 }
    }
    fn pt_up(&self) -> Option<Self> {
        if self.top > 0 { Some(Pt { top: self.top - 1, left: self.left }) } else { None }
    }
    fn pt_down(&self) -> Self {
        Pt { top: self.top + 1, left: self.left }
    }

    fn adjacent(&self) -> Vec<Pt> {
        self.pt_up().iter()
            .chain(self.pt_left().iter())
            .chain(iter::once(&self.pt_down()))
            .chain(iter::once(&self.pt_right()))
            .cloned()
            .collect()
    }
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
enum Loc {
    Wall,
    Space
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
enum Kind {
    Guard,
    Elf
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
struct Unit {
    pos: Pt,
    kind: Kind,
    hit_pts: u8
}

impl Unit {
    fn new(pos: Pt, kind: Kind) -> Self {
        Unit { pos, kind, hit_pts: 200 }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Path {
    pts: Vec<Pt>
}

impl Path {
    fn new(root: &Pt, suffix: &Path) -> Self {
        let mut pts = Vec::new();
        pts.push(root.clone());
        pts.append(&mut suffix.pts.clone());
        Path { pts }
    }

    fn origin(&self) -> &Pt {
        self.pts.first().expect("empty path")
    }

    fn destination(&self) -> &Pt {
        self.pts.last().expect("empty path")
    }
}

struct Target {
    unit: Unit,
    path: Path
}

// All valid paths on the board can be precomputed and then checked at runtime for blockage by a unit.
#[derive(Debug, Clone)]
struct Board {
    locs: HashMap<Pt, Loc>,
    units: HashSet<Unit>
}

fn paths(origin: &Pt, visited: &HashSet<Pt>, adjacent: &HashMap<Pt, Vec<Pt>>) -> Vec<Path> {

    match adjacent.get(&origin) {
        None => Vec::new(),
        Some(others) => {
            let mut candidates = others.clone();
            candidates.retain(|other| !visited.contains(other));
            let mut visited_and_self = visited.clone();
            visited_and_self.insert(*origin);

            let mut all_paths = Vec::new();
            for candidate in candidates {
                let sub_paths = paths(&candidate, &visited_and_self, adjacent);

                // because we've computed a portion of that nodes paths, we should keep them around so we don't have to recompute them
//                all_paths.append(&mut sub_paths.clone());

                for sub in sub_paths {
                    all_paths.push(Path::new(origin, &sub));
                }
            }
            all_paths
        }
    }
}

impl Board {

    fn compute_paths(&self) {

        // Pt -> Vec<Pt>
        let mut adjacent = HashMap::new();

        for pt in self.locs.keys() {
            let mut others = pt.adjacent();
            others.retain(|other| {
                match self.locs.get(other) {
                    Some(Loc::Space) => true,
                    _ => false
                }
            });
            adjacent.insert(pt.clone(), others);
        }
    }

    fn step(&self) {
        // for each unit in reading order of starting position
        //   for each target
        //     compute shortest paths to target
        //   if candidate_paths is empty { done }
        //   else {
        //     shortest_paths = shortest(candidate_paths)
        //     path = reading_order(shortest_paths)
        //     move_to(path.head) // this needs to handle not moving at all when we're already next to a target
        //   }
        //   if let Some(target) = reading_order(weakest(adjacent targets)) {
        //     attack(target)
        //     if target.isDead { remove(target) }
        //   }

//        let mut ordered_units = BinaryHeap::new();
//        for u in self.units.iter() {
//            ordered_units.push(&u);
//        }

//        for current_unit in ordered_units {
//            let potential_targets = self.units.iter().filter(|other| other.kind != current_unit.kind).collect::<Vec<_>>();
//            for target in potential_targets {
//                let shortest_path = Path::new(&current_unit.pos, &target.pos, &self.locs);
//                Target { unit: target.clone(), path: shortest_path }
//            }
//            // move
//            // attack
//        }

        unimplemented!()
    }

    fn path(&self, from: &Pt, to: &Pt) -> Vec<Pt> {
        unimplemented!()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut pts = self.locs.iter().collect::<Vec<_>>();
        pts.sort_by_key(|(a,_)| **a);
        pts.iter()
            .for_each(|(pt, loc)| {
                if pt.left == 0 && pt.top != 0 {
                    writeln!(f, "").unwrap();
                }
                if let Some(unit) = self.units.iter().find(|u| u.pos == **pt) {
                    let c = match unit.kind {
                        Kind::Guard => 'G',
                        Kind::Elf => 'E',
                    };
                    write!(f, "{}", c).unwrap()
                } else {
                    let c = match loc {
                        Loc::Wall => '#',
                        Loc::Space => '.',
                    };
                    write!(f, "{}", c).unwrap()
                }
            });
        Ok(())
    }
}

fn parse(input: String) -> Board {
    let mut locs = HashMap::new();
    let mut units = HashSet::new();
    input.lines()
        .enumerate()
        .for_each(|(top, line)| {
            line.chars()
                .enumerate()
                .for_each(|(left, c)| {
                    let (kind, loc) = match c {
                        '#' => (None, Loc::Wall),
                        '.' => (None, Loc::Space),
                        'G' => (Some(Kind::Guard), Loc::Space),
                        'E' => (Some(Kind::Elf), Loc::Space),
                        _ => panic!(format!("unexpected char {}", c))
                    };

                    locs.insert(Pt::new(top as u16, left as u16), loc);
                    if let Some(k) = kind {
                        units.insert(Unit::new(Pt::new(top as u16, left as u16), k));
                    }
                })
        });

    Board { locs, units }
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle15 { board: parse(input) })
}

struct Puzzle15 {
    board: Board
}

impl crate::Puzzle for Puzzle15 {
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

    const EXAMPLE: &str = r#"#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########"#;

    #[test]
    fn test_parse() {
        let printed = format!("{}", parse(EXAMPLE.to_owned()));
        assert_eq!(EXAMPLE, printed);
    }

    #[test]
    fn test_compute_paths() {}
}
