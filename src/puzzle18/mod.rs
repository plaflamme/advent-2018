use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy, Clone)]
struct Pt {
    x: i16,
    y: i16
}

impl Pt {

    fn new(x: i16, y: i16) -> Pt { Pt {x, y} }

    fn inbounds(&self, size: i16) -> bool {
        self.x >= 0 && self.x < size && self.y >= 0 && self.y < size
    }

    fn neighbours(&self) -> Vec<Pt> {
        let mut n = Vec::new();
        for x in self.x - 1..=self.x + 1 {
            for y in self.y - 1..=self.y + 1 {
                let pt = Pt::new(x,y);
                if &pt != self { n.push(pt) };
            }
        }
        n
    }
}

#[derive(PartialEq, Eq, Clone)]
enum Acre {
    Open,
    Trees,
    Yard
}

impl Display for Acre {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let c = match self {
            Acre::Open => ".",
            Acre::Trees => "|",
            Acre::Yard => "#",
        };
        write!(f, "{}", c)
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Outskirts {
    size: usize,
    acres: HashMap<Pt, Acre>
}

impl Outskirts {

    fn step(&self) -> Outskirts {
        let mut a = HashMap::new();

        for x in 0..self.size {
            for y in 0..self.size {
                let pt = Pt::new(x as i16 , y as i16 );
                let neighbours = pt.neighbours().iter()
                    .filter(|pt| pt.inbounds(self.size as i16))
                    .flat_map(|pt| self.acres.get(pt))
                    .collect::<Vec<_>>();
                match self.acres.get(&pt) {
                    None => panic!(format!("missing acre at {:?}", pt)),
                    Some(Acre::Open) => {
                        let trees = neighbours.iter().filter(|acre| ***acre == Acre::Trees).count();
                        let acre = if trees >= 3 { Acre::Trees } else { Acre::Open };
                        a.insert(pt, acre);
                    },
                    Some(Acre::Trees) => {
                        let yards = neighbours.iter().filter(|acre| ***acre == Acre::Yard).count();
                        let acre = if yards >= 3 { Acre::Yard } else { Acre::Trees };
                        a.insert(pt, acre);
                    },
                    Some(Acre::Yard) => {
                        let yards = neighbours.iter().filter(|acre| ***acre == Acre::Yard).count();
                        let trees = neighbours.iter().filter(|acre| ***acre == Acre::Trees).count();
                        let acre = if yards >= 1 && trees >= 1 { Acre::Yard } else { Acre::Open };
                        a.insert(pt, acre);
                    },
                }
            }
        }

        Outskirts { size: self.size, acres: a }
    }

    fn count(&self, a: &Acre) -> usize {
        self.acres.iter().filter(|(_, acre)| *acre == a).count()
    }
}

impl Display for Outskirts {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for y in 0..self.size {
            for x in 0..self.size {
                match self.acres.get(&Pt::new(x as i16,y as i16)) {
                    None => panic!(),
                    Some(acre) => write!(f, "{}", acre)?
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn parse(input: &str, size: usize) -> Outskirts {
    let mut acres = HashMap::new();
    input.lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let acre = match c {
                    '.' => Acre::Open,
                    '|' => Acre::Trees,
                    '#' => Acre::Yard,
                    _ => panic!(format!("unexpected char {}", c))
                };

                acres.insert(Pt::new(x as i16, y as i16), acre);
            })
        });

    Outskirts { size, acres }
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle18 { outskirts: parse(&input, 50) })
}

struct Puzzle18 {
    outskirts: Outskirts
}

impl crate::Puzzle for Puzzle18 {
    fn part1(&self) -> String {
        let mut outskirts = self.outskirts.clone();
        for _ in 0..10 {
            outskirts = outskirts.step();
        }
        (outskirts.count(&Acre::Yard) * outskirts.count(&Acre::Trees)).to_string()
    }

    fn part2(&self) -> String {
        let mut outskirts = self.outskirts.clone();
        let mut step = 0;
        let mut steps: Vec<(Outskirts, i32)> = Vec::new();
        let cycle_length = loop {
            outskirts = outskirts.step();
            step = step + 1;
            let found = steps.iter().find(|(other, _)| {
                *other == outskirts
            });

            match found {
                None => steps.push((outskirts.clone(), step)),
                Some((_, previous_step)) => {
                    let length = step - *previous_step;
                    println!("Found cycle after {} steps, it is {} steps long", step, length);
                    break length;
                }
            }
        };

        while step + cycle_length < 1000000000 {
            step += cycle_length;
        }

        for _ in 0..(1000000000-step) {
            outskirts = outskirts.step();
        }

        (outskirts.count(&Acre::Yard) * outskirts.count(&Acre::Trees)).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|."#;

    const ONE_MINUTE: &str = r#".......##.
......|###
.|..|...#.
..|#||...#
..##||.|#|
...#||||..
||...|||..
|||||.||.|
||||||||||
....||..|.
"#;

    const TEN_MINUTES: &str = r#".||##.....
||###.....
||##......
|##.....##
|##.....##
|##....##|
||##.####|
||#####|||
||||#|||||
||||||||||
"#;

    #[test]
    fn test() {
        let outskirts = parse(EXAMPLE, 10);
        let mut stepped = outskirts.step();
        assert_eq!(ONE_MINUTE, format!("{}", stepped));
        for _ in 1..10 {
            stepped = stepped.step();
        }
        assert_eq!(TEN_MINUTES, format!("{}", stepped));
    }
}