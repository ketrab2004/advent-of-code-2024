use std::io::BufRead;

use crate::{misc::option::OptionExt, Input, Output};

pub fn solve(input: Input) -> Output {
    let mut list_a = Vec::<i32>::new();
    let mut list_b = Vec::<i32>::new();

    for line in input.lines() {
        let line = line?;
        let (a, b) = line.split_once("   ").unwrap_or_err()?;
        list_a.push(a.parse()?);
        list_b.push(b.parse()?);
    }

    list_a.sort();
    list_b.sort();

    let mut sum = 0;
    let mut similarity_score = 0;
    for i in 0..list_a.len() {
        sum += (list_a[i] - list_b[i]).abs();

        similarity_score += list_a[i] * list_b
            .iter()
            .filter(|n| n == &&list_a[i])
            .count() as i32;
    }

    Ok((sum, similarity_score))
}
