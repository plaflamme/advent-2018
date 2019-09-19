use structopt::StructOpt;

mod puzzle1;
mod puzzle2;
mod puzzle3;

pub trait Puzzle {
    fn part1(&self, input: String) -> String;
    fn part2(&self, input: String) -> String;
}

#[derive(StructOpt)]
struct Cli {
    puzzle: usize,
    part: u32
}

fn main() {
    let puzzles: Vec<Box<dyn Puzzle>> = vec!(Box::new(puzzle1::Puzzle1), Box::new(puzzle2::Puzzle2), Box::new(puzzle3::Puzzle3));
    let args = Cli::from_args();

    assert!(args.puzzle > 0, "Puzzles start at index 1.");
    assert!(args.puzzle <= puzzles.len(), "Puzzle {} does not yet have a solution", args.puzzle);
    let ref puzzle = puzzles[args.puzzle-1];
    let input = std::fs::read_to_string(format!("src/puzzle{}/input.txt", args.puzzle)).expect("cannot read puzzle input.");
    let result = match args.part {
        1 => puzzle.part1(input),
        2 => puzzle.part2(input),
        _ => panic!("puzzles part is either 1 or 2")
    };
    
    println!("Puzzle {} part {}: {}", args.puzzle, args.part, result);
}
