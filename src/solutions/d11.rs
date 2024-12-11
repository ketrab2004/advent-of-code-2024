use std::io::BufRead;
use indicatif::ProgressBar;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{misc::option::OptionExt, output, Input, Output};


pub fn solve(input: Input) -> Output {
    let line = input
        .lines()
        .next()
        .unwrap_or_err()??;
    let mut nums = line
        .split_ascii_whitespace()
        .map(|item| item.parse());

    let mut stones = nums.clone().collect::<Result<Vec<i64>,_>>()?;

    let steps = 25;
    let progress = ProgressBar::new(steps);
    for step in 1..=steps {
        let mut i = 0;
        while i < stones.len() {
            let stone = stones[i];
            let num_length = stone.checked_ilog10().unwrap_or(0) + 1;

            if stone == 0 {
                stones[i] = 1;

            } else if num_length % 2 == 0 {
                let upper = stone / 10_i64.pow(num_length / 2);
                let lower = stone - upper * 10_i64.pow(num_length / 2);
                // dbg!(stone, i, &num_length, &upper, &lower);

                stones[i] = lower;
                stones.insert(i, upper);
                i += 1;

            } else {
                stones[i] *= 2024;
            }
            i += 1;
        }

        progress.inc(1);
        // println!("{step}. {:?}", &stones);
    }



    output!(stones.len())
}
