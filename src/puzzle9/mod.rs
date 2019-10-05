use regex::Regex;
use std::str::FromStr;

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    let re = Regex::new(r"^(\d+) players; last marble is worth (\d+) points$").unwrap();
    let caps = re.captures(&input).expect("invalid input");
    let n_players = u32::from_str(&caps[1]).expect("invalid number of players");
    let n_marbles =  u32::from_str(&caps[2]).expect("invalid number of marbles");

    Box::new(Puzzle9 { n_players, n_marbles })
}
struct Puzzle9 {
    n_players: u32,
    n_marbles: u32
}

impl crate::Puzzle for Puzzle9 {
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

    #[test]
    fn part1() {
        assert_eq!(Puzzle9 { n_players: 9, n_marbles: 25 }.part1(), "33");
    }
}