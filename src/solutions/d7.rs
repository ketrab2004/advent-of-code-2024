use std::io::BufRead;
use crate::{misc::option::OptionExt, output, Input, Output};


pub enum Operator {
    Add,
    Mul,
    Concat
}

fn get_remaining_results(current_total: i64, operands: &[i64], results: &mut Vec<i64>, allowed_operators: &[Operator]) {
    if operands.is_empty() {
        results.push(current_total);
        return;
    }
    let next = operands.first().unwrap();
    let remaining = &operands[1..];

    for operator in allowed_operators {
        match operator {
            Operator::Add => get_remaining_results(
                current_total + next,
                remaining, results, allowed_operators),
            Operator::Mul => get_remaining_results(
                current_total * next,
                remaining, results, allowed_operators),
            Operator::Concat => get_remaining_results(
                current_total * 10i32.pow(next.ilog10() + 1) as i64 + next,
                remaining, results, allowed_operators)
        }
    }
}

fn get_possible_results(operands: &[i64], allowed_operators: &[Operator]) -> Vec<i64> {
    let mut results = Vec::new();

    let start = operands.first().unwrap();
    let remaining = &operands[1..];
    get_remaining_results(*start, remaining, &mut results, allowed_operators);

    results
}

pub fn solve(input: Input) -> Output {
    let mut sum = 0;
    let mut sum_with_concat = 0;

    for line in input.lines() {
        let line = line?;

        let (result, remaining) = line.split_once(": ").unwrap_or_err()?;
        let result = result.parse::<i64>()?;
        let operands = remaining
            .split(' ')
            .map(|s| s.parse())
            .collect::<Result<Vec<i64>, _>>()?;

        if get_possible_results(operands.as_slice(), &[Operator::Add, Operator::Mul])
            .contains(&result) {
            sum += result;
        }

        if get_possible_results(operands.as_slice(), &[Operator::Add, Operator::Mul, Operator::Concat])
            .contains(&result) {
            sum_with_concat += result;
        }
    }

    output!(sum, sum_with_concat)
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    test_solver(solve, indoc::indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "}, output!(3749, 11387));
}
