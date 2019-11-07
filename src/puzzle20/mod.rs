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

struct Map {
    locations: HashMap<Pt, Loc>,
    shortest_path: HashMap<Pt, u32>,
    current_pos: Pt
}

impl Map {
    fn new() -> Self {
        let mut locations = HashMap::new();
        let current_pos = Pt::new(0,0);
        locations.insert(current_pos, Loc::Room);
        current_pos.neighbours().iter().for_each(|x| { locations.insert(x.clone(), Loc::Unk); });
        Map { locations, shortest_path: HashMap::new(), current_pos }
    }

    fn follow_segment(&mut self, directions: &VecDeque<Dir>, length: u32) -> u32 {
        let mut remains = directions.clone();
        match remains.pop_front() {
            None => length,
            Some(dir) => {
                let new_length = length + 1;
                let door_pt = self.current_pos.at(dir);
                self.locations.insert(door_pt, Loc::Door);
                self.current_pos = door_pt.at(dir);
                // TODO: record shortest path
                match self.shortest_path.get(&self.current_pos) {
                    None => self.shortest_path.insert(self.current_pos, new_length),
                    Some(s) if s > &length => self.shortest_path.insert(self.current_pos, new_length),
                    _ => None
                };
                self.locations.insert(self.current_pos, Loc::Room);
                self.current_pos.neighbours().iter().for_each(|pt| {
                    if !self.locations.contains_key(pt) {
                        self.locations.insert(*pt, Loc::Unk);
                    }
                });
                self.follow_segment(&remains, new_length)
            }
        }
    }

    fn follow_path(&mut self, path: &Path, length: u32) -> u32 {
        match path {
            Path::Noop => length,
            Path::Segment(directions) => {
                self.follow_segment(&directions, length)
            },
            Path::Sequence(paths) => {
                paths.iter().fold(length, |accumulated_length, path| {
                    self.follow_path(path, accumulated_length)
                })
            },
            Path::Branch(paths) => {
                let original_pos = self.current_pos;
                let mut shortest_path = std::u32::MAX;
                for path in paths {
                    shortest_path = shortest_path.min(self.follow_path(path, length));
                    self.current_pos = original_pos;
                }
                shortest_path
            }
        }
    }

    fn follow(&mut self, path: &Path) {
        self.follow_path(path, 0);
        // set all remaining unknown locations to Wall
        self.locations.iter_mut().for_each(|(_, loc)| {
            if *loc == Loc::Unk {
                *loc = Loc::Wall;
            }
        });
    }

    fn part1_solution(&self) -> u32 {
        let mut paths = self.shortest_path.values().collect::<Vec<_>>();
        paths.sort();
        **paths.last().unwrap()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let keys = self.locations.keys().collect::<Vec<_>>();
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

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle20 { path: Path::from(&input) })
}

struct Puzzle20 {
    path: Path
}

impl crate::Puzzle for Puzzle20 {
    // The solution is 4180
    fn part1(&self) -> String {
        println!("{:?}", self.path);
        let mut map = Map::new();
        map.follow(&self.path);
        println!("{}", map);
        map.part1_solution().to_string()
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
            map.follow(&Path::from(path_str));
            let sol = map.part1_solution();

            assert_eq!(expected_map.replace("-", "|"), format!("{}", map));
            assert_eq!(*expected_solution, sol);
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