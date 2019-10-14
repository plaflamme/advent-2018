use regex::Regex;
use std::str::FromStr;

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
    pattern: String,
    produces_plant: bool
}

impl Rule {
    fn new(pattern: &str, produces_plant: bool) -> Self {
        Rule { pattern: pattern.to_owned(), produces_plant}
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
        Ok(Rule { pattern: pattern.to_owned(), produces_plant })
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Puzzle12 {
    initial_state: String,
    rules: Vec<Rule>
}

impl crate::Puzzle for Puzzle12 {
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
    fn part1() {
        parse(EXAMPLE.to_owned());
        unimplemented!()
    }
    #[test]
    fn part2() {
        unimplemented!()
    }
}