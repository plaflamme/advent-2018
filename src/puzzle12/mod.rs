use regex::Regex;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};
use std::collections::HashSet;

fn parse(input: String) -> Puzzle12 {
    let mut lines = input.lines();
    let state_line = lines.next().expect("empty input");

    let re = Regex::new(r"^initial state: ([.#]+)$").unwrap();
    let captures = re.captures(state_line).expect("invalid input");
    let initial_state = captures[1].to_owned();

    lines.next();

    Puzzle12 { initial_state, rules: lines.map(|x| Rule::from_str(x).expect("invalid input")).collect::<Vec<_>>() }
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(parse(input))
}

#[derive(PartialEq, Eq, Debug)]
struct Rule {
    pattern: Vec<bool>,
    produces_plant: bool
}

impl Rule {
    fn new(str_pattern: &str, produces_plant: bool) -> Self {
        let pattern = str_pattern.chars().map(|c| {
            match c {
                '.' => false,
                '#' => true,
                _ => panic!("invalid pattern")
            }
        }).collect::<Vec<_>>();

        Rule { pattern: pattern.clone(), produces_plant }
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^([.#]+) => ([.#])$").unwrap();
        let captures = re.captures(s).expect("invalid input");
        let pattern = &captures[1];
        let produces_plant = match captures[2].chars().next() {
            None => panic!("missing result"),
            Some('.') => false,
            Some('#') => true,
            Some(_) => panic!("invalid result")
        };
        Ok(Rule::new(pattern, produces_plant))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Puzzle12 {
    initial_state: String,
    rules: Vec<Rule>
}

impl Puzzle12 {
    fn growing_rules(&self) -> HashSet<Vec<bool>> {
        self.rules.iter().filter(|x|x.produces_plant).map(|r|r.pattern.clone()).collect::<HashSet<_>>()
    }
}

impl crate::Puzzle for Puzzle12 {
    fn part1(&self) -> String {
        let mut gen = Generation::new(&self.initial_state);
        println!("{}", gen);
        let rules = &self.growing_rules();
        for _ in 0..20 {
            gen = gen.grow(rules);
            println!("{}", gen);
        }
        gen.plant_containing_pots().to_string()
    }

    fn part2(&self) -> String {
        let mut gen = Generation::new(&self.initial_state);
        let rules = &self.growing_rules();
        for _ in 0..(50000000000 as u64) {
            gen = gen.grow(rules);
        }
        gen.plant_containing_pots().to_string()
    }
}

struct Generation {
    gen: u16,
    min: i32,
    max: i32,
    state: HashSet<i32>
}

impl Generation {

    fn new(state: &str) -> Self {
        let s = state.chars().enumerate()
            .filter_map(|(idx,c)| {
                match c {
                    '.' => None,
                    '#' => Some(idx as i32),
                    _ => panic!("invalid pattern")
                }
            })
            .collect::<HashSet<_>>();
        let mut min = std::i32::MAX;
        let mut max = std::i32::MIN;
        s.iter().for_each(|v| {
            min = std::cmp::min(min, *v);
            max = std::cmp::max(max, *v);
        });
        Generation { gen: 0, min, max, state : s }
    }

    fn grow(&self, rules: &HashSet<Vec<bool>>) -> Generation {
        let mut min = std::i32::MAX;
        let mut max = std::i32::MIN;
        let mut new_state = HashSet::new();

        for plant_idx in (self.min-2)..=(self.max+2) {
            let mut plant_state = Vec::new();
            for other_idx in (plant_idx - 2)..=(plant_idx + 2) {
                plant_state.push(self.state.contains(&other_idx));
            }
            if rules.contains(&plant_state) {
                min = std::cmp::min(min, plant_idx);
                max = std::cmp::max(max, plant_idx);
                new_state.insert(plant_idx);
            }
        }
        Generation { gen: self.gen + 1, min, max, state: new_state }
    }

    fn plant_containing_pots(&self) -> i32 {
        self.state.iter().sum()
    }
}

impl Display for Generation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} ([{},{}]): ", self.gen, self.min, self.max)?;
        for idx in self.min..=self.max {
            let char = match self.state.contains(&idx) {
                true => '#',
                false => '.'
            };
            write!(f, "{}", char)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Puzzle;

    const EXAMPLE: &str = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    #[test]
    fn test_parse() {
        let puzzle = parse(EXAMPLE.to_owned());
        let rules = vec![
            Rule::new("...##", true),
            Rule::new("..#..", true),
            Rule::new(".#...", true),
            Rule::new(".#.#.", true),
            Rule::new(".#.##", true),
            Rule::new(".##..", true),
            Rule::new(".####", true),
            Rule::new("#.#.#", true),
            Rule::new("#.###", true),
            Rule::new("##.#.", true),
            Rule::new("##.##", true),
            Rule::new("###..", true),
            Rule::new("###.#", true),
            Rule::new("####.", true),
        ];
        let expected = Puzzle12 {
            initial_state: "#..#.#..##......###...###".to_owned(),
            rules
        };
        assert_eq!(expected, puzzle);
    }

    #[test]
    fn test_grow() {
        fn assert_gen(s: &Vec<i32>, g: &Generation) {
            assert_eq!(s.iter().map(|x|*x).collect::<HashSet<i32>>(), g.state);
        }
        let pzl = parse(EXAMPLE.to_owned());
        let gen0 = Generation::new(&pzl.initial_state);
        assert_gen(&vec![0, 3, 5, 8, 9, 16, 17, 18, 22, 23, 24], &gen0);

        let grew = gen0.grow(&pzl.growing_rules());
        assert_gen(&vec![0, 4, 9, 15, 18, 21, 24], &grew);

        let grew = grew.grow(&pzl.growing_rules());
        assert_gen(&vec![0, 1, 4, 5, 9, 10, 15, 18, 21, 24, 25], &grew);
    }

    #[test]
    fn part1() {
        let pzl = parse(EXAMPLE.to_owned());
        assert_eq!("325", pzl.part1());
    }
    #[test]
    fn part2() {
        unimplemented!()
    }
}