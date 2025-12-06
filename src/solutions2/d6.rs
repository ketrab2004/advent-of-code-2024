use std::io::{BufRead};
use crate::{Input, Output, output};

pub fn solve(input: Input) -> Output {
    let lines: Vec<String> = input.lines().collect::<Result<Vec<_>, _>>()?;

    let mut cols: Vec<Vec<_>> = Vec::new();
    let mut max_col_width = Vec::new();
    let mut solution_sum = 0;

    for line in &lines {
        for (i, num) in line.split_whitespace().enumerate() {
            if num == "*" {
                solution_sum += cols[i].iter().fold(1, |acc, x| acc * x);
                cols[i].clear();

            } else if num == "+" {
                solution_sum += cols[i].iter().sum::<i64>();
                cols[i].clear();

            } else {
                if cols.len() <= i {
                    cols.push(Vec::new());
                }
                cols[i].push(num.parse::<i64>()?);

                if max_col_width.len() <= i {
                    max_col_width.push(0);
                }
                max_col_width[i] = max_col_width[i].max(num.len() + 1);
            }
        }
    }


    let mut cephalopod: Vec<Vec<_>> = Vec::new();
    let mut cephalopod_sum = 0;

    for line in &lines {
        let mut remaining = line.as_bytes();
        for i in 0..max_col_width.len() {
            let cur = if max_col_width[i] > remaining.len() {
                remaining
            } else {
                &remaining[..max_col_width[i]]
            };

            if cephalopod.len() <= i {
                cephalopod.push(vec![0; max_col_width[i] - 1]);
            }

            if cur.starts_with(b"*") {
                cephalopod_sum += cephalopod[i].iter().fold(1, |acc, x| acc * x);
                for n in cephalopod[i].iter_mut() {
                    *n = 0;
                }

            } else if cur.starts_with(b"+") {
                cephalopod_sum += cephalopod[i].iter().sum::<i64>();
                for n in cephalopod[i].iter_mut() {
                    *n = 0;
                }

            } else {
                for j in 0..max_col_width[i].min(cur.len()) {
                    let col = cur[j];
                    if !col.is_ascii_digit() {
                        continue;
                    }
                    cephalopod[i][j] = (cephalopod[i][j] * 10) + (col - b'0') as i64;
                }
            }

            remaining = if max_col_width[i] > remaining.len() {
                &remaining[0..0]
            } else {
                &remaining[max_col_width[i]..]
            };
        }
    }

    output!(solution_sum, cephalopod_sum)
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    test_solver(solve, indoc::indoc! {"
        123 328  51 64
         45 64  387 23
          6 98  215 314
        *   +   *   +
    "}, output!(4277556, 3263827));
}
