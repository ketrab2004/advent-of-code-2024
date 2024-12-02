use std::io::BufRead;

use crate::{Input, Output};

fn is_safe(diff: i32, i: usize, prev_diff: i32) -> bool {
    !(diff.abs() < 1
        || diff.abs() > 3
        || (i > 1 && diff.signum() != prev_diff.signum()))
}

pub fn solve(input: Input) -> Output {
    let mut unsafe_count = 0;
    let mut skipped_unsafe_count = 0;
    let mut total = 0;

    for line in input.lines() {
        let line = line.unwrap();
        total += 1;

        let mut list = line.split_ascii_whitespace().enumerate();
        let mut prev = list.next().unwrap().1.parse::<i32>().unwrap();
        let mut prev_diff = 1i32;
        for (i, num) in list {
            let num = num.parse::<i32>().unwrap();
            let diff = num - prev;

            if !is_safe(diff, i, prev_diff) {
                println!("line {}:{} unsafe", total, i);
                unsafe_count += 1;
                break;
            }
            prev = num;
            prev_diff = diff;
        }

        let mut skips = 0;
        let mut i = 0usize;
        let mut peekable_list = line.split_ascii_whitespace().peekable();
        let mut prev = peekable_list.next().unwrap().parse::<i32>().unwrap();
        while let Some(num) = peekable_list.next() {
            i += 1;
            let num = num.parse::<i32>().unwrap();
            let diff = num - prev;

            let is_unsafe = !is_safe(diff, i, prev_diff);
            println!("{num}. diff{diff} unsafe:{is_unsafe}");

            if skips <= 0 && is_unsafe {
                // prev (first) can be skipped
                if i <= 1 {
                    println!("skip first i:{i} num:{num} prev:{prev}");
                    skips += 1;
                    if let Some(next) = peekable_list.peek() {
                        let next = next.parse::<i32>().unwrap();
                        let next_diff = next - num;
                        if is_safe(next_diff, i + 1, prev_diff) {
                            prev = num;
                            prev_diff = diff;
                        }

                    } else {
                        prev = num;
                        prev_diff = diff;
                    }
                    i -= 1;
                    continue;
                }

                // current can be skipped
                if let Some(next) = peekable_list.peek() {
                    let next = next.parse::<i32>().unwrap();
                    let skipped_diff = next - prev;

                    println!("line {}:{} skipped; skipped_diff:{skipped_diff} diff:{diff}; prev:{prev}", total, i);
                    if is_safe(skipped_diff, i, prev_diff) {
                        if prev != num {
                            peekable_list.next();
                        }
                        skips += 1;
                        prev_diff = skipped_diff;
                        continue;
                    }
                // current (last) can be skipped
                } else {
                    if is_safe(prev_diff, i, prev_diff) {
                        skips += 1;
                        continue;
                    }
                }
            }

            if is_unsafe {
                skipped_unsafe_count += 1;
                println!("line {}:{} unsafe; skips:{skips}; i:{i} prev_diff: {prev_diff} diff:{diff}; prev:{prev}", total, i);
                break;
            }
            prev = num;
            prev_diff = diff;
        }
    }

    (total - unsafe_count, total - skipped_unsafe_count)
}
