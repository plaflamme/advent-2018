use structopt::StructOpt;

mod puzzle1;
mod puzzle2;
mod puzzle3;
mod puzzle4;
mod puzzle5;
mod puzzle6;
mod puzzle7;
mod puzzle8;
mod puzzle9;
mod puzzle10;
mod puzzle11;
mod puzzle12;
mod puzzle13;
mod puzzle14;
mod puzzle15;
mod puzzle16;
mod puzzle17;
mod puzzle18;
mod puzzle19;
mod puzzle20;

pub trait Puzzle {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

#[derive(StructOpt)]
struct Cli {
    puzzle: Option<usize>,
    part: Option<u32>
}

fn main() {
    let puzzles: Vec<fn(String) -> Box<dyn Puzzle>> = vec!(
        puzzle1::mk,
        puzzle2::mk,
        puzzle3::mk,
        puzzle4::mk,
        puzzle5::mk,
        puzzle6::mk,
        puzzle7::mk,
        puzzle8::mk,
        puzzle9::mk,
        puzzle10::mk,
        puzzle11::mk,
        puzzle12::mk,
        puzzle13::mk,
        puzzle14::mk,
        puzzle15::mk,
        puzzle16::mk,
        puzzle17::mk,
        puzzle18::mk,
        puzzle19::mk,
        puzzle20::mk,
    );
    let args = Cli::from_args();

    let pzls = match args.puzzle {
        None => 1..=puzzles.len(),
        Some(pzl) => {
            assert!(pzl > 0, "Puzzles start at index 1.");
            assert!(pzl <= puzzles.len(), "Puzzle {} does not yet have a solution", pzl);
            pzl..=pzl
        }
    };
    let parts = match args.part {
        None => 1..=2,
        Some(part) => part..=part
    };

    for pzl in pzls {
        let ref mk_puzzle = puzzles[pzl-1];
        let input = std::fs::read_to_string(format!("src/puzzle{}/input.txt", pzl)).expect("cannot read puzzle input.");
        let puzzle = mk_puzzle(input);
        for part in parts.clone() {
            let result = match part {
                1 => puzzle.part1(),
                2 => puzzle.part2(),
                _ => panic!("puzzles part is either 1 or 2")
            };
            println!("Puzzle {} part {}: {}", pzl, part, result);
        }
    }
}
