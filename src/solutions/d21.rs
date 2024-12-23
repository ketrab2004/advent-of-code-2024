use std::{collections::HashMap, io::BufRead, iter::repeat};
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


/// Calculates the cost of taking the given path on the remaining keypads.
/// Recursing deeper, but going less deep in terms of keypads.
fn path_go_deeper(remaining: &mut [(&Grid, HashMap<((isize, isize), (isize, isize)), usize>)], line: LineIterator) -> Result<usize> {
    if remaining.len() == 0 {
        return Ok(line.count() + 1);
    }
    let (keypad, ..) = remaining[0];
    let start_pos = keypad.find_signed(b'A').unwrap_or_err()?;

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

        let dest = keypad.find_signed(button).unwrap_or_err()?;

        score += shortest_path(remaining, pos, dest)?;
        pos = dest;
        prev_delta = cur_delta;
    }
    score += shortest_path(remaining, pos, (start_pos.0, start_pos.1))?;

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
                if val == b' ' || val == b'\0' {
                    return false;
                }
            }
            true
        })
        .filter_map(|line| match path_go_deeper(remaining, *line) {
            Ok(score) => Some(score),
            Err(_) => None
        })
        .min()
        .unwrap_or_err()?;

    cache.insert((start, end), best_option);
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

    let mut part2_keypads =
        repeat(&numerical_keypad).take(1)
        .chain(repeat(&directional_keypad).take(25))
        .map(|keypad| (keypad, HashMap::new()))
        .collect::<Vec<_>>();

    let mut sum = 0;
    let mut sum2 = 0;
    for line in input.lines() {
        let line = line?;
        let line_num = line[..line.len()-1].parse::<usize>()?;

        let start = part1_keypads[0].0
            .find_signed(b'A')
            .unwrap_or_err()?;

        let mut route_sum = 0;
        let mut last_pos = (start.0, start.1);
        for output in line.as_bytes() {
            let end_pos = part1_keypads[0].0
                .find_signed(*output)
                .unwrap_or_err()?;

            route_sum += shortest_path(&mut part1_keypads, last_pos, end_pos)?;
            last_pos = end_pos;
        }
        sum += route_sum * line_num;

        route_sum = 0;
        last_pos = (start.0, start.1);
        for output in line.as_bytes() {
            let end_pos = part2_keypads[0].0
                .find_signed(*output)
                .unwrap_or_err()?;

            route_sum += shortest_path(&mut part2_keypads, last_pos, end_pos)?;
            last_pos = end_pos;
        }
        sum2 += route_sum * line_num;
    }

    output!(sum, sum2)
}
