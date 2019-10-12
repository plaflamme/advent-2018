use regex::Regex;
use std::str::FromStr;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::fmt::{Display, Formatter, Error};
use termion::color;

#[derive(PartialEq)]
enum Turn {
    GameOver,
    NoPoints,
    Points(u32)
}

#[derive(Clone, Debug)]
struct Marble {
    value: u32, // the marble's value
    idx: usize, // vector index of this marble
    next: usize, // vector index of the next marble
    prev: usize // vector index of the previous
}

struct Board {
    current_marble: Marble,
    marbles: Vec<Marble>,
    remaining_marbles: BinaryHeap<Reverse<u32>>
}

impl Board {

    fn remove_current(&mut self) -> u32 {
        let value = self.current_marble.value;

        {
            let mut prev = self.marbles.get_mut(self.current_marble.prev).expect("no previous marble");
            prev.next = self.current_marble.next;
        }
        {
            let mut next = self.marbles.get_mut(self.current_marble.next).expect("no next marble");
            next.prev = self.current_marble.prev;
            self.current_marble = next.clone();
        }


        value
    }

    fn insert(&mut self, value: u32) {
        let new_idx = self.marbles.len(); // 1
        let new_marble = Marble { value, idx: new_idx, prev: self.current_marble.prev, next: self.current_marble.idx };
        self.marbles.push(new_marble.clone());

        {
            let mut prev = self.marbles.get_mut(self.current_marble.prev).expect("no previous marble");
            if new_idx == 1 {
                prev.prev = new_idx;
            }
            prev.next = new_idx; // 1
        }

        {
            let mut next = self.marbles.get_mut(self.current_marble.idx).expect("no previous marble");
            next.prev = new_idx;
        }

        self.current_marble = new_marble.clone();
    }

    fn turn(&mut self) -> Turn {
        match self.remaining_marbles.pop() {
            None => Turn::GameOver,
            Some(Reverse(value)) => {
                if value % 23 == 0 {
                    for _ in 0..7 {
                        self.current_marble = self.marbles.get(self.current_marble.prev).expect("no more marbles").clone();
                    }
                    let score = self.remove_current();
                    Turn::Points(value + score)
                }
                else {
                    for _ in 0..2 {
                        self.current_marble = self.marbles.get(self.current_marble.next).expect("no more marbles").clone();
                    }
                    self.insert(value);
                    Turn::NoPoints
                }
            }
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut index = 0;
        while {
            let marble = self.marbles.get(index).expect("no marbles");
            if index == self.current_marble.idx {
                write!(f, "{}({}){}", color::Fg(color::LightCyan), marble.value, color::Fg(color::Reset))?;
            } else {
                write!(f, " {} ", marble.value)?;
            }
            index = marble.next;
            index != 0 as usize
        } {}
        Ok(())
    }
}

struct Game {
    current_player: usize,
    scores: Vec<u32>,
    board: Board
}

impl Game {

    fn new(n_players: u32, highest_marble: u32) -> Game {
        let mut remaining_marbles = BinaryHeap::new();
        for m in 1..=highest_marble {
            remaining_marbles.push(Reverse(m));
        }

        let mut marbles = Vec::new();
        let first_marble = Marble { value: 0, idx: 0, next: 0, prev: 0 };
        marbles.push(first_marble.clone());
        let mut board = Board { current_marble: first_marble, marbles, remaining_marbles };

        let mut scores = Vec::new();
        (0..n_players).for_each(|_| scores.push(0));

        board.turn(); // play the first turn
        Game { current_player: 0, scores: scores, board: board }
    }

    // Rust doesn't have tail call optimization, so this is a loop instead of a recursive call.
    fn play(&mut self) -> Vec<u32> {
//        println!("{}", self.board);
//        println!("{:?}", self.board.marbles);
        self.current_player = (self.current_player + 1) % self.scores.len();
        let mut result = self.board.turn();
        while result != Turn::GameOver {
//            println!("{}", self.board);
//            println!("{:?}", self.board.marbles);
            if let Turn::Points(pts) = result {
                let score = self.scores.get_mut(self.current_player).expect("unexpected missing score");
                *score = *score + pts;
            }
            self.current_player = (self.current_player + 1) % self.scores.len();
            result = self.board.turn();
        }
        self.scores.clone()
    }
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    // 411 players; last marble is worth 71058 points
    let re = Regex::new(r"^(\d+) players; last marble is worth (\d+) points$").unwrap();
    let caps = re.captures(&input).expect("invalid input");
    let n_players = u32::from_str(&caps[1]).expect("invalid number of players");
    let highest_marble =  u32::from_str(&caps[2]).expect("invalid number of marbles");

    Box::new(Puzzle9 { n_players, highest_marble })
}
struct Puzzle9 {
    n_players: u32,
    highest_marble: u32
}

impl crate::Puzzle for Puzzle9 {
    fn part1(&self) -> String {
        let mut game = Game::new(self.n_players, self.highest_marble);
        let mut scores = game.play();
        scores.sort();
        scores.last().expect("no players").to_string()
    }

    fn part2(&self) -> String {
        // TODO: This problem can probably be solved with math instead of data structures... This is slow.
        let mut game = Game::new(self.n_players, self.highest_marble * 100);
        let mut scores = game.play();
        scores.sort();
        scores.last().expect("no players").to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Puzzle;

    #[test]
    fn part1() {
        assert_eq!(Puzzle9 { n_players: 9, highest_marble: 25 }.part1(), "32");
        assert_eq!(Puzzle9 { n_players: 10, highest_marble: 1618 }.part1(), "8317");
        assert_eq!(Puzzle9 { n_players: 13, highest_marble: 7999 }.part1(), "146373");
        assert_eq!(Puzzle9 { n_players: 17, highest_marble: 1104 }.part1(), "2764");
        assert_eq!(Puzzle9 { n_players: 21, highest_marble: 6111 }.part1(), "54718");
        assert_eq!(Puzzle9 { n_players: 30, highest_marble: 5807 }.part1(), "37305");
    }
}