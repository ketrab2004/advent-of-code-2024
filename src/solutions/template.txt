use std::io::BufRead;
use crate::{Input, Output, output};

pub fn solve(input: Input) -> Output {

    for line in input.lines() {
        let line = line?;


    }

    output!(-1)
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    test_solver(solve, indoc::indoc! {"

    "}, output!(-1));
}
