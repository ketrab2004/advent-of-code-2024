use std::{collections::HashMap, io::BufRead};
use crate::{misc::option::OptionExt, Input, Output};


fn has_incorrect_dependencies(manual: impl AsRef<[i32]>, dependencies: &HashMap<i32, Vec<i32>>) -> Option<usize> {
    let manual = manual.as_ref();

    for (i, num) in manual.iter().enumerate() {
        let Some(dependants) = dependencies.get(num) else {
            continue;
        };

        for dependency in dependants {
            let Some(index) = manual.iter().position(|n| n == dependency) else {
                continue;
            };
            if index < i {
                return Some(i);
            }
        }
    }

    None
}

pub fn solve(input: Input) -> Output {
    let mut dependencies = HashMap::<i32, Vec<i32>>::new();

    let mut lines = input.lines();
    for line in lines.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let (page, dependency) = line.split_once('|').unwrap_or_err()?;
        let page = page.parse()?;
        let dependency = dependency.parse()?;

        if let Some(dependencies) = dependencies.get_mut(&page) {
            dependencies.push(dependency);
        } else {
            dependencies.insert(page, vec![dependency]);
        }
    }

    let mut sum = 0;
    let mut incorrect_sum = 0;
    for line in lines {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let mut manual = line.split(',')
            .map(|num| num.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;


        if has_incorrect_dependencies(&manual, &dependencies).is_none() {
            sum += manual.get(manual.len() / 2).unwrap_or_err()?;
            continue;
        }


        while let Some(incorrect_index) = has_incorrect_dependencies(&manual, &dependencies) {
            let removed = manual.remove(incorrect_index);
            manual.insert(incorrect_index - 1, removed);
        }

        incorrect_sum += manual.get(manual.len() / 2).unwrap_or_err()?;
    }

    Ok((sum, incorrect_sum))
}
