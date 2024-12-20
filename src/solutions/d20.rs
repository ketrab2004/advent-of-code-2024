use std::{cmp::Reverse, collections::{HashMap, VecDeque}, io::BufRead, isize, usize};
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
    pub cheat_steps: u8,
    pub cheat: Option<Cheat>
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Cheat {
    Started((isize, isize)),
    Finished((isize, isize), (isize, isize))
}

fn count_cheats(
    original_map: &Grid,
    start: (isize, isize),
    allowed_cheat_steps: u8,
    route_to_end_cache: &HashMap<(isize, isize), usize>,
    uncheated_length: isize
) -> Vec<isize> {
    let mut map = original_map.clone();
    let mut cheats = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back(PathState {
        pos: (start.0, start.1),
        score: 0,
        origin_dir: None,
        cheat_steps: allowed_cheat_steps,
        cheat: None
    });

    while let Some(current) = queue.pop_front() {
        if current.score >= uncheated_length {
            continue;
        }
        if let Some(Cheat::Finished(start, end)) = current.cheat {
            if let Some(remaining) = route_to_end_cache.get(&current.pos) {
                let score = current.score + *remaining as isize;
                if score < uncheated_length {
                    // println!("found cheat {start:?}->{end:?} with score {}+{}={score} < {uncheated_length}", current.score, remaining);
                    cheats.insert((start, end), score);
                }
                continue;
            }
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

                match step.cheat {
                    None => {
                        if cell == b'#' {
                            step.cheat = Some(Cheat::Started(current.pos));
                            step.cheat_steps -= 1;
                        }
                    },
                    Some(Cheat::Started(start)) => {
                        if cell != b'#' {
                            step.cheat = Some(Cheat::Finished(start, (nx, ny)));

                        } else if step.cheat_steps <= 1 {
                            continue;
                        }
                        step.cheat_steps -= 1;
                    },
                    Some(Cheat::Finished(_, _)) => {
                        if cell == b'#' {
                            continue;
                        }
                    }
                }

                step
            };
            if step.score > uncheated_length {
                continue;
            }

            queue.push_back(step);
            map.signed_set(nx, ny, match step.cheat {
                Some(Cheat::Started(_)) => b'X',
                _ => b'*'
            });
        }
    }
    dbg!(&map);

    cheats.iter().map(|(_, score)| *score).collect()
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


    let mut distance_field = HashMap::new();
    distance_field.insert((end.0, end.1), 0);
    let mut queue = VecDeque::new();
    queue.push_back((end.0, end.1, 0));
    while let Some((x, y, dist)) = queue.pop_front() {
        for (dx, dy) in DIRECTIONS {
            let (nx, ny) = (x + dx, y + dy);
            if distance_field.contains_key(&(nx, ny))
                || original_map.signed_get_or_default(nx, ny) == b'#' {
                continue;
            }
            distance_field.insert((nx, ny), dist + 1);
            queue.push_back((nx, ny, dist + 1));
        }
    }
    let uncheated_length = distance_field.get(&(start.0, start.1)).unwrap().clone();
    dbg!(uncheated_length);

    let mut lengths = count_cheats(
        &original_map,
        (start.0, start.1),
        2,
        &distance_field,
        uncheated_length as isize
    );
    lengths.sort();
    let good_cheats = lengths
        .iter()
        .filter(|len| **len + 100 <= uncheated_length as isize)
        .count();
    println!("{:?}", &lengths.chunk_by(|a, b| a==b).map(|a| (uncheated_length as isize - a[0], a.len())).collect::<Vec<_>>());
    dbg!(&lengths.len());

    let mut lengths = count_cheats(
        &original_map,
        (start.0, start.1),
        20,
        &distance_field,
        uncheated_length as isize
    );
    lengths.sort();
    let good_cheats2 = lengths
        .iter()
        .filter(|len| **len + 100 <= uncheated_length as isize)
        .count();
    println!("{:?}", &lengths.chunk_by(|a, b| a==b).map(|a| (uncheated_length as isize - a[0], a.len())).collect::<Vec<_>>());
    dbg!(&lengths.len());


    output!(good_cheats, good_cheats2)
}
