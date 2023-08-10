use std::path::Path;

use aoc_2022_lib::{error::{CLIArgsError, InternalSystemError, SolutionNotImplementedError}, Solution, solution::Question1};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about="CLI to launch Solutions for Advent of Code 2022", long_about = None)]
struct Args {
    #[arg(short='q', long)]
    question: u32,

    #[arg(short='f', long)]
    file: String,

    #[arg(short='v', long, default_value_t=false)]
    verbose: bool,
}

fn main() -> Result<(), InternalSystemError> {
    let args = Args::parse();

    if args.question < 1 || args.question > 25 {
        return Err(CLIArgsError::new("Question must be between 1 and 25 (inclusive)").into())
    }

    let path = Path::new(args.file.as_str());
    if !path.exists() {
        return Err(CLIArgsError::new("Specified file not found").into())
    }

    let solutions: Vec<Box<dyn Solution>> = vec![
        // put solutions in here
        Box::new(Question1::new())
    ];

    let solution = solutions.iter().filter(
        |i| i.id() == args.question
    ).last();

    match solution {
        Some(s) => s.solve(args.file, args.verbose),
        None => Err(SolutionNotImplementedError::new(args.question).into())
    }
}
