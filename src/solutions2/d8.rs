use std::{collections::HashSet, i64::MAX, io::BufRead};
use color_eyre::eyre::Result;
use itertools::Itertools;
use crate::{Input, Output, misc::{option::OptionExt, progress::pretty_progress_bar}, output};


type Coord = (i64, i64, i64);

fn dist_cubed(a: &Coord, b: &Coord) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

fn get_two_mut<T>(v: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    assert!(i != j);

    if i < j {
        let (left, right) = v.split_at_mut(j);
        (&mut left[i], &mut right[0])
    } else {
        let (left, right) = v.split_at_mut(i);
        (&mut right[0], &mut left[j])
    }
}

fn make_connections(junctions: &[Coord], connection_count: usize) -> Result<Vec<HashSet<Coord>>> {
    let mut circuits = Vec::new();
    for junction in junctions {
        circuits.push(HashSet::from([junction.clone()]));
    }

    let mut possible_connections = Vec::new();
    for (i, a) in junctions.iter().enumerate() {
        for b in junctions[i + 1..].iter() {
            possible_connections.push((dist_cubed(a, b), *a, *b))
        }
    }
    possible_connections.sort_by_key(|c| c.0);
    possible_connections.truncate(connection_count);

    for (dist, a, b) in possible_connections {
        let a_circuit_i = circuits.iter().position(|n| n.contains(&a)).unwrap_or_err()?;
        let b_circuit_i = circuits.iter().position(|n| n.contains(&b)).unwrap_or_err()?;

        if a_circuit_i == b_circuit_i {
            continue;
        }

        let (circuit_a, circuit_b) = get_two_mut(&mut circuits, a_circuit_i, b_circuit_i);

        circuit_a.extend(circuit_b.iter());
        circuits.remove(b_circuit_i);
    }

    Ok(circuits)
}

fn find_connections(junctions: &[Coord]) -> Result<(Coord, Coord)> {
    let mut circuits = Vec::new();
    for junction in junctions {
        circuits.push(HashSet::from([junction.clone()]));
    }

    let progress = pretty_progress_bar(circuits.len() as u64);

    let mut min_a = (0,0,0);
    let mut min_b = (0,0,0);
    while circuits.len() > 1 {
        let mut min_dist = MAX;
        let mut min_circuit_a = 0;
        let mut min_circuit_b = 0;

        for (i, circuit) in circuits.iter().enumerate() {
            for (j, circuit2) in circuits[i + 1..].iter().enumerate() {
                for a in circuit.iter() {
                    for b in circuit2.iter() {
                        let dist = dist_cubed(a, b);
                        if dist < min_dist {
                            min_dist = dist;
                            min_a = *a;
                            min_b = *b;
                            min_circuit_a = i;
                            min_circuit_b = i + j + 1;
                        }
                    }
                }
            }
        }

        let (circuit_a, circuit_b) = get_two_mut(&mut circuits, min_circuit_a, min_circuit_b);

        circuit_a.extend(circuit_b.iter());
        circuits.remove(min_circuit_b);
        progress.inc(1);
    }

    Ok((min_a, min_b))
}

pub fn solve(input: Input) -> Output {
    let mut positions = Vec::new();

    for line in input.lines() {
        let line = line?;

        let coordinates: Vec<i64> = line.split(',').map(|c| c.parse()).collect::<Result<Vec<_>, _>>()?;
        if coordinates.len() == 3 {
            positions.push((coordinates[0], coordinates[1], coordinates[2]));
        }
    }

    let circuits = make_connections(&positions, positions.len())?;

    // dbg!(make_connections(&positions, 10)?.iter().map(|c| c.len()).sorted().rev().collect::<Vec<_>>());

    let last_connection = find_connections(&positions)?;

    output!(
        circuits.iter().map(|c| c.len()).sorted().rev().take(3).fold(1, |acc, x| acc * x),
        last_connection.0.0 * last_connection.1.0
    )
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    // make_connections(&positions, 10)?.iter().map(|c| c.len()).sorted().rev().collect::<Vec<_>>()

    test_solver(solve, indoc::indoc! {"
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689
    "}, output!(45, 25272));
}
