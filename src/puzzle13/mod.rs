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

                        '\\'  => (None, Track::TurnBack),
                        '/'  => (None, Track::TurnFwd),

                        '+'  => (None, Track::Intersection),
                        c => panic!("unexpected input char {}", c)
                    };

                    let pt = Pt::new(x as u16, y as u16);
                    tracks.insert(pt.clone(), track);
                    match cart {
                        None => (),
                        Some(dir) => carts.push(Cart { pt: pt.clone(), dir, next_intersection: IntersectionStep::Left })
                    };
                });
        });

    Puzzle13 { tracks: Tracks { values: tracks }, carts }
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(parse(input))
}

#[derive(Debug, Clone)]
enum Track {
    NS, // |
    EW, // -

    TurnFwd, // /
    TurnBack, // \

    Intersection // +
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone)]
enum IntersectionStep {
    Left,
    Straight,
    Right
}

impl IntersectionStep {
    fn next(&self) -> Self {
        match self {
            IntersectionStep::Left => IntersectionStep::Straight,
            IntersectionStep::Straight => IntersectionStep::Right,
            IntersectionStep::Right => IntersectionStep::Left,
        }
    }

    fn apply(&self, dir: &Direction) -> Direction {
        match self {
            IntersectionStep::Straight => *dir,
            IntersectionStep::Left => {
                match dir {
                    Direction::North => Direction::West,
                    Direction::East => Direction::North,
                    Direction::South => Direction::East,
                    Direction::West => Direction::South,
                }
            },
            IntersectionStep::Right =>
                match dir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                },
        }
    }
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
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
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone)]
struct Cart { pt: Pt, dir: Direction, next_intersection: IntersectionStep }

impl Cart {
    fn advance(&mut self, tracks: &Tracks) {
        self.pt.move_towards(&self.dir);

        let new_track = tracks.values.get(&self.pt).expect(&format!("missing track at {:?}", self.pt));

        match new_track {
            Track::TurnFwd => {
                self.dir = match self.dir {
                    Direction::North => Direction::East,
                    Direction::South => Direction::West,
                    Direction::East => Direction::North,
                    Direction::West => Direction::South
                }
            },
            Track::TurnBack => {
                self.dir = match self.dir {
                    Direction::North => Direction::West,
                    Direction::South => Direction::East,
                    Direction::East => Direction::South,
                    Direction::West => Direction::North
                }
            },
            Track::Intersection => {
                self.dir = self.next_intersection.apply(&self.dir);
                self.next_intersection = self.next_intersection.next();
            }
            _ => ()
        };

    }
}

#[derive(Clone)]
struct Tracks { values: HashMap<Pt, Track> }

struct Puzzle13 {
    tracks: Tracks,
    carts: Vec<Cart>
}

impl Puzzle13 {
    fn tick(&mut self) -> Vec<Pt> {
        let mut new_positions = HashSet::new();
        let mut collisions = Vec::new();
        {
            let mut carts_to_move = self.carts.iter_mut().collect::<BinaryHeap<_>>();
            while let Some(cart) = carts_to_move.pop() {
                cart.advance(&self.tracks);
                if !new_positions.insert(&cart.pt) {
                    collisions.push(cart.pt);
                }
            }
        }
        self.carts.retain(|cart| !collisions.contains(&cart.pt));
        collisions
    }
}

impl crate::Puzzle for Puzzle13 {
    fn part1(&self) -> String {
        let mut pzl = Puzzle13 { tracks: self.tracks.clone(), carts: self.carts.clone() };

        let mut collision = None;
        while collision.is_none() {
            // this is weird because pzl gets borrowed multiple times otherwise
            collision = pzl.tick().get(0).map(|pt|*pt);
        }
        format!("First collision occurs at {:?}", collision.expect(""))
    }

    fn part2(&self) -> String {
        let mut pzl = Puzzle13 { tracks: self.tracks.clone(), carts: self.carts.clone() };

        while pzl.carts.len() > 1 {
            pzl.tick();
        }
        format!("Last remaining cart is at {:?}", pzl.carts.get(0).expect("no more carts"))
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

    const EXAMPLE2: &str = r#"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#;

    #[test]
    fn test_parse() {
        let pzl13 = parse(EXAMPLE.to_owned());
        assert_eq!(vec![Cart{ pt: Pt::new(2,0), dir: Direction::East, next_intersection: IntersectionStep::Left}, Cart{ pt: Pt::new(9,3), dir: Direction::South, next_intersection: IntersectionStep::Left}], pzl13.carts);
    }

    #[test]
    fn test_cart() {
        let mut pzl13 = parse(EXAMPLE.to_owned());
        let cart0 = pzl13.carts.get_mut(0).expect("missing cart");

        cart0.advance(&pzl13.tracks);
        assert_eq!(Pt::new(3,0), cart0.pt);
        assert_eq!(Direction::East, cart0.dir);

        cart0.advance(&pzl13.tracks);
        assert_eq!(Pt::new(4,0), cart0.pt);
        assert_eq!(Direction::South, cart0.dir);

        let cart1 = pzl13.carts.get_mut(1).expect("missing cart");

        cart1.advance(&pzl13.tracks);
        assert_eq!(Pt::new(9,4), cart1.pt);
        assert_eq!(Direction::East, cart1.dir);
        assert_eq!(IntersectionStep::Straight, cart1.next_intersection);

        cart1.advance(&pzl13.tracks);
        assert_eq!(Pt::new(10,4), cart1.pt);
        assert_eq!(Direction::East, cart1.dir);
        assert_eq!(IntersectionStep::Straight, cart1.next_intersection);

    }

    #[test]
    fn test_part1() {
        let mut pzl13 = parse(EXAMPLE.to_owned());
        for _ in 0..13 {
            assert_eq!(0, pzl13.tick().len());
        }
        assert_eq!(vec![Pt::new(7,3)], pzl13.tick())
    }


    #[test]
    fn test_part2() {
        let mut pzl13 = parse(EXAMPLE2.to_owned());
        while pzl13.carts.len() > 1 {
            pzl13.tick();
        }
        assert_eq!(Pt::new(6,4), pzl13.carts.get(0).expect("no more carts").pt);
    }
}