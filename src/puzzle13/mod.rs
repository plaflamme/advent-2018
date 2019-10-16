use std::collections::{HashMap, BinaryHeap, HashSet};

fn parse(input: String) -> Puzzle13 {
    let mut tracks = HashMap::new();
    let mut carts = Vec::new();
    input.lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| !c.is_whitespace())
                .for_each(|(x, char)| {
                    let (cart, track) = match char {
                        '|' => (None, Track::NS),
                        '^' => (Some(Direction::North), Track::NS),
                        'v' => (Some(Direction::South), Track::NS),

                        '-' => (None, Track::EW),
                        '>' => (Some(Direction::East), Track::EW),
                        '<' => (Some(Direction::West), Track::EW),

                        '\\'  => (None, Track::Turn),
                        '/'  => (None, Track::Turn),

                        '+'  => (None, Track::Intersection),
                        c => panic!("unexpected input char {}", c)
                    };

                    let pt = Pt::new(x as u16, y as u16);
                    tracks.insert(pt.clone(), track);
                    match cart {
                        None => (),
                        Some(dir) => carts.push(Cart { pt: pt.clone(), dir })
                    };
                });
        });

    Puzzle13 { tracks: Tracks { values: tracks }, carts }
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(parse(input))
}

enum Track {
    NS, // |
    EW, // -

    Turn, // / or \ but requires looking around this tile to know where to re-orient to

    Intersection // +
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Clone)]
struct Pt { y: u16, x: u16 } // y comes first for ordering
impl Pt {
    fn new(x: u16, y: u16) -> Self {
        Pt { y: y, x: x }
    }

    fn move_towards(&mut self, d: &Direction) {
        match d {
            Direction::North => self.y -= 1,
            Direction::South => self.y += 1,
            Direction::East => self.x += 1,
            Direction::West => self.x -= 1
        };
    }

    fn peek_towards(&self, d: &Direction) -> Option<Self> {
        match d {
            Direction::North => if self.y > 0 { Some(Pt::new(self.x, self.y - 1)) } else { None },
            Direction::South => Some(Pt::new(self.x, self.y + 1)),
            Direction::East => Some(Pt::new(self.x + 1, self.y)),
            Direction::West => if self.x > 0 { Some(Pt::new(self.x - 1, self.y)) } else { None }
        }
    }
}
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Cart { pt: Pt, dir: Direction }

impl Cart {
    fn advance(&mut self, tracks: &Tracks) {
        self.pt.move_towards(&self.dir);

        let new_track = tracks.values.get(&self.pt).expect(&format!("missing track at {:?}", self.pt));

        match new_track {
            Track::Turn => {
                let new_dir = match self.dir {
                    Direction::East | Direction::West => {
                        let north_track = self.pt.peek_towards(&Direction::North).and_then(|x| tracks.values.get(&x));
                        let south_track = self.pt.peek_towards(&Direction::South).and_then(|x| tracks.values.get(&x));

                        // only one of them should be NS
                        match (north_track, south_track) {
                            (Some(Track::NS), Some(Track::NS)) => panic!(format!("Invalid turn at {:?}. Both north and south tracks are possible destinations.", self.pt)),
                            (Some(Track::NS), _) => Direction::North,
                            (_, Some(Track::NS)) => Direction::South,
                            _ => panic!("invalid track!")
                        }
                    },
                    Direction::North | Direction::South => {
                        let east_track = self.pt.peek_towards(&Direction::East).and_then(|x| tracks.values.get(&x));
                        let west_track = self.pt.peek_towards(&Direction::West).and_then(|x| tracks.values.get(&x));

                        // only one of them should be EW
                        match (east_track, west_track) {
                            (Some(Track::EW), Some(Track::EW)) => panic!(format!("Invalid turn at {:?}. Both east and west tracks are possible destinations.", self.pt)),
                            (Some(Track::EW), _) => Direction::East,
                            (_, Some(Track::EW)) => Direction::West,
                            _ => panic!("invalid track!")
                        }
                    }
                };

                // TODO: there was something else to do here...
                self.dir = new_dir;
            },
            Track::Intersection => unimplemented!(),
            _ => ()
        };

    }
}

struct Tracks { values: HashMap<Pt, Track> }

struct Puzzle13 {
    tracks: Tracks,
    carts: Vec<Cart>
}

impl Puzzle13 {
    fn tick(&mut self) -> Vec<&Pt> {
        let mut new_positions = HashSet::new();
        let mut collisions = Vec::new();
        let mut carts_to_move = self.carts.iter_mut().collect::<BinaryHeap<_>>();
        while let Some(cart) = carts_to_move.pop() {
            cart.advance(&self.tracks);
            if !new_positions.insert(&cart.pt) {
                collisions.push(&cart.pt);
            }
        }
        collisions
    }
}

impl crate::Puzzle for Puzzle13 {
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

    const EXAMPLE: &str = r#"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/"#;

    #[test]
    fn test_parse() {
        let pzl13 = parse(EXAMPLE.to_owned());
        assert_eq!(vec![Cart{ pt: Pt::new(2,0), dir: Direction::East}, Cart{ pt: Pt::new(9,3), dir: Direction::South}], pzl13.carts);
    }

    #[test]
    fn test_cart() {
        let mut pzl13 = parse(EXAMPLE.to_owned());
        let mut cart0 = pzl13.carts.get_mut(0).expect("missing cart");

        cart0.advance(&pzl13.tracks);
        assert_eq!(Pt::new(3,0), cart0.pt);
        assert_eq!(Direction::East, cart0.dir);

        cart0.advance(&pzl13.tracks);
        assert_eq!(Pt::new(4,0), cart0.pt);
        assert_eq!(Direction::South, cart0.dir);
    }

    #[test]
    fn test_collision() {
        let mut pzl13 = parse(EXAMPLE.to_owned());
        assert_eq!(0, pzl13.tick().len());
    }
}