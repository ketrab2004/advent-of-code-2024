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
                    &from,
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
            for j in 0..tos.len() {
                if i == j {
                    continue;
                }
                let connections = connections
                    .get(&tos[j])
                    .unwrap_or_err()?;
                if collection.iter().all(|node| connections.contains(node)) {
                    collection.push(&tos[j]);
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
