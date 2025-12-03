use std::{collections::HashMap, io::BufRead};
use crate::{Input, Output, output, misc::progress::pretty_progress_bar};


fn local_max<'a>(line: &'a [u8], remaining_depth: u32, cache: &mut HashMap<(&'a [u8], u32), i64>) -> i64 {
    if line.is_empty() {
        return -1;
    }

    if let Some(max) = cache.get(&(line, remaining_depth)) {
        return *max;
    };

    let mut max = 0;
    for (i, c) in line[..(line.len() + 1 - remaining_depth as usize)].iter().enumerate() {
        match remaining_depth - 1 {
            0 => {
                let n = (c - b'0').into();
                if n > max {
                    max = n;
                }
            },
            _ => {
                let n = ((c - b'0') as i64 * 10i64.pow(remaining_depth - 1))
                    + local_max(&line[i+1..], remaining_depth - 1, cache);
                if n > max {
                    max = n;
                }
            }
        }
    }
    cache.insert((line, remaining_depth), max);
    max
}

pub fn solve(input: Input) -> Output {
    let mut max_sum = 0;
    let mut big_max_sum = 0;

    let lines = input.lines().collect::<Result<Vec<_>, _>>()?;

    let progress = pretty_progress_bar(lines.len() as u64);
    for line in lines {
        let line = line.as_bytes();

        max_sum += local_max(line, 2, &mut HashMap::new());

        big_max_sum += local_max(line, 12, &mut HashMap::new());

        progress.inc(1);
    }

    output!(max_sum, big_max_sum)
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    assert_eq!(local_max(b"1234567890", 2, &mut HashMap::new()), 90);
    assert_eq!(local_max(b"123", 3, &mut HashMap::new()), 123);
    assert_eq!(local_max(b"1234", 3, &mut HashMap::new()), 234);

    test_solver(solve, indoc::indoc! {"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "}, output!(357, 3121910778619i64));
}
