use std::{collections::HashMap, io::BufRead};
use crate::{Input, Output, misc::{grid::Grid, option::OptionExt}, output};

pub fn solve(input: Input) -> Output {
    let mut grid = Grid::from(input
        .lines()
        .map(|line| line.unwrap())
    )?;

    let starting_pos = grid.find(b'S').unwrap_or_err()?;
    let mut beams = HashMap::from([(starting_pos.0, 1i64)]);
    let mut y = starting_pos.1 + 1;

    let mut split_count = 0;

    while y < grid.get_size().1 {
        let previous_beams = beams.clone();
        beams.clear();
        for (x, timelines) in previous_beams.iter() {
            if grid.get(*x, y) == Some(b'^') {
                *beams.entry(*x - 1).or_insert(0) += timelines;
                *beams.entry(*x + 1).or_insert(0) += timelines;

                split_count += 1;
            } else {
                *beams.entry(*x).or_insert(0) += timelines;
            }
        }

        for (x, _timelines) in beams.iter() {
            grid.set(*x, y, b'|');
        }
        y += 1;
    }

    let timeline_count: i64 = beams.iter().map(|(_x, timestep)| *timestep).sum();

    output!(split_count, timeline_count)
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    test_solver(solve, indoc::indoc! {"
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............
    "}, output!(21, 40));
}
