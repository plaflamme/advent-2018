use structopt::StructOpt;

mod puzzle1;

#[derive(StructOpt)]
struct Cli {
    puzzle: u32,
    part: u32
}

fn main() {
    let args = Cli::from_args();
    let result = match args {
        Cli { puzzle: 1, part} => puzzle1::solve(part),
        _ => panic!("puzzle not implemented")
    };

    println!("Puzzle {} part {}: {}", args.puzzle, args.part, result);
}
