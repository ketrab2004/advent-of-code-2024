use std::{collections::HashMap, io::BufRead, ops::Rem};
use crate::{misc::{grid::Grid, option::OptionExt, progress::pretty_progress_bar}, output, Input, Output};


const DIRECTIONS: [(isize, isize); 4] = [
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, 0)
];


fn traverse(map: &mut Grid, ux: isize, uy: isize) -> bool {
    let (mut x, mut y) = (ux as isize, uy as isize);
    let mut visited = HashMap::<(isize, isize), [bool; 4]>::new();
    let mut current_direction = 0;

    loop {
        // dbg!(&map);
        let (dx, dy) = DIRECTIONS[current_direction];
        let next = map.signed_get_or_default(x + dx, y + dy);
        match next {
            b'#' | b'O' => {
                current_direction = (current_direction + 1).rem(DIRECTIONS.len())
            },
            b'\0' => {
                map.signed_set(x, y, b'X');
                break
            },
            _ => {
                if next == b'X' {
                    if let Some(dirs) = visited.get(&(x, y)) {
                        if dirs[current_direction] {
                            return true;
                        }
                    }
                }

                map.signed_set(x, y, b'X');

                let mut dirs = visited.get(&(x, y)).unwrap_or(&[false; 4]).clone();
                dirs[current_direction] = true;
                visited.insert((x, y), dirs);

                x += dx;
                y += dy;
            }
        }
    }

    false
}

pub fn solve(input: Input) -> Output {
    let original_map = Grid::from(input
        .lines()
        .map(|line| line.unwrap())
    )?;
    let (ux, uy, _) = original_map
        .iter()
        .find(|(_, _, value)| value == &b'^')
        .unwrap_or_err()?;

    let mut map = original_map.clone();
    assert_eq!(traverse(&mut map, ux as isize, uy as isize), false);

    let count = map
        .iter()
        .filter(|(_, _, value)| value == &b'X')
        .count();


    let progress = pretty_progress_bar(count as u64);

    let mut obstruction_count = 0;
    for (x, y, _) in map.iter().filter(|(_, _, value)| value == &b'X') {
        let mut map = original_map.clone();
        map.signed_set(x as isize, y as isize, b'O');
        if traverse(&mut map, ux as isize, uy as isize) {
            obstruction_count += 1;
        }
        progress.inc(1);
    }

    output!(count, obstruction_count)
}
