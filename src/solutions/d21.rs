use std::{collections::HashMap, io::BufRead};
use color_eyre::eyre::{eyre, Result};
use crate::{misc::{grid::Grid, option::OptionExt}, output, Input, Output};


#[derive(Debug, Clone, Copy)]
struct LineIterator {
    current: (isize, isize),
    delta: (isize, isize),
    x_first: bool
}
impl LineIterator {
    pub fn new(dx: isize, dy: isize, x_first: bool) -> Self {
        Self {
            current: (0, 0),
            delta: (dx, dy),
            x_first
        }
    }
}
impl Iterator for LineIterator {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.delta {
            return None;
        }

        if self.x_first {
            self.current.0 += self.delta.0.signum();

            if self.current.0 == self.delta.0 {
                self.x_first = false;
            }
        } else {
            self.current.1 += self.delta.1.signum();

            if self.current.1 == self.delta.1 {
                self.x_first = true;
            }
        }

        Some(self.current)
    }
}


/// Calculates the cost of taking the given step on the remaining keypads.
/// Recursing deeper, but going less deep in terms of keypads.
fn path_go_deeper(remaining: &mut [(&Grid, HashMap<((isize, isize), (isize, isize)), usize>)], line: LineIterator) -> Result<usize> {
    if remaining.len() == 0 {
        return Ok(line.count() + 1);
    }
    let (keypad, ..) = remaining[0];
    let start_pos = keypad
        .iter_signed()
        .find(|(_, _, value)| *value == b'A')
        .unwrap_or_err()?;

    let mut prev_delta = (0, 0);
    let mut pos = (start_pos.0, start_pos.1);
    let mut score = 0;
    for cur_delta in line {
        let step_delta = (prev_delta.0 - cur_delta.0, prev_delta.1 - cur_delta.1);
        let button = match step_delta {
            (1, 0) => b'<',
            (0, 1) => b'^',
            (-1, 0) => b'>',
            (0, -1) => b'v',
            _ => return Err(eyre!("Invalid step {step_delta:?}"))
        };

        let button_pos = keypad
            .iter_signed()
            .find(|(_, _, value)| *value == button)
            .unwrap_or_err()?;
        let dest = (button_pos.0, button_pos.1);

        score += shortest_path(remaining, pos, dest)?;
        pos = dest;
        prev_delta = cur_delta;
    }
    score += shortest_path(remaining, pos, (start_pos.0, start_pos.1))?;
    // println!("x first line {} depth {} has score {score}", line.x_first, remaining.len() + 1);

    Ok(score)
}

/// Returns the number of steps to get from `start` to `end`,
/// in the topmost (deepest) given keypad.
/// Summing the steps in less deep keypads for each step.
fn shortest_path(
    keypads: &mut [(&Grid, HashMap<((isize, isize), (isize, isize)), usize>)],
    start: (isize, isize),
    end: (isize, isize)
) -> Result<usize> {
    if keypads.len() <= 0 {
        // On the keypad controlled by us, each move (to anywhere) takes 1 step.
        return Ok(1);
    }
    let ((keypad, cache), remaining) = keypads.split_first_mut().unwrap_or_err()?;
    if let Some(steps) = cache.get(&(start, end)) {
        return Ok(*steps);
    }

    let (dx, dy) = (end.0 - start.0, end.1 - start.1);
    let best_option = [LineIterator::new(dx, dy, true), LineIterator::new(dx, dy, false)]
        .iter()
        .filter(|line| {
            for (dx, dy) in **line {
                let val = keypad.signed_get_or_default(start.0 + dx, start.1 + dy);
                // println!("step ({dx}, {dy}) at ({}, {}) is {}", start.0 + dx, start.1 + dy, unsafe{char::from_u32_unchecked(val as u32)});
                if val == b' ' || val == b'\0' {
                    return false;
                }
            }
            // println!("x first line {} is valid", line.x_first);
            true
        })
        .filter_map(|line| match path_go_deeper(remaining, *line) {
            Ok(score) => Some(score),
            Err(_) => None
        })
        .min()
        .unwrap_or_err()?;

    cache.insert((start, end), best_option);
    println!("from {} to {} in depth {} is score {best_option}",
        unsafe{char::from_u32_unchecked(keypad.signed_get_or_default(start.0, start.1) as u32)},
        unsafe{char::from_u32_unchecked(keypad.signed_get_or_default(end.0, end.1) as u32)},
        remaining.len() + 1
    );
    Ok(best_option)
}

pub fn solve(input: Input) -> Output {
    let numerical_keypad = Grid::from_string("789\n456\n123\n 0A".to_string())?;
    let directional_keypad = Grid::from_string(" ^A\n<v>".to_string())?;

    let mut part1_keypads = [
        &numerical_keypad,
        &directional_keypad,
        &directional_keypad
    ].map(|keypad| (keypad, HashMap::new()));

    let mut sum = 0;
    for line in input.lines() {
        let line = line?;

        let start = part1_keypads[0].0
            .iter_signed()
            .find(|(_, _, value)| *value == b'A')
            .unwrap_or_err()?;

        let mut route_sum = 0;
        let mut last_pos = (start.0, start.1);
        for output in line.as_bytes() {
            let end = part1_keypads[0].0
                .iter_signed()
                .find(|(_, _, value)| value == output)
                .unwrap_or_err()?;
            let end_pos = (end.0, end.1);

            route_sum += shortest_path(&mut part1_keypads, last_pos, end_pos)?;
            last_pos = end_pos;
        }

        let line_num = line[..line.len()-1].parse::<usize>()?;
        sum += route_sum * line_num;
        dbg!(line, &route_sum);
    }

    output!(sum)
}
