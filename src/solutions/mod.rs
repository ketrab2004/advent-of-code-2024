use crate::{Input, Output};

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;


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
        _ => None
    }
}
