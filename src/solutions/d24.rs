use std::{collections::{HashMap, VecDeque}, io::BufRead};
use color_eyre::eyre::Result;
use regex::Regex;
use regex_macro::regex;
use crate::{misc::option::OptionExt, output, Input, Output};


fn numbered_node_name(prefix: char, number: i32) -> String {
    format!("{prefix}{number:0>2}")
}

fn operate(operation_regex: &Regex, operation: &str, variables: &mut HashMap<String, i8>) -> Result<i8> {
    let captures = operation_regex.captures(operation)
        .unwrap_or_err()?;
    let a = variables.get(&captures[1]).unwrap_or_err()?;
    let b = variables.get(&captures[3]).unwrap_or_err()?;

    let output = match &captures[2] {
        "AND" => a & b,
        "XOR" => a ^ b,
        "OR" => a | b,
        _ => unreachable!()
    };

    variables.insert(captures[4].to_string(), output);
    Ok(output)
}

fn check_single_dependencies<'a>(node: &String, remaining_ops: &mut Vec<&str>, dependencies: &'a HashMap<String, Vec<(String, Vec<String>)>>, swaps: &mut Vec<String>) -> Result<Vec<&'a String>> {
    let mut correct_deps = Vec::new();

    let Some(deps) = dependencies.get(node) else {
        return Ok(Vec::new());
    };
    for (op, dep) in deps {
        if let Some(index) = remaining_ops.iter().position(|x| x == op) {
            remaining_ops.remove(index);
            correct_deps.extend(dep);
        } else {
            swaps.push(node.clone());
        }
    }

    Ok(correct_deps)
}

fn check_output_dependencies(depth: i32, dependencies: &HashMap<String, Vec<(String, Vec<String>)>>, swaps: &mut Vec<String>) -> Result<()> {
    let current = numbered_node_name('z', depth);

    let mut remaining = vec!["XOR"];
    let higher = check_single_dependencies(&current, &mut remaining, dependencies, swaps)?;

    if depth <= 1 {
        return Ok(());
    }

    remaining.clear();
    remaining.extend_from_slice(&["OR", "XOR"]);
    for high in higher {
        let had_or = remaining.contains(&"OR");
        let even_higher = check_single_dependencies(high, &mut remaining, dependencies, swaps)?;

        if had_or && !remaining.contains(&"OR") {
            for higher in even_higher {
                let mut higher_remaining = vec!["AND", "XOR"];
                check_single_dependencies(higher, &mut higher_remaining, dependencies, swaps)?;
            }
        }
    }

    Ok(())
}

// fn check_deeper<'a>(node: &String, dependents: &'a HashMap<String, Vec<(String, String)>>, remaining_ops: &mut Vec<&str>) -> Result<Iterator<Item=&'a String>> {
//     let deps = dependents.get(node).unwrap_or_err()?;

// }

fn check_dependents<'a>(depth: i32, dependents: &'a HashMap<String, Vec<(String, String)>>, incorrect: &mut Vec<&'a String>) -> Result<()> {
    let nodes = [numbered_node_name('x', depth), numbered_node_name('y', depth)];

    for node in nodes {
        dbg!(&node);
        let deps = dependents.get(&node).unwrap_or_err()?;
        let mut remaining_ops = vec!["XOR", "AND"];
        for (op, dep) in deps {
            if let Some(index) = remaining_ops.iter().position(|x| x == op) {
                remaining_ops.remove(index);
                let next_op = match op.as_str() {
                    "XOR" => "XOR",
                    "AND" => "OR",
                    _ => unreachable!()
                };
                let Some(deps) = dependents.get(dep) else {
                    incorrect.push(dep);
                    continue;
                };
                let mut found = false;
                for (op, dep) in deps {
                    if op == next_op && !found {
                        found = true;
                        continue;
                    }

                    incorrect.push(dep);
                }
            } else {
                incorrect.push(dep);
            }
        }
    }

    Ok(())
}

pub fn solve(input: Input) -> Output {
    let input_regex = regex!(r"([\w\d]+): (\d)");
    let operation_regex = regex!(r"([\w\d]+) (\w+) ([\w\d]+) -> ([\w\d]+)");
    let mut lines = input.lines();

    let mut variables = HashMap::new();

    for line in lines.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let captures = input_regex.captures(line.as_str())
            .unwrap_or_err()?;
        variables.insert(captures[1].to_string(), captures[2].parse::<i8>()?);
    }

    let mut dependencies = HashMap::<String, Vec<(String, Vec<String>)>>::new();
    let mut dependents = HashMap::<String, Vec<(String, String)>>::new();

    let mut queue = VecDeque::new();
    for line in lines {
        let line = line?;
        if line.is_empty() {
            break;
        }
        queue.push_back(line.clone());

        let captures = operation_regex.captures(line.as_str())
            .unwrap_or_err()?;

        let mut dep = vec![captures[1].to_string(), captures[3].to_string()];
        dep.sort();
        dependencies.entry(captures[4].to_string())
            .or_default()
            .push((captures[2].to_string(), dep.clone()));
        for dependent in dep {
            dependents.entry(dependent)
                .or_default()
                .push((captures[2].to_string(), captures[4].to_string()));
        }
    }
    while let Some(current) = queue.pop_front() {
        if let Err(_) = operate(operation_regex, current.as_str(), &mut variables) {
            queue.push_back(current);
        };
    }

    // graphviz:
    // for (dest, (dep, op)) in &dependencies {
    //     for from in dep {
    //         println!("{from} -> {dest} [label=\"{op}\"]");
    //     }
    // }

    let mut output = 0i64;
    let mut i = 0;
    while let Some(bit) = variables.get(numbered_node_name('z', i).as_str()) {
        output |= (*bit as i64) << i;
        i += 1;
    }

    let mut swaps = Vec::new();
    // for i in 0..=i {
    //     check_output_dependencies(i, &dependencies, &mut swaps)?;
    // }
    for i in 2..i-1 {
        check_dependents(i, &dependents, &mut swaps)?;
    }
    swaps.sort();
    debug_assert_eq!(swaps.len(), 8, "{swaps:?}");

    output!(output, swaps.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(","))
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    test_solver(solve, indoc::indoc! {"
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj

    "}, output!(2024));
}
