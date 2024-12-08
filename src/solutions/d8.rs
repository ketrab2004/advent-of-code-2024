use std::io::BufRead;
use itertools::Itertools;
use crate::{misc::grid::Grid, output, Input, Output};


pub fn solve(input: Input) -> Output {
    let original_map = Grid::from(input
        .lines()
        .map(|line| line.unwrap())
    ).unwrap();

    let antennae = original_map
        .iter_signed()
        .filter(|(_, _, value)| value.is_ascii_alphanumeric());

    let mut anti_nodes = original_map.clone();
    let mut real_anti_nodes = original_map.clone();

    for (_, _, antenna_type) in antennae.clone().unique_by(|(_, _, value)| *value) {
        let filtered_antennae = antennae.clone()
            .filter(|(_, _, value)| value == &antenna_type)
            .collect::<Vec<_>>();

        for (i, (x, y, _)) in filtered_antennae.iter().enumerate() {
            for (x2, y2, _) in filtered_antennae.iter().skip(i + 1) {
                let (dx, dy) = (x2 - x, y2 - y);

                anti_nodes.signed_set(x - dx, y - dy, b'#');
                anti_nodes.signed_set(x2 + dx, y2 + dy, b'#');


                let mut i = 0;
                while real_anti_nodes.signed_set(x - dx*i, y - dy*i, b'#') {
                    i += 1;
                }
                i = 0;
                while real_anti_nodes.signed_set(x2 + dx*i, y2 + dy*i, b'#') {
                    i += 1;
                }
            }
        }
    }


    let anti_node_count = anti_nodes
        .iter()
        .filter(|(_, _, value)| value == &b'#')
        .count();
    let real_anti_node_count = real_anti_nodes
        .iter()
        .filter(|(_, _, value)| value == &b'#')
        .count();

    output!(anti_node_count, real_anti_node_count)
}
