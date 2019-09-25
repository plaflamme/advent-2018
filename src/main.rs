use structopt::StructOpt;

mod puzzle1;
mod puzzle2;
mod puzzle3;
mod puzzle4;
mod puzzle5;
mod puzzle6;

pub trait Puzzle {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

#[derive(StructOpt)]
struct Cli {
    puzzle: usize,
    part: u32
}

fn main() {
    let puzzles: Vec<fn(String) -> Box<dyn Puzzle>> = vec!(
        puzzle1::mk,
        puzzle2::mk,
        puzzle3::mk,
        puzzle4::mk,
        puzzle5::mk,
        puzzle6::mk,
    );
    let args = Cli::from_args();

    assert!(args.puzzle > 0, "Puzzles start at index 1.");
    assert!(args.puzzle <= puzzles.len(), "Puzzle {} does not yet have a solution", args.puzzle);

    let ref mk_puzzle = puzzles[args.puzzle-1];
    let input = std::fs::read_to_string(format!("src/puzzle{}/input.txt", args.puzzle)).expect("cannot read puzzle input.");
    let puzzle = mk_puzzle(input);
    let result = match args.part {
        1 => puzzle.part1(),
        2 => puzzle.part2(),
        _ => panic!("puzzles part is either 1 or 2")
    };
    
    println!("Puzzle {} part {}: {}", args.puzzle, args.part, result);
}
