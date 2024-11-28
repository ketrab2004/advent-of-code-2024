use std::{cmp::{min, Ordering}, fs::File, io::{self, BufReader, Read}, path::PathBuf};
use clap::{crate_authors, crate_description, crate_version, Parser};
use chrono::{Datelike, Utc};


#[derive(Parser, Debug)]
#[command(author=crate_authors!(", "), version=crate_version!(), about=crate_description!(), long_about = None)]
struct Args {
    #[arg(
        short, long,
        num_args = 0..=1, require_equals = true,
        default_value = None, default_missing_value = "./input",
        help = "Source of puzzle input",
        long_help = "stdin by default, --input without an explicit path defaults to ./input"
    )]
    input: Option<PathBuf>,

    #[arg(short, long, help = "Year of puzzle")]
    year: Option<i32>,

    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..=25), help = "Day of puzzle")]
    day: Option<u32>,
}


fn main() {
    let args = Args::parse();

    let now = Utc::now();
    let mut year = args.year.unwrap_or(now.year());
    let day = args.day.unwrap_or(match (year.cmp(&now.year()), now.month()) {
        (Ordering::Less, _) => 25,
        (Ordering::Greater, _) => 1,
        (Ordering::Equal, 12) => min(now.day(), 25),
        (Ordering::Equal, _) => if args.year.is_none() {
            year = now.year() - 1;
            25
        } else {
            1
        }
    });

    let input: BufReader<Box<dyn Read>> = match args.input {
        Some(path)  => BufReader::new(Box::new(File::open(path).expect("Could not open file"))),
        None => BufReader::new(Box::new(io::stdin()))
    };

    match year {
        2024 => match day {
            _ => println!("Given day has no solutions")
        },
        _ => println!("Given year has no solutions")
    };
}
