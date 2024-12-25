use std::{collections::{HashMap, HashSet, VecDeque}, io::BufRead};
use color_eyre::eyre::Result;
use itertools::Itertools;
use regex::Regex;
use regex_macro::regex;
use crate::{misc::option::OptionExt, output, Input, Output};


fn numbered_node_name(prefix: char, number: i32) -> String {
    format!("{prefix}{number:0>2}")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeType {
    Input,

    InputXor,
    Output,

    InputXorAnd, InputAnd,
    Carry
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

/// Checks the dependents of the given node, assuming it is of the given type.
/// Adds the incorrect dependent to the swaps,
/// except if there are more than 1 incorrect dependents.
///
/// Returns whether the given node is incorrect.
/// When more than one of its dependents is incorrect,
/// or if it is an output node but doesn't start with 'z',
fn check_node_dependents(node: &String, typ: NodeType, dependents: &HashMap<String, Vec<(String, String)>>, swaps: &mut HashSet<String>) -> bool {
    if typ == NodeType::Output && !node.starts_with('z') {
        return true;
    }

    let Some(deps) = dependents.get(node) else {
        return typ != NodeType::Output;
    };

    let mut allowed_deps = match typ {
        NodeType::Input => [
            ("XOR", NodeType::InputXor),
            ("AND", NodeType::InputAnd)
        ].as_slice(),
        NodeType::InputXor => [
            ("XOR", NodeType::Output),
            ("AND", NodeType::InputXorAnd)
        ].as_slice(),
        NodeType::Output => [].as_slice(),
        NodeType::InputXorAnd | NodeType::InputAnd => &[
            ("OR", NodeType::Carry)
        ].as_slice(),
        NodeType::Carry => &[
            ("AND", NodeType::InputXorAnd),
            ("XOR", NodeType::Output)
        ].as_slice()
    }.to_vec();

    // z09
    // bqw
    // jcp
    // z27
    // -ckj
    // -bch
    // -kfp


    let mut to_swap = None;
    for (op, dep) in deps {
        let mut found = None;
        for (i, (allowed_op, next_type)) in allowed_deps.iter().enumerate() {
            if *allowed_op == op {
                found = Some((i, *next_type));
                break;
            }
        }
        let Some((i, next_type)) = found else {
            println!("Under {node} ({typ:?}) {dep} has incorrect connection {op:?}");
            if to_swap.is_some() {
                return true;
            }
            to_swap = Some(dep);
            continue;
        };

        allowed_deps.remove(i);

        let incorrect = check_node_dependents(dep, next_type, dependents, swaps);
        if incorrect {
            if to_swap.is_some() {
                return true;
            }
            to_swap = Some(dep);
        }
    }

    if let Some(to_swap) = to_swap {
        swaps.insert(to_swap.clone());
    }

    false
}

fn check_node_dependencies(node: &String, typ: NodeType, dependencies: &HashMap<String, Vec<(String, Vec<String>)>>, swaps: &mut HashSet<String>) -> bool {
    if typ == NodeType::Input && !(node.starts_with('x') || node.starts_with('y')) {
        return true;
    }

    let Some(deps) = dependencies.get(node) else {
        return false;
    };

    false
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
    //         println!("{from} -> {dest} [xlabel=\"{op}\"]");
    //     }
    // }

    let mut output = 0i64;
    let mut i = 0;
    while let Some(bit) = variables.get(numbered_node_name('z', i).as_str()) {
        output |= (*bit as i64) << i;
        i += 1;
    }

    // let mut swaps = Vec::new();
    // for i in 0..=i {
    //     check_output_dependencies(i, &dependencies, &mut swaps)?;
    // }
    // for i in 2..i-1 {
    //     check_dependents(i, &dependents, &mut swaps)?;
    // }
    let mut swaps = HashSet::new();
    // debug_assert!(!check_node(&numbered_node_name('x', 2), NodeType::Input, &dependents, &mut swaps));
    for i in 2..i-1 {
        debug_assert!(!check_node_dependents(&numbered_node_name('x', i), NodeType::Input, &dependents, &mut swaps));
        debug_assert!(!check_node_dependents(&numbered_node_name('y', i), NodeType::Input, &dependents, &mut swaps));
    }
    debug_assert_eq!(swaps.len(), 8, "{swaps:?}");

    debug_assert!(!check_node_dependents(&numbered_node_name('x', 0), NodeType::Input, &dependents, &mut swaps));
    debug_assert!(!check_node_dependents(&numbered_node_name('x', 0), NodeType::Input, &dependents, &mut swaps));
    output!(output, swaps.iter().sorted().map(|s| s.as_str()).collect::<Vec<_>>().join(","))
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
