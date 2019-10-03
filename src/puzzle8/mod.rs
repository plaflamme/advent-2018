use std::str::FromStr;

fn parse(input: String) -> Vec<u32> {
    input.split_ascii_whitespace().map(|x| u32::from_str(x).expect(format!("invalid number {}", x).as_str())).collect()
}
pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle8 { nodes: parse(input) })
}

struct Puzzle8 {
    nodes: Vec<u32>
}

impl crate::Puzzle for Puzzle8 {
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
    use crate::Puzzle;

    fn example_input() -> Puzzle8 {
        Puzzle8 { nodes: vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2] }
    }

    #[test]
    fn test_part1() {
        assert_eq!(example_input().part1(), "138");
    }

    #[test]
    fn test_part2() {}
}
