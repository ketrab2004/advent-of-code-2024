use std::{collections::{HashMap, VecDeque}, io::BufRead};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{Input, Output, misc::{option::OptionExt, progress::pretty_progress_bar}, output};


#[derive(Debug, Default, Hash)]
struct Machine {
    indicators: Vec<bool>,
    button_indicators: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>
}

fn machine_solve_indicators(machine: &Machine) -> usize {
    let mut queue = VecDeque::new();

    queue.push_back((0, machine.indicators.iter().map(|_| false).collect::<Vec<_>>()));

    while let Some((depth, indicators)) = queue.pop_front() {
        if indicators == machine.indicators {
            return depth;
        }

        for (i, button) in machine.button_indicators.iter().enumerate() {
            let mut op_indicators = indicators.clone();
            for indicator in button {
                op_indicators[*indicator] = !op_indicators[*indicator];
            }
            queue.push_back((depth + 1, op_indicators));
        }
    }

    0
}


fn machine_solve_power(machine: &Machine) -> usize {
    let mut queue = VecDeque::new();

    queue.push_back((0, machine.joltage_requirements.iter().map(|_| 0).collect::<Vec<_>>()));

    while let Some((depth, indicators)) = queue.pop_front() {
        if indicators == machine.joltage_requirements {
            return depth;
        }

        if indicators.iter().enumerate().any(|(i, v)| *v > machine.joltage_requirements[i]) {
            continue;
        }

        for (i, button) in machine.button_indicators.iter().enumerate() {
            let mut op_indicators = indicators.clone();
            for indicator in button {
                op_indicators[*indicator] += 1;
            }
            queue.push_back((depth + 1, op_indicators));
        }
    }

    0
}

pub fn solve(input: Input) -> Output {
    let mut machines = Vec::new();

    for line in input.lines() {
        let full_line = line?;
        let mut machine = Machine::default();

        let (indicators, remaining) = full_line.split_once(' ').unwrap_or_err()?;

        for indicator in indicators[1..indicators.len()-1].bytes() {
            machine.indicators.push(match indicator {
                b'#' => true,
                _ => false
            });
        }

        for part in remaining.split(' ') {
            let mut content = Vec::new();
            for inner_part in part[1..part.len()-1].split(',') {
                content.push(inner_part.parse()?);
            }

            if part.starts_with("{") {
                machine.joltage_requirements = content;
            } else {
                machine.button_indicators.push(content);
            }
        }

        machines.push(machine);
    }

    let progress = pretty_progress_bar(machines.len() as u64);

    let mut min_button_press_indicator_sum = 0;
    let mut min_button_press_power_sum = 0;

    // (min_button_press_indicator_sum, min_button_press_power_sum) = machines.par_iter().map(|machine| {
    //     (machine_solve_indicators(machine), machine_solve_power(machine))
    // }).reduce(|| (0, 0), |(a, b), (c, d)| (a + c, b + d));

    for machine in machines {
        min_button_press_indicator_sum += machine_solve_indicators(&machine);
        min_button_press_power_sum += machine_solve_power(&machine);
        progress.inc(1);
    }

    output!(min_button_press_indicator_sum, min_button_press_power_sum)
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    test_solver(solve, indoc::indoc! {"
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    "}, output!(7, 33));
}
