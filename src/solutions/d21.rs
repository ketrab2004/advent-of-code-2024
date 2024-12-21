use std::{collections::{HashMap, VecDeque}, io::BufRead};
use color_eyre::eyre::Result;
use crate::{misc::{grid::Grid, option::OptionExt}, output, Input, Output};


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

fn shortest_path(keypad: &Grid, desired_output: &[u8]) -> Result<Vec<u8>> {
    let mut output = Vec::new();

    let mut current = keypad
        .iter_signed()
        .find(|(_, _, value)| *value == b'A')
        .unwrap_or_err()?;

    // let (width, height) = keypad.get_size();
    // let mut directions = HashMap::with_capacity(width * height);
    // let mut queue = VecDeque::with_capacity(width * height);
    for n in desired_output {
        let mut map = keypad.clone();
        let dest = keypad
            .iter_signed()
            .find(|(_, _, value)| value == n)
            .unwrap_or_err()?;
        println!("moving from {current:?} to {dest:?}");

        let dy = dest.1 - current.1;
        for v in 0..dy.abs() {
            if dy > 0 {
                output.push(b'v');
                map.signed_set(current.0, current.1 + v, b'*');
            } else {
                output.push(b'^');
                map.signed_set(current.0, current.1 - v, b'*');
            }
        }
        let dx = dest.0 - current.0;
        for v in 0..dx.abs() {
            if dx > 0 {
                output.push(b'>');
                map.signed_set(current.0 + v, current.1 + dy, b'*');
            } else {
                output.push(b'<');
                map.signed_set(current.0 - v, current.1 + dy, b'*');
            }
        }
        output.push(b'A');

        // queue.clear();
        // directions.clear();
        // queue.push_back(current);
        // while let Some(next) = queue.pop_front() {
        //     if next == dest {
        //         break;
        //     }
        //     for (dir, (dx, dy)) in DIRECTIONS.iter().enumerate() {
        //         let (nx, ny) = (next.0 + dx, next.1 + dy);
        //         let new_value = keypad.signed_get_or_default(nx, ny);
        //         if new_value != b' ' && new_value != b'\0' {
        //             directions.insert((nx, ny), dir);
        //             queue.push_back((nx, ny, new_value));
        //         }
        //     }
        // }
        // let mut next = (current.0, current.1);
        // loop {
        //     map.signed_set(next.0, next.1, b'*');
        //     if next == (dest.0, dest.1) {
        //         break;
        //     }
        //     for (dir, (dx, dy)) in DIRECTIONS.iter().enumerate() {
        //         let new_pos = (next.0 + dx, next.1 + dy);
        //         let Some(next_dir) = directions.get(&new_pos) else {
        //             continue;
        //         };
        //         if *next_dir == dir {
        //             next = new_pos;
        //             output.push(dir_to_char(dir));
        //         }
        //     }
        // }
        dbg!(&map);
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

        let route = shortest_path(&numerical_keypad, line.as_bytes())?;
        dbg!(unsafe{String::from_utf8_unchecked(route.clone())});
        let route = shortest_path(&directional_keypad, route.as_slice())?;
        dbg!(unsafe{String::from_utf8_unchecked(route.clone())});
        let route = shortest_path(&directional_keypad, route.as_slice())?;
        dbg!(unsafe{String::from_utf8_unchecked(route.clone())});
        dbg!(&line, line[..line.len()-1].parse::<usize>()?, route.len());

        sum += route.len() * line[..line.len()-1].parse::<usize>()?;
    }

    output!(sum)
}
