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

    fn solve_part_2(&mut self, pattern: Vec<u8>) -> usize {

        unimplemented!()
    }
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle14 { seed: usize::from_str(&input.trim()).expect(&format!("invalid input {}", input)) })
}

struct Puzzle14 {
    seed: usize
}

impl crate::Puzzle for Puzzle14 {
    fn part1(&self) -> String {
        let mut scoreboard = Scoreboard::new();
        scoreboard.solve_after_recipes(self.seed)
    }

    fn part2(&self) -> String {
        unimplemented!()
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

    }
}