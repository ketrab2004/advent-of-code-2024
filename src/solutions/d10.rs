use std::{collections::{HashMap, VecDeque}, io::BufRead};
use crate::{misc::{grid::Grid, option::OptionExt, vector2::Directions}, output, Input, Output};


fn char_to_num(c: u8) -> u8 {
    c - b'0'
}

pub fn solve(input: Input) -> Output {
    let mut map = Grid::from(input
        .lines()
        .map(|line| line.unwrap())
    )?;

    let mut queue = VecDeque::new();
    let mut ends = HashMap::new();

    for (x, y, value) in map
        .iter_signed()
        .filter(|(_, _, value)| *value == b'0') {
        queue.push_back((x, y, value, (x, y)));
        ends.insert((x, y), Vec::new());
    }


    let mut routes = 0;
    let mut full_routes = 0;

    while let Some((x, y, value, start)) = queue.pop_front() {
        map.signed_set(x, y, b'X');

        let num = char_to_num(value);

        for (dx, dy) in isize::DIRECTIONS {
            let (x2, y2) = (x + dx, y + dy);

            let Some(next_value) = map.signed_get(x2, y2) else {
                continue;
            };
            if !next_value.is_ascii_digit() {
                continue;
            }
            let next_num = char_to_num(next_value);

            if num < next_num && next_num - num == 1 {
                if next_num >= 9 {
                    full_routes += 1;

                    let route_ends = ends.get_mut(&start).unwrap_or_err()?;
                    if route_ends.contains(&(x2, y2)) {
                        continue;
                    }
                    route_ends.push((x2, y2));
                    routes += 1;

                } else {
                    queue.push_back((x2, y2, next_value, start));
                }
            }
        }
    }


    output!(routes, full_routes)
}
