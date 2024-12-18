use std::{collections::{HashMap, VecDeque}, io::BufRead, iter};
use cgmath::num_traits::ops::bytes;
use color_eyre::eyre::Result;
use indicatif::{ProgressBar, ProgressStyle};

use crate::{misc::{grid::Grid, option::OptionExt}, output, Input, Output};


const DIRECTIONS: [(isize, isize); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1)
];

fn find_path(grid: &mut Grid, bytes_to_fall: &mut impl Iterator<Item = (isize, isize)>, start: (isize, isize), end: (isize, isize)) -> Result<Option<usize>> {
    let mut steps = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start.0 as isize, start.1 as isize, 0));

    let mut last_dist = 0;
    while let Some((x, y, dist)) = queue.pop_front() {
        if x == end.0 && y == end.1 {
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
    let mut current = end;
    loop {
        if current == (0, 0) {
            return Ok(Some(path_length));
        }
        path_length += 1;
        let Some(dir) = steps.get(&current) else {
            return Ok(None);
        };
        let (dx, dy) = DIRECTIONS[*dir];
        current = (current.0 - dx, current.1 - dy);
        grid.signed_set(current.0, current.1, b'O');
    }
}

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

    let path_length = find_path(
        &mut grid,
        &mut bytes_to_fall.map(|pos| *pos),
        (0, 0),
        (size - 1, size - 1)
    )?;
    dbg!(&grid);


    grid = Grid::from_size(size as usize, size as usize, b' ');
    let mut bytes_to_fall = steps.iter();
    let mut game_over_step = (size - 1, size - 1);
    let progress = ProgressBar::new(steps.len() as u64);
    progress.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:64} {pos:>4}/{len:4} {eta} {msg}")?
            .progress_chars("#<-")
    );
    for i in 0..steps.len() {
        let (x, y) = bytes_to_fall.next().unwrap_or_err()?;
        grid.signed_set(*x, *y, b'#');

        let mut grid = grid.clone();
        let path_length = find_path(
            &mut grid,
            &mut iter::empty(),
            (0, 0),
            (size - 1, size - 1)
        )?;
        if path_length.is_none() {
            game_over_step = (*x, *y);
            dbg!(i, &grid);
            break;
        }
        progress.inc(1);
    }

    output!(
        path_length.unwrap_or_err()?,
        format!("{},{}", game_over_step.0, game_over_step.1)
    )
}
