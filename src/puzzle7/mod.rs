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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Work {
    finish_at: u32, // This needs to come first for ordering
    step: char,
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

    fn cost(base_cost: u32, step: &char) -> u32 {
        *step as u32 - 'A' as u32 + 1 + base_cost
    }

    fn solve(&self, workers: u8, base_cost: u32) -> Solution {
        let mut steps = self.to_steps();
        let mut steps_ran: HashSet<char> = HashSet::new();
        let mut run_sequence: Vec<char> = Vec::new();
        let mut work_queue: BinaryHeap<Reverse<Work>> = BinaryHeap::new();
        let mut current_time: u32 = 0;

        // TODO: this is pretty disgusting, but here's what's going on
        //   we first check if anything in the work queue is done (items that have finish_at == current_time)
        //     anything done is added to steps_ran and run_sequence
        //   we prune the list of dependencies for the pending steps
        //   we check for new work (steps that have no dependencies pending)
        //     fill in the work queue to capacity with available work, marking when each work will finish
        //   prune the list of pending steps (remove then ones that are running)
        //   advance the time to when the top item of the work queue will finish
        //   repeat until there are no outstanding steps left and there are no items in the work queue
        while !steps.is_empty() || !work_queue.is_empty() {
            // check for finished work
            fn clear_finished_work(q: &mut BinaryHeap<Reverse<Work>>, now: u32, finished: &mut Vec<Work>) {
                match q.peek() {
                    None => (),
                    Some(peeked) =>
                        if peeked.0.finish_at == now {
                            finished.push(peeked.0);
                            q.pop();
                            clear_finished_work(q, now, finished);
                        }
                }
            }
            let mut finished = Vec::new();
            clear_finished_work(&mut work_queue, current_time, &mut finished);
            finished.iter().for_each(|work| {
                run_sequence.push(work.step);
                steps_ran.insert(work.step);
            });

            steps.iter_mut()
                .for_each(|x| {
                    x.depends_on.retain(|x| !steps_ran.contains(x) )
                });

            // find some new work
            let mut new_steps_to_run = steps.iter()
                .filter_map(|x| if x.depends_on.is_empty() { Some(Reverse(x.id)) } else { None } )
                .collect::<BinaryHeap<_>>();

            while work_queue.len() < workers as usize && !new_steps_to_run.is_empty() {
                let work = new_steps_to_run.pop().unwrap();
                let finish_at = current_time + Puzzle7::cost(base_cost, &work.0);
                work_queue.push(Reverse(Work { step: work.0, finish_at }));
            }

            // remove steps under work
            steps.retain(|step| work_queue.iter().find(|work| work.0.step == step.id).is_none());

            println!("{}", current_time);
            steps.iter().for_each(|x| println!("{:?}", x));
            work_queue.iter().for_each(|x| println!("{:?}", x));
            println!("{:?}", run_sequence);

            match work_queue.peek() {
                None => (),
                Some(Reverse(w)) => current_time = w.finish_at
            }
        }

        Solution { sequence: run_sequence, time: current_time }
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
        assert_eq!(solution.sequence.iter().collect::<String>(), "CABFDE");
        assert_eq!(solution.time, 15);
    }
}