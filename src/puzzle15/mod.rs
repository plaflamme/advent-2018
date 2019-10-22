use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
struct Pt { top: u16, left: u16 }
impl Pt {
    fn new(top: u16, left: u16) -> Self {
        Pt { top, left }
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
    kind: Kind,
    hit_pts: u8
}

impl Unit {
    fn new(kind: Kind) -> Self {
        Unit { kind, hit_pts: 200 }
    }
}

#[derive(Debug, Clone)]
struct Board {
    locs: HashMap<Pt, Loc>,
    units: HashMap<Pt, Unit>
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
                if let Some(unit) = self.units.get(pt) {
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
    let mut units = HashMap::new();
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
                        units.insert(Pt::new(top as u16, left as u16), Unit::new(k));
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
}
