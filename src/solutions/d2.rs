use std::io::BufRead;

use crate::{Input, Output};

fn is_safe(diff: i32, i: usize, prev_diff: i32) -> bool {
    !(diff.abs() < 1
        || diff.abs() > 3
        || (i > 1 && diff.cmp(&0) != prev_diff.cmp(&0)))
}

fn line_is_safe(line_list: impl Iterator<Item = i32>) -> bool {
    let mut list = line_list.enumerate();
    let mut prev = list.next().unwrap().1;
    let mut prev_diff = 1i32;
    for (i, num) in list {
        let diff = num - prev;

        if !is_safe(diff, i, prev_diff) {
            return false;
        }
        prev = num;
        prev_diff = diff;
    }
    true
}

pub fn solve(input: Input) -> Output {
    let mut unsafe_count = 0;
    let mut skipped_unsafe_count = 0;
    let mut total = 0;

    for line in input.lines() {
        let line = line.unwrap();
        total += 1;

        let mut list = line
            .split_ascii_whitespace()
            .map(|string| string.parse::<i32>().unwrap());

        let mut list2 = list.clone();
        let list_line = list.clone().collect::<Vec<i32>>();

        if !line_is_safe(list) {
            unsafe_count += 1;
        } else {
            continue;
        }

        if !list2
            .enumerate()
            .any(|(i, _num)| {
                let mut temp = list_line.clone();
                temp.remove(i);
                line_is_safe(temp.iter().map(|n| *n))
            }
        ) {
            skipped_unsafe_count += 1;
        }
    }

    (total - unsafe_count, total - skipped_unsafe_count)
}
