use std::str::FromStr;
use cached::cached;

fn power_level(x: u16, y: u16, serial_number: u16) -> i32 {
    let rack_id = x as i32 + 10;
    let power_level = rack_id * y as i32;
    let power_level = power_level + serial_number as i32;
    let power_level = power_level * rack_id;
    let power_level = power_level / 100 % 10;
    let power_level = power_level - 5;
    power_level
}

cached! {
    PWR_LEVELS;
    fn tile_power_level(x: u16, y: u16, side: u16, serial_number: u16) -> i32 = {
        if side == 1 {
            power_level(x, y, serial_number)
        } else {
            let mut squares = Vec::new();
            if side % 2 == 0 {
                let half = side / 2;
                squares.push((x,y,half));
                squares.push((x,y+half,half));
                squares.push((x+half,y,half));
                squares.push((x+half,y+half,half));
            } else {
                let part = side - 1;
                for i in 0..=part {
                    squares.push((x + part, y + i, 1));
                }
                for i in 0..part {
                    squares.push((x + i, y + part, 1));
                }

                squares.push((x,y,part));
            }

            let mut power = 0;
            for (x,y,side) in squares {
                power += tile_power_level(x,y,side,serial_number);
            }
            power
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Pt {
    x: u16,
    y: u16
}

struct Grid {
    serial_number: u16
}

impl Grid {
    fn new(serial_number: u16) -> Self {
        Grid { serial_number }
    }

    fn tile_power(&self, pt: &Pt, side: u16) -> i32 {
        tile_power_level(pt.x, pt.y, side, self.serial_number)
    }

    fn iter(&self, side: u16) -> Tile {
        Tile { side, next: Some(Pt { x: 1, y: 1 }) }
    }

    fn solve(&self, side: u16) -> Pt {
        self.iter(side)
            .max_by_key(|x| {
                self.tile_power(x, side)
            })
            .expect("empty grid")
    }

    fn solve_all(&self) -> (Pt, u16) {
        let mut max_power = -1000000;
        let mut winning_pt: Pt = Pt {x:1,y:1};
        let mut winning_side = 1;
        for side in 1..=300 {
            use cached::Cached;
            {
                let cache = PWR_LEVELS.lock().unwrap();
                println!("size -> {:?}", cache.cache_size());
                println!("hits -> {:?}", cache.cache_hits().unwrap());
                println!("misses -> {:?}", cache.cache_misses().unwrap());
            }

            let pt = self.solve(side);
            let power = self.tile_power(&pt, side);
            if power > max_power {
                max_power = power;
                winning_pt = pt;
                winning_side = side;
                println!("{:?} {}, {}", winning_pt, side, max_power);
            }
        }
        (winning_pt, winning_side)
    }
}

struct Tile {
    side: u16,
    next: Option<Pt>
}

impl Iterator for Tile {
    type Item = Pt;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.clone();
        self.next = match self.next {
            Some(Pt{x,y}) if x + self.side + 1 <= 300 => Some(Pt { x: x + 1, y }),
            Some(Pt{x:_,y}) if y + self.side + 1 <= 300 => Some(Pt { x: 1, y: y + 1 }),
            _ => None
        };
        next
    }
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle11 { serial_number: u16::from_str(input.trim()).expect(&format!("invalid seed {}", input)) })
}

struct Puzzle11 {
    serial_number: u16
}

impl crate::Puzzle for Puzzle11 {
    fn part1(&self) -> String {
        let grid = Grid::new(self.serial_number);
        format!("{:?}", grid.solve(3))
    }

    fn part2(&self) -> String {
        let grid = Grid::new(self.serial_number);
        let (pt, side) = grid.solve_all();
        format!("{},{},{}", pt.x, pt.y, side)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(-5, power_level(122, 79, 57));
        assert_eq!(0, power_level(217, 196, 39));
        assert_eq!(4, power_level(101, 153, 71));
    }

    #[test]
    fn iterator() {
        let grid = Grid::new(18);
        assert_eq!(Some(Pt{x:1,y:1}), grid.iter(3).next());

        let tiles = grid.iter(3).collect::<Vec<_>>();
        assert_eq!(297*297, tiles.len());
    }

    #[test]
    fn part1() {
        let grid = Grid::new(18);
        let max_tile = grid.solve(3);
        assert_eq!(Pt{x:33,y:45}, max_tile);
        assert_eq!(29, grid.tile_power(&max_tile, 3));

        let grid = Grid::new(42);
        let max_tile = grid.solve(3);
        assert_eq!(Pt{x:21,y:61}, max_tile);
        assert_eq!(30, grid.tile_power(&max_tile, 3));
    }

    #[test]
    fn part2() {
        let grid = Grid::new(18);
        let (max_tile, side) = grid.solve_all();
        assert_eq!(16, side);
        assert_eq!(Pt{x:90,y:269}, max_tile);
        assert_eq!(29, grid.tile_power(&max_tile, side));

        let grid = Grid::new(42);
        let (max_tile, side) = grid.solve_all();
        assert_eq!(Pt{x:21,y:61}, max_tile);
        assert_eq!(30, grid.tile_power(&max_tile, side));
    }

}