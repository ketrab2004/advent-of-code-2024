use std::{collections::{HashMap, VecDeque}, io::BufRead};
use crate::{misc::{grid::Grid, option::OptionExt}, output, Input, Output};


const DIRECTIONS: [(isize, isize); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1)
];

pub fn solve(input: Input) -> Output {
    let mut steps = Vec::new();
    for line in input.lines() {
        let line = line?;

        let (left, right) = line.split_once(',').unwrap_or_err()?;
        steps.push((
            left.parse::<isize>()?,
            right.parse::<isize>()?
        ));
    }

    let size = 70 + 1;
    let mut grid = Grid::from_size(size as usize, size as usize, b' ');
    let mut bytes_to_fall = steps.iter();
    for _ in 0..1024 {
        let (x, y) = bytes_to_fall.next().unwrap_or_err()?;
        grid.signed_set(*x, *y, b'#');
    }

    let mut steps = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((0isize, 0isize, 0));

    let mut last_dist = 0;
    while let Some((x, y, dist)) = queue.pop_front() {
        if x >= size && y >= size {
            break;
        }

        if dist > last_dist {
            if let Some(step) = bytes_to_fall.next() {
                grid.signed_set(step.0, step.1, b'#');
                last_dist = dist;
            }
        }

        for (dir, (dx, dy)) in DIRECTIONS.iter().enumerate() {
            let (nx, ny) = (x + dx, y + dy);

            if steps.contains_key(&(nx, ny)) {
                continue;
            }

            if grid.signed_get_or_default(nx, ny) == b' ' {
                grid.signed_set(nx, ny, b'.');
                queue.push_back((nx, ny, dist + 1));
                steps.insert((nx, ny), dir);
            }
        }
    }

    let mut path_length = 0;
    let mut current = (size - 1, size - 1);
    loop {
        if current == (0, 0) {
            break;
        }
        path_length += 1;
        let dir = steps.get(&current).unwrap_or_err()?;
        let (dx, dy) = DIRECTIONS[*dir];
        current = (current.0 - dx, current.1 - dy);
    }

    output!(path_length)
}
