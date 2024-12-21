use std::{cmp::Reverse, collections::{HashMap, VecDeque}, io::BufRead};
use color_eyre::eyre::Result;
use priority_queue::PriorityQueue;
use crate::{misc::{grid::Grid, option::OptionExt}, output, Input, Output};


const DIRECTIONS: [(isize, isize); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1)
];

fn draw_line_y_first(dx: isize, dy: isize, output: &mut Vec<u8>) {
    for _ in 0..dy.abs() {
        if dy > 0 {
            output.push(b'v');
        } else {
            output.push(b'^');
        }
    }
    for _ in 0..dx.abs() {
        if dx > 0 {
            output.push(b'>');
        } else {
            output.push(b'<');
        }
    }
    output.push(b'A');
}

fn draw_line(keypad: &Grid, from: (isize, isize), dx: isize, dy: isize) -> Vec<u8> {
    let mut output = Vec::with_capacity((dx.abs() + dy.abs()) as usize);

    let range = if dx >= 0 { 1..=dx } else { dx..=-1 };
    for v in range {
        if dx > 0 {
            output.push(b'>');
        } else {
            output.push(b'<');
        }
        // v needs special logic as it is absolute but shouldn't be absolute
        if keypad.signed_get_or_default(from.0 + v, from.1) == b' ' {
            output.clear();
            draw_line_y_first(dx, dy, &mut output);
            return output;
        }
    }
    let range = if dy >= 0 { 1..=dy } else { dy..=-1 };
    for v in range {
        if dy > 0 {
            output.push(b'v');
        } else {
            output.push(b'^');
        }
        if keypad.signed_get_or_default(from.0 + dx - dx.signum(), from.1 + v) == b' ' {
            output.clear();
            draw_line_y_first(dx, dy, &mut output);
            return output;
        }
    }
    output.push(b'A');

    output
}

fn shortest_path(keypad: &Grid, desired_output: &[u8], upper_keypad: Option<&Grid>) -> Result<Vec<u8>> {
    let mut output = Vec::new();

    let mut current = keypad
        .iter_signed()
        .find(|(_, _, value)| *value == b'A')
        .unwrap_or_err()?;

    // let (width, height) = keypad.get_size();
    // let mut directions = HashMap::with_capacity(width * height);
    // let mut queue = PriorityQueue::with_capacity(width * height);
    for n in desired_output {
        // let mut map = keypad.clone();
        let dest = keypad
            .iter_signed()
            .find(|(_, _, value)| value == n)
            .unwrap_or_err()?;
        // println!("moving from {current:?} to {dest:?}");

        let dx = dest.0 - current.0;
        let dy = dest.1 - current.1;

        // dbg!((dx, dy), draw_line(keypad, (current.0, current.1), dx, dy));
        output.extend(draw_line(keypad, (current.0, current.1), dx, dy));

        // queue.clear();
        // directions.clear();
        // queue.push((dest.0, dest.1, None), Reverse(0));
        // while let Some((next, prio)) = queue.pop() {
        //     if (next.0, next.1) == (current.0, current.1) {
        //         break;
        //     }
        //     dbg!(prio);
        //     for (dir, (dx, dy)) in DIRECTIONS.iter().enumerate() {
        //         let (nx, ny) = (next.0 - dx, next.1 - dy);
        //         let new_value = keypad.signed_get_or_default(nx, ny);
        //         if new_value == b' ' || new_value == b'\0' || directions.contains_key(&(nx, ny)) {
        //             continue;
        //         }
        //         directions.insert((nx, ny), dir);
        //         let mut next_prio = prio.0 + 1000;

        //         if let Some(upper_keypad) = upper_keypad {
        //             let current_upper_key = match next.2 {
        //                 Some(dir) => dir_to_char(dir),
        //                 None => b'A'
        //             };
        //             let current = upper_keypad
        //                 .iter_signed()
        //                 .find(|(_, _, value)| *value == current_upper_key)
        //                 .unwrap_or_err()?;
        //             let next = upper_keypad
        //                 .iter_signed()
        //                 .find(|(_, _, value)| *value == dir_to_char(dir))
        //                 .unwrap_or_err()?;

        //             next_prio += (next.0 - current.0).abs() + (next.1 - current.1).abs();
        //             dbg!(char::from(current_upper_key), char::from(dir_to_char(dir)), next_prio);
        //         }

        //         queue.push((nx, ny, Some(dir)), Reverse(next_prio));
        //     }
        // }

        // let mut next = (current.0, current.1);
        // loop {
        //     map.signed_set(next.0, next.1, b'*');
        //     if next == (dest.0, dest.1) {
        //         break;
        //     }
        //     let dir = directions.get(&next).unwrap_or_err()?;
        //     let (dx, dy) = DIRECTIONS[*dir];
        //     next = (next.0 + dx, next.1 + dy);
        //     output.push(dir_to_char(*dir));
        // }
        // output.push(b'A');
        // dbg!(&map);
        current = dest;
    }

    Ok(output)
}

pub fn solve(input: Input) -> Output {
    let numerical_keypad = Grid::from_string("789\n456\n123\n 0A".to_string())?;
    let directional_keypad = Grid::from_string(" ^A\n<v>".to_string())?;

    let mut sum = 0;
    for line in input.lines() {
        let line = line?;

        let route = shortest_path(&numerical_keypad, line.as_bytes(), Some(&directional_keypad))?;
        dbg!(unsafe{String::from_utf8_unchecked(route.clone())});
        let route = shortest_path(&directional_keypad, route.as_slice(), Some(&directional_keypad))?;
        dbg!(unsafe{String::from_utf8_unchecked(route.clone())});
        let route = shortest_path(&directional_keypad, route.as_slice(), None)?;
        dbg!(unsafe{String::from_utf8_unchecked(route.clone())});
        dbg!(&line, line[..line.len()-1].parse::<usize>()?, route.len());

        sum += route.len() * line[..line.len()-1].parse::<usize>()?;
    }

    output!(sum)
}