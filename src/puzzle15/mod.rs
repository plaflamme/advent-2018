use std::collections::{HashMap, HashSet, BinaryHeap};
use std::fmt::{Display, Error, Formatter};
use std::iter;
use std::cell::RefCell;
use std::cmp::{Reverse, Ordering};

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
struct Pt { top: u16, left: u16 }
impl Pt {
    fn new(top: u16, left: u16) -> Self {
        Pt { top, left }
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

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Kind::Guard => write!(f, "G"),
            Kind::Elf =>  write!(f, "E")
        }
    }
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

#[derive(Hash, PartialEq, Eq, Ord, Debug, Clone)]
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

impl PartialOrd for Path {

    // This part is pretty crucial and wasn't very clear in the instructions
    //   The best path is the shortest, but tie breaking is reading order of destination and then first step
    //   My original solution was only checking first step which works for all test examples
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.pts.len().cmp(&other.pts.len())
            .then(self.destination().cmp(other.destination()))
            .then(self.origin().cmp(other.origin())))
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
            if !others.is_empty() { adjacent_pts.insert(pt.clone(), others); };
        }

        Map { locs, adjacent_pts }
    }

    fn shortest_path(&self, from: &Pt, to: &Pt, excluding: &HashSet<Pt>) -> Option<Path> {
        let shortest = pathfinding::directed::dijkstra::dijkstra(
            from,
            |other| {
                self.adjacent(&other).iter()
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
    Unreachable,
    Dead(Unit),
    Alive(Unit, Option<MoveOutcome>, AttackOutcome)
}

#[derive(Debug)]
enum RoundOutcome {
    Partial(Vec<TurnOutcome>),
    Full(Vec<TurnOutcome>)
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
            println!("Starting round {}", rounds + 1);
            println!("{}", self);
            match self.round() {
                RoundOutcome::Full(_) => rounds += 1,
                RoundOutcome::Partial(_) => {
                    let sum: u32 = self.all_units.iter()
                        .filter(|x| x.borrow().hit_pts > 0)
                        .map(|x| x.borrow().hit_pts as u32)
                        .sum();
                    break Outcome::Solved(rounds, sum);
                }
            }
        }
    }

    fn solve_part2(&mut self) -> Outcome {
        let mut rounds = 0;
        loop {
            println!("Starting round {}", rounds + 1);
            println!("{}", self);
            let round_outcome = &self.round();

            let elf_died = self.all_units
                .iter()
                .filter(|x| x.borrow().hit_pts <= 0)
                .find(|unit| unit.borrow().kind == Kind::Elf)
                .is_some();

            if elf_died { return Outcome::ElfDied } else {
                match round_outcome {
                    RoundOutcome::Partial(_) => break,
                    RoundOutcome::Full(_) => rounds += 1
                };
            }
        }

        let sum: u32 = self.all_units.iter()
            .filter(|x| x.borrow().hit_pts > 0)
            .map(|x| x.borrow().hit_pts as u32)
            .sum();

        Outcome::Solved(rounds, sum)
    }

    fn round(&mut self) -> RoundOutcome {
        self.all_units.sort_by_key(|x| x.borrow().pos);
        let mut turn_outcomes = Vec::new();

        for current_unit in self.all_units.iter() {
            match self.turn(&current_unit) {
                TurnOutcome::NoTargets => return RoundOutcome::Partial(turn_outcomes),
                outcome => turn_outcomes.push(outcome)
            }
        }
        // If the last unit has a chance to finish, then the round is a full round even if one side wins at this point.
        RoundOutcome::Full(turn_outcomes)
    }

    fn turn(&self, current_unit: &RefCell<Unit>) -> TurnOutcome {
        let cloned = current_unit.borrow().clone();
        if cloned.hit_pts <= 0 { TurnOutcome::Dead(cloned) } else {
            let potential_targets = self.all_units
                .iter()
                .filter(|other| other.borrow().hit_pts > 0)
                .filter(|other| other.borrow().kind != cloned.kind)
                .collect::<Vec<_>>();

            if potential_targets.is_empty() { TurnOutcome::NoTargets } else {
                match self.attack(current_unit, &potential_targets) {
                    AttackOutcome::NotInRange => {
                        match self.move_unit(current_unit, &potential_targets) {
                            MoveOutcome::Unreachable => TurnOutcome::Unreachable,
                            moved => {
                                let attack = self.attack(current_unit, &potential_targets);
                                TurnOutcome::Alive(cloned, Some(moved), attack)
                            }
                        }
                    },
                    outcome => TurnOutcome::Alive(cloned, None, outcome)
                }
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

        // Because our shortest path algorithm only returns one option, we have to instead compute the path from each possible first step around this unit
        //   From those paths, we can take the shortest ones and then pick the one where the origin is in reading order.
        let first_steps = self.in_range(&unit.borrow().pos);

        let chosen = first_steps
            .iter()
            .flat_map(|origin| {
                in_range
                    .iter()
                    .flat_map(|pt| self.map.shortest_path(&origin, pt, &self.current_unit_positions()))
                    .collect::<Vec<_>>()
            })
            .map(|path| Reverse(path))
            .collect::<BinaryHeap<_>>()
            .peek()
            .map(|Reverse(path)| path.clone());

         match chosen {
             None => MoveOutcome::Unreachable,
             Some(path) => {
                 let move_to = *path.origin();
                 let from = unit.borrow().pos.clone();
                 unit.borrow_mut().pos = move_to;
                 MoveOutcome::Moved(from, move_to)
             }
         }
    }

    fn attack(&self, attacker: &RefCell<Unit>, potential_targets: &Vec<&RefCell<Unit>>) -> AttackOutcome {
        let mut in_range = potential_targets
            .iter()
            .filter(|target| {
                self.map.adjacent(&target.borrow().pos).contains(&attacker.borrow().pos)
            })
            .collect::<Vec<_>>();

        in_range.sort_by(|lhs_ref, rhs_ref| {
            let lhs = lhs_ref.borrow();
            let rhs = rhs_ref.borrow();
            lhs.hit_pts.cmp(&rhs.hit_pts).then(lhs.pos.cmp(&rhs.pos))
        });

        match in_range.first() {
            None => AttackOutcome::NotInRange,
            Some(target) => {
                target.borrow_mut().hit_pts -= *self.attack_pwr.get(&attacker.borrow().kind).unwrap_or(&3) as i16;
                AttackOutcome::Attacked(target.borrow().clone())
            }
        }
    }

    // all Pts "in range" of the specied Pt
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
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut pts = self.map.locs.iter().collect::<Vec<_>>();
        let mut line_units: Vec<&RefCell<Unit>> = Vec::new();
        pts.sort_by_key(|(a,_)| **a);
        pts.iter()
            .for_each(|(pt, loc)| {
                if pt.left == 0 && pt.top != 0 {
                    let summary = line_units.drain(0..).map(|u| format!("{}({})", u.borrow().kind, u.borrow().hit_pts)).collect::<Vec<_>>().join(", ");
                    writeln!(f, "   {}", summary).unwrap();
                }
                if let Some(unit) = self.all_units.iter().find(|u| u.borrow().hit_pts > 0 && u.borrow().pos == **pt) {
                    line_units.push(unit);
                    write!(f, "{}", unit.borrow().kind).unwrap()
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
    fn test_move() {
        let expected = parse(MOVE_3_EXAMPLE.to_owned());
        let mut board = parse(MOVE_EXAMPLE.to_owned());
        board.round();
        board.round();
        board.round();

        let mut e = expected.all_units.iter().map(|x| (x.borrow().kind, x.borrow().pos)).collect::<Vec<_>>();
        e.sort();
        let mut b = board.all_units.iter().map(|x| (x.borrow().kind, x.borrow().pos)).collect::<Vec<_>>();
        b.sort();

        assert_eq!(e, b);
    }

    #[test]
    fn test_part1() {
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
    fn test_part2() {

        for (board, expected) in EXAMPLES.iter() {
            let pzl = Puzzle15 { board: parse(board.to_string()) };
            assert_eq!(expected.to_string(), pzl.part2());
        }

    }
}
