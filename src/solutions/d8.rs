use std::{io::BufRead, iter::Filter};
use itertools::Itertools;
use crate::{misc::grid::{Grid, GridIterator}, output, Input, Output};


fn get_antennae(map: &Grid) -> impl Iterator<Item = (usize, usize, u8)> + '_ {
    map
        .iter()
        .filter(|(_, _, value)| value.is_ascii_alphanumeric())
}

pub fn solve(input: Input) -> Output {
    let original_map = Grid::from(input
        .lines()
        .map(|line| line.unwrap())
    ).unwrap();

    let mut anti_nodes = original_map.clone();
    let mut real_anti_nodes = original_map.clone();

    for (_, _, antenna_type) in get_antennae(&original_map).unique_by(|(_, _, value)| *value) {
        let filtered_antennae = get_antennae(&original_map)
            .filter(|(_, _, value)| value == &antenna_type)
            .collect::<Vec<_>>();

        for (i, (x, y, _)) in filtered_antennae.iter().enumerate() {
            let (x, y) = (*x as isize, *y as isize);
            for (x2, y2, _) in filtered_antennae.iter().skip(i + 1) {
                let (x2, y2) = (*x2 as isize, *y2 as isize);
                let (dx, dy) = (x2 - x, y2 - y);

                anti_nodes.signed_set(x - dx, y - dy, b'#');
                anti_nodes.signed_set(x2 + dx, y2 + dy, b'#');


                let mut i = 0;
                while real_anti_nodes.signed_get(x - dx*i, y - dy*i).is_some() {
                    real_anti_nodes.signed_set(x - dx*i, y - dy*i, b'#');
                    i += 1;
                }
                i = 0;
                while real_anti_nodes.signed_get(x2 + dx*i, y2 + dy*i).is_some() {
                    real_anti_nodes.signed_set(x2 + dx*i, y2 + dy*i, b'#');
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
