use std::str::FromStr;

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle14 { seed: u32::from_str(&input).expect(&format!("invalid input {}", input)) })
}

struct Puzzle14 {
    seed: u32
}

impl crate::Puzzle for Puzzle14 {
    fn part1(&self) -> String {
        unimplemented!()
    }

    fn part2(&self) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn part1() {

    }

    #[test]
    fn part2() {

    }
}