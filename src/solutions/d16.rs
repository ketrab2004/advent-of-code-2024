use std::{cmp::Reverse, collections::{HashMap, HashSet, VecDeque}, io::BufRead};
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


pub fn solve(input: Input) -> Output {
    let mut map = Grid::from(input
        .lines()
        .map(|line| line.unwrap())
    ).unwrap();

    let start = map
        .iter_signed()
        .find(|(_, _, value)| *value == b'S')
        .unwrap_or_err()?;
    let end = map
        .iter_signed()
        .find(|(_, _, value)| *value == b'E')
        .unwrap_or_err()?;
    map.signed_set(end.0, end.1, b'.');

    let mut routes = HashMap::<(isize, isize), PathStep>::new();
    let mut queue = PriorityQueue::new();
    queue.push(PathStep {
        pos: (start.0, start.1),
        dir: 0,
        score: 0
    }, Reverse(0));

    let mut score = 0;
    while let Some((step, _)) = queue.pop() {
        if step.pos.0 == end.0 && step.pos.1 == end.1 {
            score = step.score;
            break;
        }

        for (dir, (dx, dy)) in DIRECTIONS.iter().enumerate() {
            let (nx, ny) = (step.pos.0 + dx, step.pos.1 + dy);
            if map.signed_get_or_default(nx, ny) != b'.' {
                continue;
            }

            let mut rot = (dir as i32 - step.dir).rem_euclid(DIRECTIONS.len() as i32) as usize;
            rot = rot.min(DIRECTIONS.len() - rot);
            let step = PathStep {
                pos: (nx, ny),
                dir: dir as i32,
                score: step.score + 1 + 1000 * rot
            };
            queue.push(step, Reverse(step.score));

            let old_step = routes.get(&step.pos);
            if old_step.is_none() || (*old_step.unwrap()).score < step.score {
                routes.insert((nx, ny), step);
            }
            map.signed_set(nx, ny, dir_to_char(dir));
        }
    }
    dbg!(&map);

    let mut best_route_steps = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(PathStep {
        pos: (end.0, end.1),
        dir: 0,
        score
    });

    while let Some(step) = queue.pop_front() {
        if routes.get(&step.pos).is_none() {
            continue;
        }
        best_route_steps.insert(step.pos);
        println!("{:?}", step);

        for (dx, dy) in DIRECTIONS {
            let prev = (step.pos.0 + dx, step.pos.1 + dy);
            if best_route_steps.contains(&prev) {
                continue;
            }
            let Some(prev_step) = routes.get(&prev) else {
                continue;
            };
            println!("prev: {:?}", prev_step);
            if prev_step.score < step.score {
                queue.push_back(*prev_step);
            }
        }
    }

    for (x, y) in best_route_steps.iter() {
        map.signed_set(*x, *y, b'O');
    }
    dbg!(map);


    output!(score, best_route_steps.len() + 1)
}
