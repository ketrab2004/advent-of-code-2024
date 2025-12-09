use std::{cmp::{min, max}, io::BufRead};
use crate::{Input, Output, misc::{option::OptionExt, progress::pretty_progress_bar}, output};

pub fn solve(input: Input) -> Output {
    let mut red_corners = Vec::new();

    for line in input.lines() {
        let line = line?;

        let (x, y) = line.split_once(',').unwrap_or_err()?;
        red_corners.push((
            x.parse::<i64>()?,
            y.parse::<i64>()?,
        ));
    }

    let mut largest_area = 0;
    let mut largest_aa = 0;

    let progress = pretty_progress_bar(((red_corners.len() * (red_corners.len() - 1)) / 2) as u64);

    for (i, (x, y)) in red_corners.iter().enumerate() {
        for (x2, y2) in red_corners[i + 1..].iter() {
            let area = ((x2 - x).abs() + 1) * ((y2 - y).abs() + 1);
            if area > largest_area {
                largest_area = area;
            }

            let x_range = *min(x, x2)..*max(x, x2);
            let y_range = *min(y, y2)..*max(y, y2);

            if area > largest_aa {
                let mut is_safe = true;
                for (k, (start_x, start_y)) in red_corners.iter().enumerate() {
                    let k2 = (k + 1) % red_corners.len();
                    let (end_x, end_y) = red_corners[k2];

                    if ((x_range.contains(start_x) && y_range.contains(start_y))
                        || (x_range.contains(&end_x) && y_range.contains(&end_y)))
                        && *start_x != x_range.start
                        && *start_y != y_range.start
                        && end_x != x_range.end
                        && end_y != y_range.end
                    {
                        is_safe = false;
                        break;

                    } else if *start_x == end_x
                        && *start_x != x_range.start
                        && x_range.contains(&start_x)
                        && min(*start_y, end_y) < y_range.end
                        && max(*start_y, end_y) > y_range.start
                    {
                        is_safe = false;
                        break;

                    } else if *start_y == end_y
                        && *start_y != y_range.start
                        && y_range.contains(&start_y)
                        && min(*start_x, end_x) < x_range.end
                        && max(*start_x, end_x) > x_range.start
                    {
                        is_safe = false;
                        break;
                    }
                }

                if is_safe {
                    largest_aa = area;
                }
            }
            progress.inc(1);
        }
    }

    output!(largest_area, largest_aa)
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    test_solver(solve, indoc::indoc! {"
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
    "}, output!(50, 24));
}
