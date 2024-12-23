use std::{cmp::Reverse, collections::{HashMap, HashSet, VecDeque}, io::BufRead, usize};
use priority_queue::PriorityQueue;
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


#[derive(Debug, Clone, Copy)]
struct PathStepOrigins {
    pub dirs: [usize; 4]
}
impl Default for PathStepOrigins {
    fn default() -> Self {
        Self {
            dirs: [usize::MAX; 4]
        }
    }
}


pub fn solve(input: Input) -> Output {
    let mut map = Grid::from(input
        .lines()
        .map(|line| line.unwrap())
    )?;

    let start = map.find_signed(b'S').unwrap_or_err()?;
    let end = map.find_signed(b'E').unwrap_or_err()?;
    map.signed_set(end.0, end.1, b'.');

    let mut origins = HashMap::new();
    let mut queue = PriorityQueue::new();
    queue.push(PathStep {
        pos: (start.0, start.1),
        dir: 0,
        score: 0
    }, Reverse(0));
    origins.insert((start.0, start.1), PathStepOrigins {
        dirs: [0, usize::MAX, usize::MAX, usize::MAX]
    });

    while let Some((current, _)) = queue.pop() {
        if current.pos == (end.0, end.1) {
            break;
        }
        let mut origin_step = *origins.get(&current.pos).unwrap_or_err()?;

        for (dir, (dx, dy)) in DIRECTIONS.iter().enumerate() {
            let (nx, ny) = (current.pos.0 + dx, current.pos.1 + dy);
            let cell = map.signed_get_or_default(nx, ny);
            if cell == b'#' || cell == b'\0' {
                continue;
            }

            let mut rot = (dir as i32 - current.dir).rem_euclid(DIRECTIONS.len() as i32) as usize;
            rot = rot.min(DIRECTIONS.len() - rot);
            if rot != 0 {
                if origin_step.dirs[dir] > current.score + 1000 * rot {
                    origin_step.dirs[dir] = current.score + 1000 * rot;
                }
            }

            let step = PathStep {
                pos: (nx, ny),
                dir: dir as i32,
                score: current.score + 1 + 1000 * rot
            };
            let origin = origins.entry((nx, ny)).or_default();
            if step.score < origin.dirs[dir] {
                origin.dirs[dir] = step.score;
                queue.push(step, Reverse(step.score));
                map.signed_set(nx, ny, dir_to_char(dir));
            }
        }

        origins.insert(current.pos, origin_step);
    }
    let end_origins = *origins
        .get(&(end.0, end.1))
        .unwrap_or_err()?;
    let score = *end_origins
        .dirs
        .iter()
        .min()
        .unwrap_or_err()?;
    dbg!(&map);


    let mut path = HashSet::new();
    let mut queue = VecDeque::new();
    for (dir, dir_score) in end_origins.dirs.iter().enumerate() {
        if score == *dir_score {
            queue.push_back((end.0, end.1, score, dir));
        }
    }

    while let Some((x, y, score, current_dir)) = queue.pop_front() {
        if path.contains(&(x, y)) {
            continue;
        }
        path.insert((x, y));

        for (dir, (dx, dy)) in DIRECTIONS.iter().enumerate() {
            let (nx, ny) = (x - dx, y - dy);
            let Some(next) = origins.get(&(nx, ny)) else {
                continue;
            };

            let mut rot = (dir as i32 - current_dir as i32).rem_euclid(DIRECTIONS.len() as i32) as usize;
            rot = rot.min(DIRECTIONS.len() - rot);

            if next.dirs[dir] == score.wrapping_sub(1 + 1000 * rot) {
                queue.push_back((nx, ny, next.dirs[dir], dir));
            }
        }
    }
    for (x, y) in &path {
        map.signed_set(*x, *y, b'O');
    }
    dbg!(&map);


    output!(score, path.len())
}
