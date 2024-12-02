use crate::{Input, Output};

mod d1;
mod d2;


pub fn solve_day(day: u32, input: Input) -> Option<Output> {
    match day {
        1 => Some(d1::solve(input)),
        2 => Some(d2::solve(input)),
        _ => None
    }
}
