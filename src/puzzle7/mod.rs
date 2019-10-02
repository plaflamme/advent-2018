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
}

impl crate::Puzzle for Puzzle7 {
    fn part1(&self) -> String {
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
        run_sequence.iter().collect()
    }

    fn part2(&self) -> String {
        unimplemented!()
    }
}