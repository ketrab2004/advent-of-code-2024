use std::{collections::{VecDeque}, io::BufRead};
use color_eyre::eyre::Result;
use good_lp::{Expression, Solution, SolverModel, default_solver, variable, variables};
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

        for button in machine.button_indicators.iter() {
            let mut op_indicators = indicators.clone();
            for indicator in button {
                op_indicators[*indicator] = !op_indicators[*indicator];
            }
            queue.push_back((depth + 1, op_indicators));
        }
    }

    0
}

fn machine_solve_power(machine: &Machine) -> Result<usize> {
    let mut var = variables! {};
    let mut vars = Vec::new();
    for _ in machine.button_indicators.iter() {
        vars.push(var.add(variable().min(0).integer()));
    }

    let mut problem = var.minimise(vars.iter().fold(Expression::from(0), |acc, &v| acc + v))
        .using(default_solver);
    problem.set_parameter("log", "0");

    for (i, joltage) in machine.joltage_requirements.iter().enumerate() {
        let mut sum = Expression::from(0);
        for (j, indicator) in machine.button_indicators.iter().enumerate() {
            if indicator.contains(&i) {
                sum += vars[j];
            }
        }

        problem.add_constraint(sum.eq(*joltage as i32));
    }

    let solution = problem.solve()?;

    Ok(vars.iter().map(|var| solution.value(*var) as usize).sum())
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

    for machine in machines {
        min_button_press_indicator_sum += machine_solve_indicators(&machine);
        min_button_press_power_sum += machine_solve_power(&machine)?;
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
