use std::{collections::HashMap, io::BufRead, ops::Rem};
use crate::{misc::{grid::Grid, option::OptionExt, progress::pretty_progress_bar, vector2::Directions}, output, Input, Output};


/// Returns whether the path loops.
fn traverse(map: &mut Grid, mut x: isize, mut y: isize) -> bool {
    let directions = isize::DIRECTIONS;
    let mut visited = HashMap::<(isize, isize), [bool; 4]>::new();
    let mut current_direction = 3;

    loop {
        let (dx, dy) = directions[current_direction];
        let next = map.signed_get_or_default(x + dx, y + dy);
        match next {
            b'#' | b'O' => {
                current_direction = (current_direction + 1).rem(directions.len())
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

                let mut dirs = *visited.get(&(x, y)).unwrap_or(&[false; 4]);
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
    let (ux, uy) = original_map.find_signed(b'^').unwrap_or_err()?;

    let mut map = original_map.clone();
    assert!(!traverse(&mut map, ux, uy));

    let count = map
        .iter()
        .filter(|(_, _, value)| value == &b'X')
        .count();


    let progress = pretty_progress_bar(count as u64);

    let mut obstruction_count = 0;
    for (x, y, _) in map.iter_signed().filter(|(_, _, value)| value == &b'X') {
        let mut map = original_map.clone();
        map.signed_set(x, y, b'O');
        if traverse(&mut map, ux, uy) {
            obstruction_count += 1;
        }
        progress.inc(1);
    }

    output!(count, obstruction_count)
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    test_solver(solve, indoc::indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "}, output!(41, 6));
}
