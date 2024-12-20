use std::{collections::{HashMap, HashSet, VecDeque}, io::BufRead, isize, usize};
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
    pub score: isize
}


fn count_cheats(
    original_map: &Grid,
    start: (isize, isize),
    allowed_cheat_steps: usize,
    route_to_end_cache: &HashMap<(isize, isize), usize>,
    uncheated_length: isize
) -> Vec<isize> {
    let allowed_cheat_steps = allowed_cheat_steps as isize;
    let mut map = original_map.clone();
    let mut visited = HashSet::new();
    let mut cheats = Vec::new();

    let mut queue = VecDeque::new();
    queue.push_back(PathState {
        pos: (start.0, start.1),
        score: 0
    });

    while let Some(current) = queue.pop_front() {
        visited.insert(current.pos);
        if current.score >= uncheated_length {
            continue;
        }

        for dx in -allowed_cheat_steps..=allowed_cheat_steps {
            for dy in -allowed_cheat_steps..=allowed_cheat_steps {
                let steps = dx.abs() + dy.abs();
                if steps == 0 || steps > allowed_cheat_steps {
                    continue;
                }
                let next = (current.pos.0 + dx, current.pos.1 + dy);
                let Some(remaining) = route_to_end_cache.get(&next) else {
                    continue;
                };
                let score = current.score + steps + *remaining as isize;
                if score < uncheated_length {
                    cheats.push(score);
                }
            }
        }

        for (dx, dy) in DIRECTIONS {
            let (nx, ny) = (current.pos.0 + dx, current.pos.1 + dy);
            if visited.contains(&(nx, ny)) {
                continue;
            }
            let cell = original_map.signed_get_or_default(nx, ny);
            if cell == b'\0' || cell == b'#' {
                continue;
            }

            let step = PathState {
                pos: (nx, ny),
                score: current.score + 1,
            };
            if step.score > uncheated_length {
                continue;
            }

            queue.push_back(step);
            map.signed_set(nx, ny, b'*');
        }
    }

    cheats
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
    let uncheated_length = *distance_field
        .get(&(start.0, start.1))
        .unwrap_or_err()?;

    let lengths = count_cheats(
        &original_map,
        (start.0, start.1),
        2,
        &distance_field,
        uncheated_length as isize
    );
    let good_cheats = lengths
        .iter()
        .filter(|len| **len + 100 <= uncheated_length as isize)
        .count();

    let lengths = count_cheats(
        &original_map,
        (start.0, start.1),
        20,
        &distance_field,
        uncheated_length as isize
    );
    let good_cheats2 = lengths
        .iter()
        .filter(|len| **len + 100 <= uncheated_length as isize)
        .count();


    output!(good_cheats, good_cheats2)
}
