use std::{borrow::Cow, collections::VecDeque, io::BufRead};
use trie_rs::Trie;
use crate::{misc::option::OptionExt, output, Input, Output};


pub fn solve(input: Input) -> Output {
    let mut lines = input.lines();
    let mut patterns = Trie::from_iter(lines
        .next()
        .unwrap_or_err()??
        .split(", "));
    lines.next();

    let mut possible_designs = 0;
    for line in lines {
        let line = line?;

        let mut queue = Vec::new();
        queue.push(line.as_str());
        'search: while let Some(remaining) = queue.pop() {
            if remaining.len() == 0 {
                possible_designs += 1;
                break;
            }

            let mut inc_search = patterns.0.inc_search();
            for (i, c) in remaining.bytes().enumerate() {
                let Some(partial_result) = inc_search.query(&c) else {
                    continue 'search;
                };
                if partial_result.is_match() {
                    queue.push(&remaining[i + 1..]);
                }
            }
        }
    }

    output!(possible_designs)
}
