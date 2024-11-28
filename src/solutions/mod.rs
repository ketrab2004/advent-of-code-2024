use crate::{Input, Output};

mod d1;


pub fn solve_day(day: u32, input: Input) -> Option<Output> {
    match day {
        1 => Some(d1::solve(input)),
        _ => None
    }
}
