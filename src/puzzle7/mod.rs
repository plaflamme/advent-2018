use std::str::FromStr;
use regex::Regex;

struct Dependency {
    step: char,
    before: char
}

impl FromStr for Dependency {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^Step (.) must be finished before step (.) can begin.$").unwrap();
        let caps = re.captures(s).expect("invalid date input");
        let step = caps[1].chars().next().unwrap();
        let before = caps[2].chars().next().unwrap();
        Ok(Dependency { step, before })
    }
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    let deps = input.lines().map(|line| Dependency::from_str(line).expect(format!("invalid line {}", line).as_str())).collect();
    Box::new(Puzzle7 { deps })
}

struct Puzzle7 {
    deps: Vec<Dependency>
}

impl crate::Puzzle for Puzzle7 {
    fn part1(&self) -> String {
        unimplemented!()
    }

    fn part2(&self) -> String {
        unimplemented!()
    }
}