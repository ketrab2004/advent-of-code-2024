use std::{collections::HashMap, io::BufRead};
use crate::{misc::option::OptionExt, output, Input, Output};


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

    output!(sum, incorrect_sum)
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    test_solver(solve, indoc::indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "}, output!(143, 123));
}
