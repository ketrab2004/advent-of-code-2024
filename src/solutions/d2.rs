use std::io::BufRead;
use crate::{output, Input, Output};


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
        total += 1;

        let list = line?
            .split_ascii_whitespace()
            .map(|string| string.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;

        if !line_is_safe(list.iter().copied()) {
            unsafe_count += 1;
        } else {
            continue;
        }

        if !list
            .iter()
            .enumerate()
            .any(|(i, _num)| {
                let mut temp = list.clone();
                temp.remove(i);
                line_is_safe(temp.iter().copied())
            }
        ) {
            skipped_unsafe_count += 1;
        }
    }

    output!(total - unsafe_count, total - skipped_unsafe_count)
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    test_solver(solve, indoc::indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "}, output!(2, 4));
}
