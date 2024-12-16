use std::{cmp::Reverse, collections::HashSet, io::BufRead, usize};
use priority_queue::PriorityQueue;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use crate::{misc::{grid::Grid, option::OptionExt}, output, Input, Output};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PathStep {
    pub pos: (isize, isize),
    pub dir: i32,
    pub score: usize
}

const DIRECTIONS: [(isize, isize); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1)
];

fn dir_to_char(dir: usize) -> u8 {
    match dir {
        0 => b'>',
        1 => b'v',
        2 => b'<',
        3 => b'^',
        _ => b'?'
    }
}

#[derive(Debug, Clone)]
struct PartialRouteResult {
    pub score: usize,
    pub taken: HashSet<(isize, isize)>,
    pub unique_routes: usize
}

fn solve_partial(map: &Grid, start: PathStep, end: (isize, isize), best_score_so_far: usize) -> Option<PartialRouteResult> {
    let mut map = map.clone();
    let mut taken = HashSet::new();
    taken.insert(start.pos);
    let mut best_score_so_far = best_score_so_far;
    let mut next = Some(start);

    // println!("solve_partial {start:?}");

    let mut options = Vec::new();
    while let Some(current) = next {
        if current.pos == end {
            return Some(PartialRouteResult {
                score: current.score,
                unique_routes: 1,
                taken
            });
        }

        options.clear();
        for (dir, (dx, dy)) in DIRECTIONS.iter().enumerate() {
            let (nx, ny) = (current.pos.0 + dx, current.pos.1 + dy);
            if map.signed_get_or_default(nx, ny) != b'.' {
                continue;
            }

            let mut rot = (dir as i32 - current.dir).rem_euclid(DIRECTIONS.len() as i32) as usize;
            rot = rot.min(DIRECTIONS.len() - rot);
            let step = PathStep {
                pos: (nx, ny),
                dir: dir as i32,
                score: current.score + 1 + 1000 * rot
            };
            options.push(step);
            map.signed_set(nx, ny, dir_to_char(dir));
        }

        match options.len() {
            0 => next = None,
            1 => {
                let option = options.pop().unwrap();
                if option.score > best_score_so_far {
                    next = None;
                    continue;
                }
                taken.insert(option.pos);
                next = Some(option);
            },
            _ => {
                options.sort_by(|a, b| a.score.cmp(&b.score));
                // let mut finished_options = Vec::new();
                let mut finished_options = options.par_iter().filter_map(|option| {
                    solve_partial(&map, *option, end, best_score_so_far)
                }).collect::<Vec<_>>();
                // for option in &options {
                //     let Some(result) = solve_partial(&map, *option, end, best_score_so_far) else {
                //         continue;
                //     };
                //     if result.score < best_score_so_far {
                //         best_score_so_far = result.score;
                //     }
                //     finished_options.push(result);
                // }

                for option in &finished_options {
                    if option.score < best_score_so_far {
                        best_score_so_far = option.score;
                    }
                }

                let mut solutions = 0;
                for option in finished_options {
                    if option.score > best_score_so_far {
                        continue;
                    }
                    solutions += option.unique_routes;

                    taken.extend(option.taken);
                }

                if solutions == 0 {
                    next = None;
                    continue;
                } else {
                    return Some(PartialRouteResult {
                        score: best_score_so_far,
                        unique_routes: solutions,
                        taken
                    });
                }
            }
        }
    }

    None
}


pub fn solve(input: Input) -> Output {
    let mut original_map = Grid::from(input
        .lines()
        .map(|line| line.unwrap())
    ).unwrap();

    let start = original_map
        .iter_signed()
        .find(|(_, _, value)| *value == b'S')
        .unwrap_or_err()?;
    let end = original_map
        .iter_signed()
        .find(|(_, _, value)| *value == b'E')
        .unwrap_or_err()?;
    original_map.signed_set(end.0, end.1, b'.');


    let start_step = PathStep {
        pos: (start.0, start.1),
        dir: 0,
        score: 0
    };

    let mut map = original_map.clone();
    let mut queue = PriorityQueue::new();
    queue.push(start_step, Reverse(0));

    let mut score = 0;
    while let Some((current, _)) = queue.pop() {
        if current.pos.0 == end.0 && current.pos.1 == end.1 {
            score = current.score;
            break;
        }

        for (dir, (dx, dy)) in DIRECTIONS.iter().enumerate() {
            let (nx, ny) = (current.pos.0 + dx, current.pos.1 + dy);
            if map.signed_get_or_default(nx, ny) != b'.' {
                continue;
            }

            let mut rot = (dir as i32 - current.dir).rem_euclid(DIRECTIONS.len() as i32) as usize;
            rot = rot.min(DIRECTIONS.len() - rot);
            let step = PathStep {
                pos: (nx, ny),
                dir: dir as i32,
                score: current.score + 1 + 1000 * rot
            };
            queue.push(step, Reverse(step.score));
            map.signed_set(nx, ny, dir_to_char(dir));
        }
    }
    dbg!(&map);


    let best_paths_result = solve_partial(
        &original_map,
        start_step,
        (end.0, end.1),
        score
    )
        .unwrap_or_err()?;

    let mut filled_map = map.clone();
    for (x, y) in &best_paths_result.taken {
        filled_map.signed_set(*x, *y, b'O');
    }
    dbg!(filled_map);
    dbg!(best_paths_result.unique_routes);

    output!(score, best_paths_result.taken.len())
}
