pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle15 {} )
}

struct Puzzle15 {}

impl crate::Puzzle for Puzzle15 {
    fn part1(&self) -> String {
        unimplemented!()
    }

    fn part2(&self) -> String {
        unimplemented!()
    }
}