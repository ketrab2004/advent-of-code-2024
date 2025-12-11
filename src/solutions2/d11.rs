use std::{collections::{HashMap, HashSet, VecDeque}, io::BufRead};
use crate::{Input, Output, misc::option::OptionExt, output};


#[derive(Debug, Clone, Copy, Default)]
struct Visited {
    count: i64,
    dac: bool,
    fft: bool
}

pub fn solve(input: Input) -> Output {
    let mut connections = HashMap::new();

    for line in input.lines() {
        let line = line?;

        let (from, to) = line.split_once(":").unwrap_or_err()?;

        let mut tos = Vec::new();
        for part in to.trim_ascii().split(" ") {
            tos.push(part.to_owned());
        }

        connections.insert(from.to_owned(), tos);
    }


    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(("you", 1));

    let mut total_options = 0i64;

    while let Some((current, options)) = queue.pop_front() {
        for next in connections[current].iter() {
            if *next == "out" {
                total_options += options;
                continue;
            }

            if visited.contains(next) {
                for item in queue.iter_mut() {
                    if item.0 == *next {
                        item.1 += options;
                    }
                }
                continue;
            }
            visited.insert(next);

            queue.push_back((next, options));
        }
    }

    // uncomment for graphviz dot commands:
    // for (key, values) in connections.iter() {
    //     for value in values.iter() {
    //         println!("{key} -> {value}");
    //     }
    // }


    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(("svr", vec![Visited { count: 1, ..Default::default()}]));

    let mut valid_server_options = 0;

    while let Some((current, visiteds)) = queue.pop_front() {
        for next in connections[current].iter() {
            if *next == "out" {
                for visited in visiteds.iter() {
                    if visited.dac && visited.fft {
                        valid_server_options += visited.count;
                    }
                }
                continue;
            }

            if visited.contains(next) {
                let mut merged = false;
                for item in queue.iter_mut() {
                    if item.0 != *next {
                        continue;
                    }
                    merged = true;

                    'visited: for visited in visiteds.iter() {
                        for already_visited in item.1.iter_mut() {
                            if (already_visited.dac == visited.dac || next == "dac")
                                && (already_visited.fft == visited.fft || next == "fft")
                            {
                                already_visited.count += visited.count;
                                continue 'visited;
                            }
                        }
                        item.1.push(visited.clone());
                    }
                }
                if merged {
                    continue;
                }
            }
            visited.insert(next);

            let mut next_visiteds = visiteds.clone();
            for visited in next_visiteds.iter_mut() {
                if next == "dac" {
                    visited.dac = true;

                } else if next == "fft" {
                    visited.fft = true;
                }
            }
            queue.push_back((next, next_visiteds));
        }
    }

    // let valid_server_options = total_server_options.iter().filter(|op| {
    //     op.contains(&"dac") && op.contains(&"fft")
    // }).count();

    output!(total_options, valid_server_options)
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    // test_solver(solve, indoc::indoc! {"
    //     aaa: you hhh
    //     you: bbb ccc
    //     bbb: ddd eee
    //     ccc: ddd eee fff
    //     ddd: ggg
    //     eee: out
    //     fff: out
    //     ggg: out
    //     hhh: ccc fff iii
    //     iii: out
    // "}, output!(5));

    test_solver(solve, indoc::indoc! {"
        svr: aaa bbb
        you: bbb ccc
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out
    "}, output!(4, 2));
}
