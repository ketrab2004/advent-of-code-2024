use std::{cmp::min, io::BufRead};
use color_eyre::eyre::Result;
use itertools::Itertools;
use regex::Regex;
use regex_macro::regex;
use crate::{misc::option::OptionExt, output, Input, Output};


fn parse_coords(regex: &Regex, s: &str) -> Result<(isize, isize)> {
    let captures = regex.captures(s).unwrap_or_err()?;
    let x = captures[1].parse::<isize>()?;
    let y = captures[2].parse::<isize>()?;
    Ok((x, y))
}


// Friday the 13th 😨
pub fn solve(input: Input) -> Output {
    let button_regex = regex!(r"Button [AB]: X([+-]\d+), Y([+-]\d+)");
    let prize_regex = regex!(r"Prize: X=(\d+), Y=(\d+)");

    let mut total = 0;
    let mut huge_total = 0;
    for mut chunk in &input.lines().chunks(4) {
        let button_a = parse_coords(&button_regex, chunk.next().unwrap_or_err()??.as_str())?;
        let button_b = parse_coords(&button_regex, chunk.next().unwrap_or_err()??.as_str())?;
        let prize = parse_coords(&prize_regex, chunk.next().unwrap_or_err()??.as_str())?;

        let max_b_steps = min(min(prize.0 / button_b.0, prize.1 / button_b.1), 100);
        for b_steps in (0..=max_b_steps).rev() {
            let (x, y) = (prize.0 - button_b.0 * b_steps, prize.1 - button_b.1 * b_steps);

            let a_steps = min(min(x / button_a.0, y / button_a.1), 100);
            let remaining = (
                x - button_a.0 * a_steps,
                y - button_a.1 * a_steps
            );

            if remaining.0 == 0 && remaining.1 == 0 {
                total += a_steps * 3 + b_steps * 1;
                break;
            }
        }


        // Cramer's rule
        let (c1, c2) = (prize.0 + 10000000000000, prize.1 + 10000000000000);
        let (a1, a2) = button_a;
        let (b1, b2) = button_b;

        if a1 * b2 - b1 * a2 == 0 {
            continue;
        }

        let a_steps = (c1 * b2 - b1 * c2) / (a1 * b2 - b1 * a2);
        let b_steps = (a1 * c2 - c1 * a2) / (a1 * b2 - b1 * a2);
        let diff = (
            b_steps * b1 + a_steps * a1 - c1,
            b_steps * b2 + a_steps * a2 - c2
        );

        if diff.0 == 0 && diff.1 == 0 {
            huge_total += a_steps * 3 + b_steps * 1;
        }
    }

    output!(total, huge_total)
}
