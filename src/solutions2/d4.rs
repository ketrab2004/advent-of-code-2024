use std::io::BufRead;
use crate::{Input, Output, output, misc::{grid::Grid, vector2::Directions}};

fn get_forklift_positions(grid: &Grid) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    for (x, y, c) in grid.iter() {
        if c == b'@' {
            let count = isize::DIAGONAL_DIRECTIONS.iter()
                .filter(|(dx, dy)| grid.signed_get(x as isize + *dx, y as isize + *dy) == Some(b'@'))
                .count();

            if count < 4 {
                positions.push((x, y));
            }
        }
    }
    positions
}

pub fn solve(input: Input) -> Output {
    let mut grid = Grid::from(input
        .lines()
        .map(|line| line.unwrap())
    )?;

    let mut pos = get_forklift_positions(&grid);
    let forklifts = pos.len();
    let mut aa = 0;

    while pos.len() > 0 {
        aa += pos.len();

        for (x, y) in pos {
            grid.set(x, y, b'o');
        }

        pos = get_forklift_positions(&grid);
    }

    output!(forklifts, aa)
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    test_solver(solve, indoc::indoc! {"
    ..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.
    "}, output!(13, 43));
}
