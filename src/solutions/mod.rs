use crate::{Input, Output};

automod::dir!("src/solutions");


pub fn solve_day(day: u32, input: Input) -> Option<Output> {
    match day {
        1 => Some(d1::solve(input)),
        2 => Some(d2::solve(input)),
        3 => Some(d3::solve(input)),
        4 => Some(d4::solve(input)),
        5 => Some(d5::solve(input)),
        6 => Some(d6::solve(input)),
        7 => Some(d7::solve(input)),
        8 => Some(d8::solve(input)),
        9 => Some(d9::solve(input)),
        10 => Some(d10::solve(input)),
        11 => Some(d11::solve(input)),
        12 => Some(d12::solve(input)),
        13 => Some(d13::solve(input)),
        14 => Some(d14::solve(input)),
        15 => Some(d15::solve(input)),
        16 => Some(d16::solve(input)),
        17 => Some(d17::solve(input)),
        18 => Some(d18::solve(input)),
        19 => Some(d19::solve(input)),
        20 => Some(d20::solve(input)),
        21 => Some(d21::solve(input)),
        22 => Some(d22::solve(input)),
        23 => Some(d23::solve(input)),
        24 => Some(d24::solve(input)),
        _ => None
    }
}
