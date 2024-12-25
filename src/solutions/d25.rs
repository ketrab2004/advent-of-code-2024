use std::io::BufRead;
use itertools::Itertools;

use crate::{misc::grid::Grid, output, Input, Output};

pub fn solve(input: Input) -> Output {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    let mut max_depth = 0;

    for grid in input
        .lines()
        .batching(|lines| {
            let mut lines = lines
                .map_while(|line| match line {
                    Ok(line) => if line.is_empty() {
                        None
                    } else {
                        Some(line)
                    },
                    Err(_) => None
                }).peekable();

            lines.peek()?;
            Some(Grid::from(lines))
        }
    ) {
        let grid = grid?;
        let (_, height) = grid.get_size();
        max_depth = height - 2;
        let (width, height) = grid.get_size();
        let is_key = grid.get_or_default(0, height - 1) == b'#';

        let mut depths = Vec::with_capacity(width);
        for x in 0..width {
            let mut depth = 0;
            let range: Box<dyn Iterator<Item = _>> = if is_key {
                Box::new((1..height - 1).rev())
            } else {
                Box::new(1..height - 1)
            };
            for y in range {
                if grid.get_or_default(x, y) != b'#' {
                    break;
                }
                depth += 1;
            }
            depths.push(depth);
        }

        (if is_key {
            &mut keys
        } else {
            &mut locks
        }).push(depths);
    }

    dbg!(&keys);
    dbg!(&locks);

    let mut count = 0;
    for key in keys {
        'locks: for lock in &locks {
            for (key_depth, lock_depth) in key.iter().zip(lock.iter()) {
                if key_depth + lock_depth > max_depth {
                    continue 'locks;
                }
            }
            println!("{key:?} {lock:?}");
            count += 1;
        }
    }

    output!(count)
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    test_solver(solve, indoc::indoc! {"
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####

    "}, output!(3));
}
