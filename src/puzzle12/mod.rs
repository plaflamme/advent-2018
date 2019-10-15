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

        Rule { pattern: pattern.clone(), produces_plant}
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
        let rules = &self.growing_rules();
        for _ in 0..20 {
            gen = gen.grow(rules);
        }
        gen.plant_containing_pots().to_string()
    }

    fn part2(&self) -> String {
        unimplemented!()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Pot {
    index: i16,
    has_plant: bool
}

struct Generation {
    gen: u16,
    state: Vec<Pot>
}

impl Generation {

    fn new(state: &str) -> Self {
        let s = state.chars().enumerate().map(|(idx,c)| {
            let has_plant = match c {
                '.' => false,
                '#' => true,
                _ => panic!("invalid pattern")
            };

            Pot { index: idx as i16, has_plant }
        }).collect::<Vec<_>>();

        Generation { gen: 0, state : s }
    }

    fn grow(&self, rules: &HashSet<Vec<bool>>) -> Generation {
        let mut gen_state = self.state.clone();
        if let Some(first) = self.state.iter().next() {
            gen_state.insert(0, Pot { index: first.index - 1, has_plant: false });
            gen_state.insert(0, Pot { index: first.index - 2, has_plant: false });
        }
        if let Some(last) = self.state.iter().rev().next() {
            gen_state.push(Pot { index: last.index + 1, has_plant: false });
            gen_state.push(Pot { index: last.index + 2, has_plant: false });
        }

        let next_gen = gen_state.iter()
            .enumerate()
            .map(|(idx, pot)| {
                let start = idx as i16 - 2;
                let end = idx as i16 + 2;
                let mut state = Vec::new();
                for other in start..=end {
                    if other < 0 {
                        state.push(false)
                    } else {
                        match gen_state.get(other as usize) {
                            None => state.push(false),
                            Some(s) => state.push(s.has_plant)
                        }
                    }
                }
                (pot, state)
            })
            .map(|(pot, pot_state)| {
                if rules.contains(&pot_state) {
                    Pot { index: pot.index, has_plant: true }
                } else {
                    Pot { index: pot.index, has_plant: false }
                }
            })
            .skip_while(|pot| !pot.has_plant)
            .collect::<Vec<_>>();

        Generation { gen: self.gen + 1, state: next_gen }
    }

    fn plant_containing_pots(&self) -> i16 {
        self.state.iter().filter_map(|pot| if pot.has_plant { Some(pot.index) } else { None }).sum()
    }
}

impl Display for Generation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let plants: String = self.state.iter().map(|p| {
            match p.has_plant {
                true => '#',
                false => '.'
            }
        }).collect();
        write!(f, "{}: {}", self.gen, plants)
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
        let pzl = parse(EXAMPLE.to_owned());
        let gen0 = Generation::new(&pzl.initial_state);
        let grew = gen0.grow(&pzl.growing_rules());
        let actual = grew.state.iter().filter_map(|pot| if pot.has_plant { Some(pot.index) } else { None }).collect::<Vec<_>>();
        assert_eq!(vec![0, 4, 9, 15, 18, 21, 24], actual);
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