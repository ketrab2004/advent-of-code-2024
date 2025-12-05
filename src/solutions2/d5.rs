use std::{cmp::{max, min}, io::BufRead, ops::RangeInclusive};
use crate::{Input, Output, output, misc::option::OptionExt};

pub fn solve(input: Input) -> Output {
    let mut lines = input.lines();
    let mut fresh_ranges = Vec::new();

    for line in lines.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let (a, b) = line.split_once("-").unwrap();
        fresh_ranges.push(a.parse::<usize>()?..=b.parse()?);
    }

    let mut fresh_count = 0;
    for line in lines {
        let line = line?;
        let val = line.parse::<usize>()?;

        for range in &fresh_ranges {
            if range.contains(&val) {
                fresh_count += 1;
                break;
            }
        }
    }

    let mut normalized_ranges: Vec<RangeInclusive<usize>> = Vec::new();
    for range in &fresh_ranges {
        let mut matches = Vec::new();

        for (i, normalized_range) in normalized_ranges.iter().enumerate() {
            if normalized_range.contains(range.start())
                || normalized_range.contains(range.end())
                || range.contains(normalized_range.start())
                || range.contains(normalized_range.end())
            {
                matches.push(i);
            }
        }

        match matches.len() {
            0 => normalized_ranges.push(range.clone()),
            1 => {
                let i = matches.first_mut().unwrap();
                let m = &normalized_ranges[*i];
                normalized_ranges[*i] = *min(m.start(), range.start())..=*max(m.end(), range.end());
            },
            _ => {
                let min = *min(matches.iter().map(|i| &normalized_ranges[*i]).min_by_key(|a| a.start()).unwrap_or_err()?.start(), range.start());
                let max = *max(matches.iter().map(|i| &normalized_ranges[*i]).max_by_key(|a| a.end()).unwrap_or_err()?.end(), range.end());

                for i in matches.iter().rev() {
                    normalized_ranges.remove(*i);
                }
                normalized_ranges.push(min..=max);
            }
        }
    }

    let mut normalized_fresh_count = 0;
    for range in &normalized_ranges {
        normalized_fresh_count += range.end() - range.start() + 1;
    }

    output!(fresh_count, normalized_fresh_count)
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    test_solver(solve, indoc::indoc! {"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    "}, output!(3, 14));
}
