use std::{collections::{HashMap, HashSet}, io::BufRead};
use crate::{misc::option::OptionExt, output, Input, Output};


pub fn solve(input: Input) -> Output {
    let mut connections = HashMap::<String, Vec<String>>::new();

    for line in input.lines() {
        let line = line?;

        let (from, to) = line.split_once("-").unwrap_or_err()?;
        connections.entry(from.into())
            .or_default()
            .push(to.into());
        connections.entry(to.into())
            .or_default()
            .push(from.into());
    }

    let mut collections = HashSet::new();
    let mut largest_collection_size = 0;
    let mut largest_collections = HashSet::new();
    for (from, tos) in &connections {
        for i in 0..tos.len()-1 {
            for j in i+1..tos.len() {
                let mut collection = [
                    from,
                    &tos[i],
                    &tos[j]
                ];
                if !connections
                    .get(&tos[i])
                    .unwrap_or_err()?
                    .contains(&tos[j]) {
                    continue;
                }
                collection.sort();
                collections.insert(collection);
            }

            let mut collection = vec![from];
            for (j, j_value) in tos.iter().enumerate() {
                if i == j {
                    continue;
                }
                let connections = connections
                    .get(j_value)
                    .unwrap_or_err()?;
                if collection.iter().all(|node| connections.contains(node)) {
                    collection.push(j_value);
                }
            }
            collection.sort();
            if collection.len() > largest_collection_size {
                largest_collection_size = collection.len();
                largest_collections.clear();
            }
            if collection.len() == largest_collection_size {
                largest_collections.insert(collection);
            }
        }
    }

    let mut chieftain_connections = 0;
    for collection in collections {
        if collection.iter().any(|node| node.starts_with('t')) {
            chieftain_connections += 1;
        }
    }
    let largest_collection = largest_collections
        .iter()
        .next()
        .unwrap_or_err()?
        .iter()
        .map(|string| string.as_str())
        .collect::<Vec<_>>();

    output!(chieftain_connections, largest_collection.join(","))
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    test_solver(solve, indoc::indoc! {"
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
    "}, output!(7, "co,de,ka,ta"));
}
