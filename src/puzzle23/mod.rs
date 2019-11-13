use std::str::FromStr;
use regex::Regex;

use z3;
use z3::ast::Ast;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Pt {
    x: i32,
    y: i32,
    z: i32
}

impl Pt {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Pt { x, y, z }
    }

    fn distance(&self, other: &Pt) -> u32 {
        ((self.x - other.x).abs() +
            (self.y - other.y).abs() +
            (self.z - other.z).abs()) as u32
    }
}

impl FromStr for Pt {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(',').collect();
        Ok(
            Pt {
                x: i32::from_str(parts[0])?,
                y: i32::from_str(parts[1])?,
                z: i32::from_str(parts[2])?,
            }
        )
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Nanobot {
    pos: Pt,
    signal_radius: u32
}

impl Nanobot {
    fn in_range(&self, other: &Nanobot) -> bool {
        self.pos.distance(&other.pos) <= self.signal_radius
    }
}

impl FromStr for Nanobot {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new("^pos=<(.+)>, r=(\\d+)$").unwrap();
        let caps = re.captures(s).expect(&format!("unmatched input: {}", s));

        Ok(
            Nanobot {
                pos: Pt::from_str(&caps[1])?,
                signal_radius: u32::from_str(&caps[2])?,
            }
        )
    }
}

fn dist<'ctx>(a: &z3::ast::Int<'ctx>, b: &z3::ast::Int<'ctx>, zero: &z3::ast::Int<'ctx>) -> z3::ast::Int<'ctx> {
    let diff = a.sub(&[&b]);
    let lt_zero = diff.lt(&zero);
    lt_zero.ite::<z3::ast::Int>(&diff.unary_minus(), &diff)
}

#[derive(Debug)]
struct Solution {
    bots_in_range: u32,
    optimal: Pt
}

fn solve(bots: &Vec<Nanobot>) -> Solution {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);

    let int = |i: i32| -> z3::ast::Int {
        z3::ast::Int::from_u64(&ctx, i as u64)
    };

    // define some constants
    let one = int(1);
    let zero = int(0);

    // define some variables. these will contain the value of the pt we're looking for
    let x = z3::ast::Int::new_const(&ctx, "x");
    let y = z3::ast::Int::new_const(&ctx, "y");
    let z = z3::ast::Int::new_const(&ctx, "z");

    // define a variable for each bot that captures whether (x,y,z) is in range of that bot
    let mut in_range = Vec::new();
    for i in 0..bots.len() {
        in_range.push(z3::ast::Int::new_const(&ctx, format!("in_range_{}", i)));
    }

    // create the optimizer
    let optimizer = z3::Optimize::new(&ctx);

    // for each bot, add a constraint in the solver that states that in_range[i] == 1 when the bot is in range of (x,y,z) and 0 otherwise.
    for (i,bot) in bots.iter().enumerate() {
        // compute the distance
        let bot_dist = dist(&x, &int(bot.pos.x), &zero).add(&[&dist(&y, &int(bot.pos.y), &zero), &dist(&z, &int(bot.pos.z), &zero)]);
        let sig = int(bot.signal_radius as i32);
        // 1 when in range, 0 otherwise
        let bot_in_range = bot_dist.le(&sig).ite(&one, &zero);

        // this adds a constraint in the optimizer
        optimizer.assert(&in_range[i]._eq(&bot_in_range));
    }

    // create the variable that counts the number of bots in range
    let in_range_count = z3::ast::Int::new_const(&ctx, "sum");
    let sum = in_range.iter().fold(zero.clone(), |acc, value| {
        acc.add(&[value])
    });

    // adds a constraint such that we compute the sum
    optimizer.assert(&in_range_count._eq(&sum));

    // when multiple pts match, we must choose the closest to 0,0,0, so let's minimize that
    let dist_to_origin = z3::ast::Int::new_const(&ctx, "dist_to_origin");
    optimizer.assert(&dist_to_origin._eq(&dist(&x, &zero, &zero).add(&[&dist(&y, &zero, &zero), &dist(&z, &zero, &zero)])));

    // maximize the number of bots in range
    optimizer.maximize(&in_range_count);
    // minimize the distance to the origin
    optimizer.minimize(&dist_to_origin);

    match optimizer.check(&[]) {
        z3::SatResult::Sat => {
            let model = optimizer.get_model();
            Solution {
                bots_in_range: model.eval(&in_range_count).unwrap().as_i64().unwrap() as u32,
                optimal: Pt::new(model.eval(&x).unwrap().as_i64().unwrap() as i32, model.eval(&y).unwrap().as_i64().unwrap() as i32, model.eval(&z).unwrap().as_i64().unwrap() as i32)
            }
        },
        _ => panic!("Solver did not sat!")
    }
}

fn parse(input: &str) -> Vec<Nanobot> {
    input.lines().map(|line| Nanobot::from_str(line).unwrap() ).collect()
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle23 { bots: parse(&input) })
}

struct Puzzle23 {
    bots: Vec<Nanobot>
}

impl crate::Puzzle for Puzzle23 {
    fn part1(&self) -> String {
        let strongest = self.bots.iter().max_by_key(|bot| bot.signal_radius).expect("no bots");
        self.bots.iter().filter(|bot| strongest.in_range(bot)).count().to_string()
    }

    fn part2(&self) -> String {
        let sol = solve(&self.bots);
        println!("{:?}", sol);
        sol.optimal.distance(&Pt::new(0,0,0)).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Puzzle;

    const EXAMPLE1: &str ="pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";

    const EXAMPLE2: &str = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";

    #[test]
    fn test_parse() {
        let bots = parse(EXAMPLE1);
        assert_eq!(9, bots.len());
        assert_eq!(Nanobot { pos: Pt::new(0,0,0), signal_radius: 4 }, bots[0]);
        assert_eq!(Nanobot { pos: Pt::new(1,0,0), signal_radius: 1 }, bots[1]);
        assert_eq!(Nanobot { pos: Pt::new(1,3,1), signal_radius: 1 }, bots[bots.len()-1]);
    }

    #[test]
    fn test_part1() {
        let pzl = Puzzle23 { bots: parse(EXAMPLE1) };
        assert_eq!("7", pzl.part1());
    }

    #[test]
    fn test_part2() {
        let pzl = Puzzle23 { bots: parse(EXAMPLE2) };
        let sol = solve(&pzl.bots);
        assert_eq!(Pt::new(12,12,12), sol.optimal);
    }

}