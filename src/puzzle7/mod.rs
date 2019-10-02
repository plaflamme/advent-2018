use std::str::FromStr;
use regex::Regex;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Reverse;

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

#[derive(Debug, Clone)]
struct Step {
    id: char,
    depends_on: HashSet<char>
}

struct Solution {
    sequence: Vec<char>,
    time: u32
}

struct Puzzle7 {
    deps: Vec<Dependency>
}

impl Puzzle7 {
    fn to_steps(&self) -> Vec<Step> {
        let mut steps = HashMap::new();
        self.deps.iter().for_each(|x| {
            steps.entry(x.step).or_insert(Step { id: x.step, depends_on: HashSet::new() });
            let s = steps.entry(x.before).or_insert(Step {id: x.before, depends_on: HashSet::new() });
            s.depends_on.insert(x.step);
        });

        steps.values().cloned().collect()
    }

    fn solve(&self, workers: u8, cost: u32) -> Solution {
        let mut steps = self.to_steps();
        let mut steps_ran = HashSet::new();
        let mut run_sequence: Vec<char> = Vec::new();

        while !steps.is_empty() {
            steps.iter().for_each(|x|println!("{:?}", x));
            let mut new_steps_to_run = steps.iter()
                .filter_map(|x| if x.depends_on.is_empty() { Some(Reverse(x.id)) } else { None } )
                .collect::<BinaryHeap<_>>();
            new_steps_to_run.pop().iter().for_each(|step| {
                steps_ran.insert(step.0);
                run_sequence.push(step.0);
            });
            steps.retain(|x| !steps_ran.contains(&x.id));
            steps.iter_mut()
                .for_each(|x| {
                    x.depends_on.retain(|x| !steps_ran.contains(x) )
                });
            println!("{:?}", run_sequence);
        }

        Solution { sequence: run_sequence, time: 0 }
    }
}

impl crate::Puzzle for Puzzle7 {
    fn part1(&self) -> String {
        self.solve(1, 0).sequence.iter().collect()
    }

    fn part2(&self) -> String {
        self.solve(5, 60).time.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Puzzle;

    fn example_input() -> Vec<Dependency> {
        vec![
            Dependency {step: 'C', before: 'A'},
            Dependency {step: 'C', before: 'F'},
            Dependency {step: 'A', before: 'B'},
            Dependency {step: 'A', before: 'D'},
            Dependency {step: 'B', before: 'E'},
            Dependency {step: 'D', before: 'E'},
            Dependency {step: 'F', before: 'E'}
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(Puzzle7 { deps: example_input() }.part1(), "CABDFE");
    }

    #[test]
    fn test_part2() {
        let solution = Puzzle7 { deps: example_input() }.solve(2, 0);
        assert_eq!(solution.sequence.iter().collect::<String>(), "CABDFE");
        assert_eq!(solution.time, 15);
    }
}