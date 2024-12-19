use std::{collections::HashMap, io::BufRead};
use trie_rs::Trie;
use crate::{misc::option::OptionExt, output, Input, Output};


fn get_combinations(patterns: &Trie<u8>, remaining: &str, mut cache: &mut HashMap<String, Option<usize>>) -> Option<usize> {
    if remaining.len() <= 0 {
        return Some(1);
    }
    if let Some(cached) = cache.get(remaining) {
        return *cached;
    }

    let mut unique_combinations = 0;
    let mut inc_search = patterns.0.inc_search();
    for (i, c) in remaining.bytes().enumerate() {
        let Some(partial_result) = inc_search.query(&c) else {
            break;
        };

        if partial_result.is_match() {
            if let Some(combinations) = get_combinations(patterns, &remaining[i + 1..], &mut cache) {
                unique_combinations += combinations;
            }
        }
    }
    let unique_combinations = match unique_combinations {
        0 => None,
        _ => Some(unique_combinations)
    };

    cache.insert(String::from(remaining), unique_combinations);
    unique_combinations
}

pub fn solve(input: Input) -> Output {
    let mut lines = input.lines();
    let patterns = Trie::from_iter(lines
        .next()
        .unwrap_or_err()??
        .split(", "));
    lines.next();

    let mut cache = HashMap::new();
    let mut possible_designs = 0;
    let mut unique_combinations = 0;
    for line in lines {
        let line = line?;

        if let Some(combinations) = get_combinations(&patterns, &line, &mut cache) {
            possible_designs += 1;
            unique_combinations += combinations;
        }
    }

    output!(possible_designs, unique_combinations)
}
