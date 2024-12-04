use std::io::BufRead;
use regex_macro::regex;
use crate::{Input, Output};


pub fn solve(input: Input) -> Output {
    let regex = regex!(r"(?:mul\((\d{1,3}),(\d{1,3})\)|do(?:n't)?\(\))");
    let mut sum = 0;
    let mut enabled = true;
    let mut more_precise_sum = 0;

    for line in input.lines() {

        for hit in regex.captures_iter(&line?) {
            if hit[0].starts_with("mul(") {
                let a = hit[1].parse::<i32>()?;
                let b = hit[2].parse::<i32>()?;
                let add = a * b;
                sum += add;
                if enabled {
                    more_precise_sum += add;
                }

            } else if hit[0].starts_with("don't(") {
                enabled = false;

            } else if hit[0].starts_with("do(") {
                enabled = true;
            }
        }
    }

    Ok((sum, more_precise_sum))
}
