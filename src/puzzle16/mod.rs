use std::str::FromStr;
use itertools::Itertools;
use regex::Regex;

#[derive(PartialEq, Eq, Debug)]
struct Bench([u8; 4]);

impl FromStr for Bench {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\[(\d), (\d), (\d), (\d)]$").unwrap();
        let caps = re.captures(s).expect("invalid bench input");
        Ok(
            Bench(
                [
                    u8::from_str(&caps[1])?,
                    u8::from_str(&caps[2])?,
                    u8::from_str(&caps[3])?,
                    u8::from_str(&caps[4])?
                ]
            )
        )
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Instr {
    code: u8,
    a: u8,
    b: u8,
    c: u8
}

impl FromStr for Instr {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();
        Ok(
            Instr {
                code: u8::from_str(parts[0])?,
                a: u8::from_str(parts[1])?,
                b: u8::from_str(parts[2])?,
                c: u8::from_str(parts[3])?,
            }
        )
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Valid {
    before: Bench,
    instruction: Instr,
    after: Bench
}

impl Valid {
    fn from(lines: &Vec<&str>) -> Self {
        assert_eq!(3, lines.len());
        Valid {
            before: Bench::from_str(lines[0]).expect(""),
            instruction: Instr::from_str(lines[1]).expect(""),
            after: Bench::from_str(lines[2]).expect(""),
        }
    }
}

fn parse(input: &str) -> (Vec<Valid>, Vec<Instr>) {
    let mut prev_empty = false;
    let mut split_idx= 0;
    for (idx,line) in input.lines().enumerate() {
        if line.is_empty() && prev_empty {
            split_idx = idx;
            break;
        }
        prev_empty = line.is_empty();
    }

    assert_ne!(0, split_idx);


    let mut part1_input = input.lines().collect::<Vec<_>>();
    let part2_input = part1_input.split_off(split_idx);

    let part1 = part1_input
        .iter()
        .cloned()
        .filter(|line| !line.is_empty())
        .batching(|lines| {
            let i = lines.take(3).collect::<Vec<_>>();
            if i.is_empty() { None } else {
                Some(Valid::from(&i))
            }
        })
        .collect::<Vec<_>>();

    let part2 = part2_input
        .iter()
        .cloned()
        .filter(|line| !line.is_empty())
        .map(|line| Instr::from_str(line).expect(&format!("invalid line {}", line)))
        .collect::<Vec<_>>();

    (part1, part2)
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    let (part1, part2) = parse(&input);
    Box::new(Puzzle16 { part1, part2 })
}

struct Puzzle16 {
    part1: Vec<Valid>,
    part2: Vec<Instr>
}

impl crate::Puzzle for Puzzle16 {
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
    use crate::Puzzle;
    use lazy_static::lazy_static;

    const PART1_EXAMPLE: &str = r#"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]"#;

    const PARSE_EXAMPLE: &str = r#"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]

Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]


1 2 3 4
1 2 3 4"#;

    #[test]
    fn test_parse_instr() {
        assert_eq!(Instr { code: 1, a: 2, b: 3, c: 4}, Instr::from_str("1 2 3 4").unwrap());
    }

    #[test]
    fn test_parse_valid() {
        let expected = Valid { before: Bench([3,2,1,1]), instruction: Instr { code: 9, a: 2, b: 1, c: 2}, after: Bench([3,2,2,1])};
        assert_eq!(expected, Valid::from(&PART1_EXAMPLE.lines().collect()));
    }

    #[test]
    fn test_parse() {
        let (part1, part2) = parse(PARSE_EXAMPLE);
        assert_eq!(2, part1.len());
        assert_eq!(2, part2.len());
    }
}