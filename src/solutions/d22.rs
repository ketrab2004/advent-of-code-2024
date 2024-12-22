use std::{collections::{HashMap, HashSet}, io::BufRead};
use indicatif::{ProgressBar, ProgressStyle};
use crate::{Input, Output, output};


fn mix(num: i64, changed_num: i64) -> i64 {
    num ^ changed_num
}
fn prune(num: i64) -> i64 {
    num % 16777216
}

fn next_num(num: i64) -> i64 {
    let mut new = num;

    new = prune(mix(new, new * 64));

    new = prune(mix(new, new / 32));

    new = prune(mix(new, new * 2048));

    new
}

pub fn solve(input: Input) -> Output {
    let steps = 2000;

    let mut starting_nums = Vec::new();

    let mut diff_history = Vec::with_capacity(steps - 1);
    let mut price_history = Vec::with_capacity(steps);
    let mut all_sequences = HashSet::new();
    let mut all_passed_sequences = Vec::new();

    let mut sum = 0;
    for line in input.lines() {
        let line = line?;
        diff_history.clear();
        price_history.clear();
        let mut passed_sequences = HashMap::with_capacity(steps);

        let mut highest_diff_index = [None; 10];

        let mut current = line.parse()?;
        starting_nums.push(current);

        let mut max = -1;
        let mut last_price = 0;
        for i in 0..steps {
            let price = current % 10;
            price_history.push(price);
            if i > 0 {
                let diff = price - last_price;
                diff_history.push(diff);
            }
            last_price = price;

            if i > 3 {
                let sequence = (diff_history[i-4], diff_history[i-3], diff_history[i-2], diff_history[i-1]);
                all_sequences.insert(sequence);
                if passed_sequences.get(&sequence).is_none() {
                    passed_sequences.insert(sequence, current);

                    if highest_diff_index[price as usize].is_none() {
                        highest_diff_index[price as usize] = Some(i);
                    }
                }
            }

            if current > max {
                max = current;
            }
            current = next_num(current);
        }
        sum += current;

        all_passed_sequences.push(passed_sequences);
    }

    let progress = ProgressBar::new(all_sequences.len() as u64);
    progress.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:64} {pos:>4}/{len:4} {eta} {msg}")?
            .progress_chars("#<-")
    );
    let mut best_banana_winnings = -1;
    for sequence in all_sequences {
        let mut banana_winnings = 0;
        for passed_sequences in &all_passed_sequences {
            if let Some(num) = passed_sequences.get(&sequence) {
                banana_winnings += num % 10;
            }
        }

        if banana_winnings > best_banana_winnings {
            best_banana_winnings = banana_winnings;
        }

        progress.inc(1);
    }

    output!(sum, best_banana_winnings)
}
