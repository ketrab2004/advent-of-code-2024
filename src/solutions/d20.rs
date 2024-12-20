use std::{cmp::Reverse, collections::{HashMap, VecDeque}, io::BufRead, usize};
use priority_queue::PriorityQueue;

use crate::{misc::{grid::Grid, option::OptionExt}, output, Input, Output};


const DIRECTIONS: [(isize, isize); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1)
];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct PathState {
    pub pos: (isize, isize),
    pub score: isize,
    pub origin_dir: Option<usize>,
    pub cheat: u8,
    pub cheating: bool
}


pub fn solve(input: Input) -> Output {
    let mut original_map = Grid::from(input
        .lines()
        .map(|line| line.unwrap())
    )?;

    let start = original_map
        .iter_signed()
        .find(|(_, _, value)| *value == b'S')
        .unwrap_or_err()?;
    let end = original_map
        .iter_signed()
        .find(|(_, _, value)| *value == b'E')
        .unwrap_or_err()?;
    original_map.signed_set(end.0, end.1, b'.');


    let mut uncheated_length = isize::MAX;
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0, None));
    while let Some((x, y, score, origin_dir)) = queue.pop_front() {
        if x == end.0 && y == end.1 {
            uncheated_length = score;
            break;
        }
        for (dir, (dx, dy)) in DIRECTIONS.iter().enumerate() {
            if let Some(origin_dir) = origin_dir {
                if (dir + DIRECTIONS.len() / 2) % DIRECTIONS.len() == origin_dir {
                    continue;
                }
            }
            let (nx, ny) = (x + dx, y + dy);
            if original_map.signed_get_or_default(nx, ny) != b'#' {
                queue.push_back((nx, ny, score + 1, Some(dir)));
            }
        }
    }
    dbg!(uncheated_length);


    let mut map = original_map.clone();
    let mut lengths = Vec::new();

    let mut queue = PriorityQueue::new();
    queue.push(PathState {
        pos: (start.0, start.1),
        score: 0,
        origin_dir: None,
        cheat: 2,
        cheating: false
    }, Reverse(0));

    while let Some((current, _)) = queue.pop() {
        if current.pos == (end.0, end.1) {
            if current.score < uncheated_length {
                lengths.push(current.score);
            }
            continue;
        }

        if current.score >= uncheated_length {
            continue;
        }
        for (dir, (dx, dy)) in DIRECTIONS.iter().enumerate() {
            if let Some(origin_dir) = current.origin_dir {
                if (dir + DIRECTIONS.len() / 2) % DIRECTIONS.len() == origin_dir {
                    continue;
                }
            }
            let (nx, ny) = (current.pos.0 + dx, current.pos.1 + dy);
            let cell = original_map.signed_get_or_default(nx, ny);
            if cell == b'\0' {
                continue;
            }

            let step = {
                let mut step = PathState {
                    pos: (nx, ny),
                    origin_dir: Some(dir),
                    ..current
                };
                step.score += 1;

                if current.cheating {
                    if step.cheat == 0 {
                        continue;
                    }
                    step.cheat -= 1;

                    if cell != b'#' {
                        step.cheating = false;
                    } else if step.cheat == 0 {
                        continue;
                    }
                } else {
                    if cell == b'#' {
                        if step.cheat < 2 {
                            continue;
                        }
                        step.cheating = true;
                        step.cheat -= 1;
                    }
                }
                step
            };

            queue.push(step, Reverse(step.score));
            map.signed_set(nx, ny, if step.cheating { b'X' } else { b'*' });
        }
    }
    dbg!(&map, uncheated_length);
    println!("{:?}", &lengths.chunk_by(|a, b| a==b).map(|a| (uncheated_length - a[0], a.len())).collect::<Vec<_>>());
    dbg!(&lengths.len());

    output!(lengths.iter().filter(|len| **len + 100 <= uncheated_length).count())
}
