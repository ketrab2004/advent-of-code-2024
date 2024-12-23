use std::{collections::HashMap, io::BufRead};
use color_eyre::eyre::Result;
use crate::{misc::option::OptionExt, output, Input, Output};


fn count_stones(input: &[i64], depth: u64, previous_steps: &Vec<i64>, cache: &mut HashMap<i64, HashMap<u64, usize>>) -> Result<usize> {
    let mut sum = 0;

    for stone in input {
        {
            let stone_cache = cache.entry(*stone).or_insert(HashMap::new());
            if let Some(cached) = stone_cache.get(&depth) {
                sum += cached;
                continue;
            }
        }

        let mut next_steps = previous_steps.clone();
        next_steps.push(*stone);

        let num_length = stone.checked_ilog10().unwrap_or(0) + 1;
        let step_input = if *stone == 0 {
            vec![1]

        } else if num_length % 2 == 0 {
            let upper = stone / 10_i64.pow(num_length / 2);
            let lower = stone - upper * 10_i64.pow(num_length / 2);
            vec![upper, lower]

        } else {
            vec![stone * 2024]
        };

        let count = if depth <= 1 {
            step_input.len()
        } else {
            count_stones(step_input.as_slice(), depth - 1, &next_steps, cache)?
        };
        sum += count;
        let stone_cache = cache.get_mut(stone).unwrap_or_err()?;
        stone_cache.insert(depth, count);
    }

    Ok(sum)
}

pub fn solve(input: Input) -> Output {
    let input = input
        .lines()
        .next()
        .unwrap_or_err()??;
    let input = input
        .split_ascii_whitespace()
        .map(|item| item.parse());

    let stones = input.clone().collect::<Result<Vec<i64>,_>>()?;

    let mut cache = HashMap::new();
    let previous_steps = vec![];

    let steps = 25;
    let mut stone_count = 0;
    for stone in &stones {
        let stones = [*stone];
        stone_count += count_stones(stones.as_slice(), steps, &previous_steps, &mut cache)?;
    }


    let steps = 75;
    let mut stone_count2 = 0;
    for stone in stones {
        let stones = [stone];
        stone_count2 += count_stones(stones.as_slice(), steps, &previous_steps, &mut cache)?;
    }

    output!(stone_count, stone_count2)
}
