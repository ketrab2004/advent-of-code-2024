use std::io::BufRead;
use crate::{Input, Output, output};

pub fn solve(input: Input) -> Output {
    let mut clicks = Vec::<i32>::new();

    for line in input.lines() {
        let line = line?;

        let dir = if &line[..1] == "L" {
            -1
        } else {
            1
        };
        clicks.push(line[1..].parse::<i32>()? * dir);
    }

    let mut zero_count = 0;
    let mut fine_zero_count = 0;
    let mut current = 50;
    let click_count = 100;

    for click in clicks {
        for _ in 0..(click.abs()) {
            current = (current + click.signum()).rem_euclid(click_count);

            if current == 0 {
                fine_zero_count += 1;
            }
        }

        if current == 0 {
            zero_count += 1;
        }
    }

    output!(zero_count, fine_zero_count)
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    test_solver(solve, indoc::indoc! {"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    "}, output!(3, 6));
}
