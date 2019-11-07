use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    North,
    East,
    South,
    West
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Path {
    Segment(VecDeque<Dir>), // NESW
    Branch(VecDeque<Path>), // (N|E|S)
    Sequence(VecDeque<Path>), // NESW(SWE|)WSN
    Noop
}

impl Path {

    fn from(s: &str) -> Path {
        let mut remains = s;
        let mut sequence = VecDeque::new();
        loop {
            match remains.chars().next() {
                None | Some('$') => break,
                Some('^') | Some(')') => {
                    let (_, r) = remains.split_at(1);
                    remains = r;
                }
                Some(_) => {
                    let (path, s) = Path::parse_sequence(remains);
                    remains = s;
                    sequence.push_back(path);
                }
            }
        }

        match sequence.len() {
            1 => sequence.front().unwrap().clone(),
            _ => Path::Sequence(sequence)
        }
    }

    fn parse_segment(s: &str) -> (Path, &str) {
        let mut segment = VecDeque::new();
        for c in s.chars() {
            match c {
                'N' => segment.push_back(Dir::North),
                'E' => segment.push_back(Dir::East),
                'S' => segment.push_back(Dir::South),
                'W' => segment.push_back(Dir::West),
                _ => break
            }
        }
        (Path::Segment(segment.clone()), &s[segment.len()..])
    }

    // a branch ends when we hit a closing bracket and will consume it
    fn parse_branch(s: &str) -> (Path, &str) {
        let mut remains = s;
        let mut branches = VecDeque::new();
        loop {
            match remains.chars().next() {
                Some(')') => {
                    let (_, b) = remains.split_at(1);
                    remains = b;
                    break
                },
                Some('|') | Some('(') => {
                    let (_, b) = remains.split_at(1);
                    let (path, s) = Path::parse_sequence(b);
                    remains = s;
                    branches.push_back(path);
                },
                c => panic!(format!("unexpected char {:?}", c))
            };
        };

        (Path::Branch(branches), remains)
    }

    // a sequence ends when we hit a closing bracket or a |, but does not consume it
    fn parse_sequence(s: &str) -> (Path, &str) {
        let mut remains = s;
        let mut sequence = VecDeque::new();

        loop {
            match remains.chars().next() {
                None => break,
                Some(c) => {
                    match c {
                        '$' | ')' | '|' => break,
                        '(' => {
                            let (branch, s) = Path::parse_branch(remains);
                            remains = s;
                            sequence.push_back(branch);
                        },
                        'N' | 'E' | 'S' | 'W' => {
                            let (segment, s) = Path::parse_segment(remains);
                            remains = s;
                            sequence.push_back(segment);
                        },
                        c => panic!(format!("unexpected char {:?}", c))
                    }
                }
            };
        };

        // simplify the path if possible
        match sequence.len() {
            0 => (Path::Noop, remains),
            1 => (sequence.front().unwrap().clone(), remains),
            _ => (Path::Sequence(sequence), remains)
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
struct Pt {
    y: i32, // sort by y first
    x: i32
}

impl Pt {
    fn new(x: i32, y: i32) -> Self {
        Pt{x,y}
    }
    fn max() -> Self { Pt::new(std::i32::MAX, std::i32::MAX) }
    fn min() -> Self { Pt::new(std::i32::MIN, std::i32::MIN) }

    // NW is the minimum point and SE is the maximum
    fn north(&self) -> Self { Pt::new(self.x, self.y - 1) }
    fn east(&self) -> Self { Pt::new(self.x + 1, self.y) }
    fn south(&self) -> Self { Pt::new(self.x, self.y + 1) }
    fn west(&self) -> Self { Pt::new(self.x - 1, self.y) }

    fn at(&self, dir: Dir) -> Pt {
        match dir {
            Dir::North => self.north(),
            Dir::East => self.east(),
            Dir::South => self.south(),
            Dir::West => self.west(),
        }
    }

    fn neighbours(&self) -> Vec<Pt> {
        vec![self.north(), self.east(), self.south(), self.west()]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Loc {
    Door,
    Wall,
    Room,
    Unk
}

#[derive(Debug)]
struct Solution {
    room_pt: Pt,
    path: Vec<Pt>
}

struct Map {
    locations: HashMap<Pt, Loc>,
    current_pos: Pt
}

impl Map {
    fn new() -> Self {
        let mut locations = HashMap::new();
        let current_pos = Pt::new(0,0);
        locations.insert(current_pos, Loc::Room);
        current_pos.neighbours().iter().for_each(|x| { locations.insert(x.clone(), Loc::Unk); });
        Map { locations, current_pos }
    }

    fn follow(&mut self, directions: &VecDeque<Dir>) {
        let mut remains = directions.clone();
        match remains.pop_front() {
            None => { // set all remaining unknown locations to Wall
                self.locations.iter_mut().for_each(|(pt, loc)| {
                    if *loc == Loc::Unk {
                        *loc = Loc::Wall;
                    }
                });
            },
            Some(dir) => {
                let door_pt = self.current_pos.at(dir);
                self.locations.insert(door_pt, Loc::Door);
                self.current_pos = door_pt.at(dir);
                self.locations.insert(self.current_pos, Loc::Room);
                self.current_pos.neighbours().iter().for_each(|pt| {
                    if !self.locations.contains_key(pt) {
                        self.locations.insert(*pt, Loc::Unk);
                    }
                });
                self.follow(&remains);
            }
        }
    }

    fn follow_path(&mut self, path: &Path) {
        match path {
            Path::Noop => (),
            Path::Segment(directions) => self.follow(&directions),
            Path::Sequence(paths) => paths.iter().for_each(|path| self.follow_path(path)),
            Path::Branch(paths) => {
                let original_pos = self.current_pos;
                for path in paths {
                    self.follow_path(path);
                    self.current_pos = original_pos;
                }
            }
        }
    }

    fn room_doors(&self, pt: &Pt) -> Vec<Dir> {
        let door_at = |dir: Dir| {
            self.locations.get(&pt.at(dir))
                .filter(|l| **l == Loc::Door)
                .map(|_| dir)
        };

        vec![door_at(Dir::North), door_at(Dir::East), door_at(Dir::South), door_at(Dir::West)]
            .iter()
            .flatten()
            .cloned()
            .collect()
    }

    fn part1_solution(&self) -> Solution {
        let mut paths = self.locations
            .iter()
            .filter(|(pt, loc)| **loc == Loc::Room && **pt != Pt::new(0,0))
            .map(|(other_room, _)| {
                let (path, _) = pathfinding::directed::dijkstra::dijkstra(
                    &Pt::new(0,0),
                    |room_pt| {
                        self.room_doors(room_pt)
                            .iter()
                            .map(|dir| (room_pt.at(*dir).at(*dir), 1))
                            .collect::<Vec<_>>()
                    },
                    |pt| { pt == other_room }
                ).expect(&format!("unable to find a path to {:?} in map {}", other_room, self));

                (other_room, path)
            })
            .collect::<Vec<_>>();

        paths.sort_by_key(|(_, path)| path.len());

        let (other, longest_shortest_path) = paths.last().unwrap();

        Solution { room_pt: **other, path: longest_shortest_path.clone() }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut keys = self.locations.keys().collect::<Vec<_>>();
        let mut xs = keys.iter().map(|x| x.x).collect::<Vec<_>>();
        xs.sort();
        let mut ys = keys.iter().map(|x| x.y).collect::<Vec<_>>();
        ys.sort();
        let (min_x,max_x) = (*xs.first().unwrap(), *xs.last().unwrap());
        let (min_y,max_y) = (*ys.first().unwrap(), *ys.last().unwrap());

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if x == 0 && y == 0 {
                    write!(f, "X")?
                } else {
                    let c = match self.locations.get(&Pt::new(x,y)) {
                        Some(Loc::Door) => '|',
                        Some(Loc::Room) => '.',
                        _ => '#'
                    };
                    write!(f, "{}", c)?
                }
            }
            writeln!(f, "")?
        }

        Ok(())
    }
}

fn parse(input: &str) -> VecDeque<Dir> {
    input.chars()
        .flat_map(|c| {
            match c {
                '^' | '$' => None,
                'N' => Some(Dir::North),
                'E' => Some(Dir::East),
                'S' => Some(Dir::South),
                'W' => Some(Dir::West),
                '(' | ')' => unimplemented!(),
                _ => panic!(format!("unexpected character {}", c))
            }
        })
        .collect::<VecDeque<_>>()
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle20 { regex: input })
}

struct Puzzle20 {
    regex: String
}

impl crate::Puzzle for Puzzle20 {
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
    use lazy_static::lazy_static;

    lazy_static! {
        static ref EXAMPLES: HashMap<&'static str, (u32, &'static str)> = {
            let mut m = HashMap::new();

            m.insert("^WNE$", (3, r#"#####
#.|.#
#|###
#.|X#
#####
"#));

            m.insert("^ENWWW(NEEE|SSE(EE|N))$", (10, r#"#########
#.|.|.|.#
#-#######
#.|.|.|.#
#-#####-#
#.#.#X|.#
#-#-#####
#.|.|.|.#
#########
"#));

            m.insert("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$", (18, r#"###########
#.|.#.|.#.#
#-###-#-#-#
#.|.|.#.#.#
#-#####-#-#
#.#.#X|.#.#
#-#-#####-#
#.#.|.|.|.#
#-###-###-#
#.|.|.#.|.#
###########
"#));

            m.insert("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$", (23, r#"#############
#.|.|.|.|.|.#
#-#####-###-#
#.#.|.#.#.#.#
#-#-###-#-#-#
#.#.#.|.#.|.#
#-#-#-#####-#
#.#.#.#X|.#.#
#-#-#-###-#-#
#.|.#.|.#.#.#
###-#-###-#-#
#.|.#.|.|.#.#
#############
"#));

            m.insert("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$", (31, r#"###############
#.|.|.|.#.|.|.#
#-###-###-#-#-#
#.|.#.|.|.#.#.#
#-#########-#-#
#.#.|.|.|.|.#.#
#-#-#########-#
#.#.#.|X#.|.#.#
###-#-###-#-#-#
#.|.#.#.|.#.|.#
#-###-#####-###
#.|.#.|.|.#.#.#
#-#-#####-#-#-#
#.#.|.|.|.#.|.#
###############
"#));
            m
        };
    }

    #[test]
    fn test_examples() {
        for (path_str, (expected_solution, expected_map)) in EXAMPLES.iter() {
            let mut map = Map::new();
            map.follow_path(&Path::from(path_str));
            let sol = map.part1_solution();

            assert_eq!(expected_map.replace("-", "|"), format!("{}", map));
            assert_eq!(*expected_solution, sol.path.len() as u32 - 1);
        }
    }

    #[test]
    fn test_path_parse() {
        assert_eq!(
            Path::Segment(VecDeque::from(vec![Dir::West, Dir::North, Dir::East])),
            Path::from("^WNE$")
        );
        assert_eq!(
            Path::Sequence(
                VecDeque::from(vec![
                    Path::Segment(VecDeque::from(vec![Dir::East, Dir::North, Dir::West, Dir::West, Dir::West])),
                    Path::Branch(VecDeque::from(vec![
                        Path::Segment(VecDeque::from(vec![Dir::North, Dir::East, Dir::East, Dir::East])),
                        Path::Sequence(VecDeque::from(vec![
                            Path::Segment(VecDeque::from(vec![Dir::South, Dir::South, Dir::East])),
                            Path::Branch(VecDeque::from(vec![
                                Path::Segment(VecDeque::from(vec![Dir::East, Dir::East])),
                                Path::Segment(VecDeque::from(vec![Dir::North]))
                            ]))
                        ])),

                    ]))
                ])
            ),
            Path::from("^ENWWW(NEEE|SSE(EE|N))$")
        );
    }

}