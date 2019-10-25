use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Error, Formatter};
use std::iter;
use std::cell::RefCell;

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
    hit_pts: i16
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

    fn origin(&self) -> &Pt {
        self.pts.first().expect("empty path")
    }

    fn destination(&self) -> &Pt {
        self.pts.last().expect("empty path")
    }
}

#[derive(Debug, Clone)]
struct Map {
    locs: HashMap<Pt, Loc>,
    adjacent_pts: HashMap<Pt, Vec<Pt>>,
}

impl Map {
    fn new(locs: HashMap<Pt, Loc>) -> Self {
        // Pt -> Vec<Pt>
        let mut adjacent_pts = HashMap::new();

        for pt in locs.keys() {
            let mut others = pt.adjacent();
            others.retain(|other| {
                match locs.get(other) {
                    Some(Loc::Space) => true,
                    _ => false
                }
            });
            adjacent_pts.insert(pt.clone(), others);
        }

        Map { locs, adjacent_pts }
    }

    fn shortest_path(&self, from: &Pt, to: &Pt, excluding: &HashSet<Pt>) -> Option<Path> {
        let shortest = pathfinding::directed::dijkstra::dijkstra(
            from,
            |other| {
                let adjacents = self.adjacent_pts.get(other).cloned().unwrap_or(Vec::new());
                adjacents.iter()
                    .cloned()
                    .filter(|pt| !excluding.contains(pt))
                    .map(|o| (o, 1))
                    .collect::<Vec<_>>()
            },
            |n| n == to);

        shortest.map(|(pts, _)| Path { pts })
    }

    fn adjacent(&self, pos: &Pt) -> Vec<Pt> {
        self.adjacent_pts.get(&pos).cloned().unwrap_or_else(|| Vec::new())
    }
}

#[derive(Debug)]
enum AttackOutcome {
    NotInRange,
    Attacked(Unit)
}

#[derive(Debug)]
enum MoveOutcome {
    Unreachable,
    Moved(Pt, Pt)
}

#[derive(Debug)]
enum TurnOutcome {
    NoTargets,
    Dead(Unit),
    Alive(Unit, Option<MoveOutcome>, AttackOutcome)
}

enum Outcome {
    ElfDied,
    Solved(u32, u32)
}

// All valid paths on the board can be precomputed and then checked at runtime for blockage by a unit.
#[derive(Debug, Clone)]
struct Board {
    map: Map,
    all_units: Vec<RefCell<Unit>>,
    attack_pwr: HashMap<Kind, u16>
}

impl Board {

    fn solve_part1(&mut self) -> Outcome {
        let mut rounds = 0;
        loop {
            println!("{}", self);
            let outcomes = self.round();
            let all_done = outcomes.iter().all(|outcome| {
                match outcome {
                    TurnOutcome::NoTargets => true,
                    TurnOutcome::Dead(_) => true,
                    _ => false
                }
            });

            if all_done { break; }
            rounds += 1
        }

        let sum: u32 = self.all_units.iter()
            .filter(|x| x.borrow().hit_pts > 0)
            .map(|x| x.borrow().hit_pts as u32)
            .sum();

        Outcome::Solved(rounds, sum)
    }

    fn solve_part2(&mut self) -> Outcome {
        let mut rounds = 0;
        loop {
            println!("{}", self);
            let outcomes = self.round();

            let elf_died = outcomes.iter().any(|outcome| {
                match outcome {
                    TurnOutcome::Dead(unit) => unit.kind == Kind::Elf,
                    _ => false
                }
            });

            if elf_died { return Outcome::ElfDied } else {
                let all_done = outcomes.iter().all(|outcome| {
                    match outcome {
                        TurnOutcome::NoTargets => true,
                        TurnOutcome::Dead(_) => true,
                        _ => false
                    }
                });

                if all_done { break; }
                rounds += 1;
            }
        }

        let sum: u32 = self.all_units.iter()
            .filter(|x| x.borrow().hit_pts > 0)
            .map(|x| x.borrow().hit_pts as u32)
            .sum();

        Outcome::Solved(rounds, sum)
    }

    fn round(&mut self) -> Vec<TurnOutcome> {
        self.all_units.sort_by_key(|x| x.borrow().pos);
        let mut turn_outcomes = Vec::new();

        for current_unit in self.all_units.iter() {
            turn_outcomes.push(self.turn(&current_unit));
        }
        turn_outcomes
    }

    fn turn(&self, current_unit: &RefCell<Unit>) -> TurnOutcome {
        let cloned = current_unit.borrow().clone();
        if cloned.hit_pts <= 0 { TurnOutcome::Dead(cloned) } else {
            let potential_targets = self.all_units
                .iter()
                .filter(|other| other.borrow().hit_pts > 0)
                .filter(|other| other.borrow().kind != cloned.kind)
                .collect::<Vec<_>>();

            if potential_targets.is_empty() { return TurnOutcome::NoTargets }

            match self.attack(current_unit, &potential_targets) {
                AttackOutcome::NotInRange => {
                    let moved = self.move_unit(current_unit, &potential_targets);
                    let attack = self.attack(current_unit, &potential_targets);
                    TurnOutcome::Alive(cloned, Some(moved), attack)
                },
                outcome => TurnOutcome::Alive(cloned, None, outcome)
            }
        }
    }

    fn move_unit(&self, unit: &RefCell<Unit>, potential_targets: &Vec<&RefCell<Unit>>) -> MoveOutcome {
        // For each potential target, compute all positions in range
        //   A position in range is one that is adjacent to the target and not occupied
        let in_range = potential_targets.iter()
            .flat_map(|target| {
                self.in_range(&target.borrow().pos)
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut excluding = self.current_unit_positions();
        excluding.remove(&unit.borrow().pos);

        // Because our shortest path algorithm only returns one option, we have to instead compute the path from each possible first step around this unit
        //   From those paths, we can take the shortest ones and then pick the one where the origin is in reading order.
        let first_steps = self.in_range(&unit.borrow().pos);

        let mut reachable = first_steps
            .iter()
            .flat_map(|origin| {
                in_range
                    .iter()
                    .flat_map(|pt| self.map.shortest_path(&origin, pt, &excluding))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        reachable.sort_by_key(|x| x.pts.len());

        if reachable.is_empty() { MoveOutcome::Unreachable } else {
            let shortest_length = reachable.first().unwrap().pts.len();
            // only consider nearest
            let mut nearest = reachable.iter().filter(|x| x.pts.len() == shortest_length).collect::<Vec<_>>();

            // sort the paths in reading order, this is our
            nearest.sort_by_key(|path| path.origin());

            let move_to = nearest.first().unwrap().origin();

            let from = unit.borrow().pos.clone();
            unit.borrow_mut().pos = *move_to;
            MoveOutcome::Moved(from, *move_to)
        }
    }

    fn attack(&self, attacker: &RefCell<Unit>, potential_targets: &Vec<&RefCell<Unit>>) -> AttackOutcome {
        let mut in_range = potential_targets
            .iter()
            .filter(|target| target.borrow().pos.distance(&attacker.borrow().pos) == 1)
            .collect::<Vec<_>>();

        if in_range.is_empty() { AttackOutcome::NotInRange } else {
            // sort by hit pts, then by reading order
            in_range.sort_by(|lhs_ref, rhs_ref| {
                let lhs = lhs_ref.borrow();
                let rhs = rhs_ref.borrow();
                lhs.hit_pts.cmp(&rhs.hit_pts).then(lhs.pos.cmp(&rhs.pos))
            });

            let target = in_range.first().expect("Unexpected empty units in range");

            target.borrow_mut().hit_pts -= *self.attack_pwr.get(&attacker.borrow().kind).unwrap_or(&3) as i16;

            AttackOutcome::Attacked(target.borrow().clone())
        }
    }

    fn in_range(&self, pos: &Pt) -> Vec<Pt> {
        let current_pos = self.current_unit_positions();
        self.map.adjacent(pos)
            .iter()
            .filter(|pt| !current_pos.contains(pt))
            .cloned()
            .collect::<Vec<_>>()
    }

    fn current_unit_positions(&self) -> HashSet<Pt> {
        self.all_units.iter()
            .cloned()
            .filter(|x| x.borrow().hit_pts > 0)
            .map(|x| x.borrow().pos)
            .collect::<HashSet<_>>()
    }

    fn unit_count(&self, kind: Kind) -> usize {
        self.all_units
            .iter()
            .filter(|x| x.borrow().hit_pts > 0 && x.borrow().kind == kind)
            .count()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut pts = self.map.locs.iter().collect::<Vec<_>>();
        pts.sort_by_key(|(a,_)| **a);
        pts.iter()
            .for_each(|(pt, loc)| {
                if pt.left == 0 && pt.top != 0 {
                    writeln!(f, "").unwrap();
                }
                if let Some(unit) = self.all_units.iter().find(|u| u.borrow().pos == **pt) {
                    let dead = unit.borrow().hit_pts <= 0;
                    let c = match unit.borrow().kind {
                        Kind::Guard => if dead { 'g' } else { 'G' },
                        Kind::Elf => if dead { 'e' } else { 'E' },
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
    let mut all_units = Vec::new();
    input.trim()
        .lines()
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
                        all_units.push(RefCell::new(Unit::new(Pt::new(top as u16, left as u16), k)));
                    }
                })
        });

    Board { map: Map::new(locs), all_units, attack_pwr: HashMap::new() }
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle15 { board: parse(input) })
}

struct Puzzle15 {
    board: Board
}

impl crate::Puzzle for Puzzle15 {
    fn part1(&self) -> String {
        let mut board = self.board.clone();
        match board.solve_part1() {
            Outcome::Solved(rounds, sum) => (rounds * sum).to_string(),
            _ => panic!("unexpected outcome")
        }
    }

    fn part2(&self) -> String {
        let elf_count = self.board.unit_count(Kind::Elf);
        let mut max_failed_pwr = 3;
        let mut min_success_pwr: Option<u16> = None;
        let mut attack_pwr = 4;
        loop {
            println!("Attack power is {}", attack_pwr);
            let mut board = self.board.clone();
            board.attack_pwr.insert(Kind::Elf, attack_pwr);
            match board.solve_part2() {
                Outcome::ElfDied => {
                    max_failed_pwr = attack_pwr;
                    match min_success_pwr {
                        None => attack_pwr *= 2,
                        Some(pwr) => attack_pwr = pwr - ((pwr - max_failed_pwr) / 2)
                    }
                },
                Outcome::Solved(rounds, sum) => {
                    min_success_pwr = Some(attack_pwr);
                    if attack_pwr == max_failed_pwr + 1 {
                        println!("{}", board);
                        println!("Attack power is {}", attack_pwr);
                        println!("Solved in {} rounds with {} hps", rounds, sum);
                        return (rounds * sum).to_string()
                    }
                    attack_pwr = attack_pwr - ((attack_pwr - max_failed_pwr) / 2)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Puzzle;
    use lazy_static::lazy_static;

    const MOVE_EXAMPLE: &str = r#"#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########"#;
    const MOVE_3_EXAMPLE: &str = r#"#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########"#;

    const EXAMPLE: &str = r#"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"#;

    lazy_static! {
        static ref EXAMPLES: HashMap<&'static str, u32> = {
            let mut m = HashMap::new();

            m.insert(r#"
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
            "#, 4988);

            m.insert(r#"
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"#, 31284);

            m.insert(r#"
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"#, 3478);

            m.insert(r#"
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"#, 6474);

            m.insert(r#"
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
######### "#, 1140);

            m
        };
    }

    #[test]
    fn test_parse() {
        let printed = format!("{}", parse(MOVE_EXAMPLE.to_owned()));
        assert_eq!(MOVE_EXAMPLE, printed);
    }

    #[test]
    fn test_move() {
        let mut board = parse(MOVE_EXAMPLE.to_owned());
        board.round();
        board.round();
        board.round();
        assert_eq!(MOVE_3_EXAMPLE, format!("{}", board));
    }

    #[test]
    fn test_example() {
        let mut board = parse(EXAMPLE.to_owned());

        match board.solve_part1() {
            Outcome::ElfDied => panic!("unexpectd outcome"),
            Outcome::Solved(rounds, sum) => {
                println!("{}", board);
                println!("Done in {} rounds with {}hp left = {}", rounds, sum, sum * rounds);

                assert_eq!(47, rounds);
                assert_eq!(27730, rounds * sum);
            }
        }
    }

    #[test]
    fn test_examples_part2() {

        for (board, expected) in EXAMPLES.iter() {
            let pzl = Puzzle15 { board: parse(board.to_string()) };
            assert_eq!(expected.to_string(), pzl.part2());
        }

    }
}
