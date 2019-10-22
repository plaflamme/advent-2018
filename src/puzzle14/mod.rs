use std::str::FromStr;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
struct Scoreboard {
    recipe_scores: VecDeque<u8>,
    current_recipes: (usize, usize),
}

impl Scoreboard {

    fn new() -> Self {
        Scoreboard::from(vec![3,7], (0,1))
    }

    fn from(recipe_scores: Vec<u8>, current_recipes: (usize, usize)) -> Self {
        Scoreboard { recipe_scores: VecDeque::from(recipe_scores), current_recipes }
    }

    fn score_of(&self, recipe: usize) -> u8 {
        *self.recipe_scores.get(recipe).expect(&format!("missing recipe at {}", recipe))
    }

    fn step(&mut self) {
        let a = self.score_of(self.current_recipes.0);
        let b = self.score_of(self.current_recipes.1);

        let new_score = a + b;
        if new_score >= 10 {
            self.recipe_scores.push_back((new_score / 10) % 10)
        }
        self.recipe_scores.push_back(new_score % 10);

        self.current_recipes.0 = (self.current_recipes.0 + a as usize + 1) % self.recipe_scores.len();
        self.current_recipes.1 = (self.current_recipes.1 + b as usize + 1) % self.recipe_scores.len();
    }

    fn solve_after_recipes(&mut self, recipes: usize) -> String {
        while self.recipe_scores.len() < (recipes + 10) {
            self.step()
        }

        self.recipe_scores
            .iter()
            .skip(recipes)
            .take(10)
            .map(|score| score.to_string())
            .collect::<String>()
    }

    fn matches(&self, pattern: &Vec<u8>, offset: usize) -> bool {
        if (self.recipe_scores.len() - offset) < pattern.len() { false } else {
            for i in 0..pattern.len() {
                let p = pattern.get(i).unwrap();
                let recipe_offset_from_last = pattern.len() - i + offset;
                let c = self.recipe_scores.get(self.recipe_scores.len() - recipe_offset_from_last).unwrap();
                if *p != *c {
                    return false;
                }
            }
            true
        }
    }

    fn solve_part_2(&mut self, pattern: &Vec<u8>) -> usize {
        while self.matches(pattern, 0) == false && self.matches(pattern, 1) == false {
            self.step();
        }

        if self.matches(pattern, 0) {
            self.recipe_scores.len() - pattern.len()
        } else {
            self.recipe_scores.len() - pattern.len() - 1
        }

    }
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    let value = usize::from_str(&input.trim()).expect(&format!("invalid input {}", input));
    let digits = input.trim().chars().map(|x| u8::from_str(&x.to_string()).expect("not a digit")).collect::<Vec<_>>();
    Box::new(Puzzle14 { value, digits })
}

struct Puzzle14 {
    value: usize,
    digits: Vec<u8>
}

impl crate::Puzzle for Puzzle14 {
    fn part1(&self) -> String {
        let mut scoreboard = Scoreboard::new();
        scoreboard.solve_after_recipes(self.value)
    }

    fn part2(&self) -> String {
        let mut scoreboard = Scoreboard::new();
        scoreboard.solve_part_2(&self.digits).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scoreboard() {
        let mut scoreboard =Scoreboard::new();
        scoreboard.step();
        assert_eq!(Scoreboard::from(vec![3,7,1,0], (0,1)), scoreboard);
        scoreboard.step();
        assert_eq!(Scoreboard::from(vec![3,7,1,0,1,0], (4,3)), scoreboard);
        scoreboard.step();
        assert_eq!(Scoreboard::from(vec![3,7,1,0,1,0,1], (6,4)), scoreboard);
        scoreboard.step();
        assert_eq!(Scoreboard::from(vec![3,7,1,0,1,0,1,2], (0,6)), scoreboard);
        scoreboard.step();
        assert_eq!(Scoreboard::from(vec![3,7,1,0,1,0,1,2,4], (4,8)), scoreboard);
        scoreboard.step();
        assert_eq!(Scoreboard::from(vec![3,7,1,0,1,0,1,2,4,5], (6,3)), scoreboard);
        scoreboard.step();
        assert_eq!(Scoreboard::from(vec![3,7,1,0,1,0,1,2,4,5,1], (8,4)), scoreboard);
        scoreboard.step();
        assert_eq!(Scoreboard::from(vec![3,7,1,0,1,0,1,2,4,5,1,5], (1,6)), scoreboard);
    }

    #[test]
    fn part1() {
        let mut scoreboard = Scoreboard::from(vec![3,7], (0,1));
        assert_eq!("5158916779", scoreboard.solve_after_recipes(9));

        let mut scoreboard = Scoreboard::from(vec![3,7], (0,1));
        assert_eq!("0124515891", scoreboard.solve_after_recipes(5));

        let mut scoreboard = Scoreboard::from(vec![3,7], (0,1));
        assert_eq!("9251071085", scoreboard.solve_after_recipes(18));

        let mut scoreboard = Scoreboard::from(vec![3,7], (0,1));
        assert_eq!("5941429882", scoreboard.solve_after_recipes(2018));
    }

    #[test]
    fn part2() {
        assert_eq!(9, Scoreboard::new().solve_part_2(&vec![5,1,5,8,9]));
        assert_eq!(5, Scoreboard::new().solve_part_2(&vec![0,1,2,4,5]));
        assert_eq!(18, Scoreboard::new().solve_part_2(&vec![9,2,5,1,0]));
        assert_eq!(2018, Scoreboard::new().solve_part_2(&vec![5,9,4,1,4]));
    }
}