
struct Instruction {}

fn parse(input: &str) -> Vec<Instruction> {
    unimplemented!()
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle19 { instructions: parse(&input) })
}

struct Puzzle19 {
    instructions: Vec<Instruction>
}

impl crate::Puzzle for Puzzle19 {
    fn part1(&self) -> String {
        unimplemented!()
    }

    fn part2(&self) -> String {
        unimplemented!()
    }
}