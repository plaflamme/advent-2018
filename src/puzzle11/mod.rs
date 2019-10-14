use std::str::FromStr;
use std::collections::HashMap;

fn power_level(x: u16, y: u16, serial_number: u16) -> i32 {
    let rack_id = x as i32 + 10;
    let power_level = rack_id * y as i32;
    let power_level = power_level + serial_number as i32;
    let power_level = power_level * rack_id;
    let power_level = power_level / 100 % 10;
    let power_level = power_level - 5;
    power_level
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Pt {
    x: u16,
    y: u16
}

struct Grid {
    fuel_cells: HashMap<Pt, i32>
}

impl Grid {
    fn new(serial_number: u16) -> Self {
        let mut fuel_cells = HashMap::new();
        for y in 1..=300 {
            for x in 1..=300 {
                fuel_cells.insert(Pt {x,y}, power_level(x,y,serial_number));
            }
        }
        Grid { fuel_cells }
    }

    fn tile(&self, pt: &Pt) -> Vec<i32> {
        let mut cells = Vec::with_capacity(9);
        for y in pt.y..(pt.y + 3) {
            for x in pt.x..(pt.x + 3) {
                cells.push(*self.fuel_cells.get(&Pt{x,y} ).expect("invalid cell coordinate"))
            }
        }
        cells
    }

    fn iter(&self) -> Tile {
        Tile { next: Some(Pt { x: 1, y: 1 }) }
    }

    fn solve(&self) -> Pt {
        self.iter()
            .max_by_key(|x| {
                let total_power: i32 = self.tile(x).iter().sum();
                total_power
            })
            .expect("empty grid")
    }
}

struct Tile {
    next: Option<Pt>
}

impl Iterator for Tile {
    type Item = Pt;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.clone();
        self.next = match self.next {
            Some(Pt{x,y}) if x + 4 <= 300 => Some(Pt { x: x + 1, y }),
            Some(Pt{x:_,y}) if y + 4 <= 300 => Some(Pt { x: 1, y: y + 1 }),
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
        format!("{:?}", grid.solve())
    }

    fn part2(&self) -> String {
        unimplemented!()
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
        assert_eq!(Some(Pt{x:1,y:1}), grid.iter().next());

        let tiles = grid.iter().collect::<Vec<_>>();
        assert_eq!(297*297, tiles.len());
    }

    #[test]
    fn grid() {
        let grid = Grid::new(18);
        let max_tile = grid.solve();
        assert_eq!(Pt{x:33,y:45}, max_tile);
        assert_eq!(29, grid.tile(&Pt{x:33,y:45}).iter().sum());

        let grid = Grid::new(42);
        let max_tile = grid.solve();
        assert_eq!(Pt{x:21,y:61}, max_tile);
        assert_eq!(30, grid.tile(&Pt{x:21,y:61}).iter().sum());
    }

}