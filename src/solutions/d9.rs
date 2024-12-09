use std::io::BufRead;
use itertools::Itertools;

use crate::{misc::option::OptionExt, output, Input, Output};

fn print_blocks(blocks: &Vec<Option<usize>>) {
    print!("[");
    for block in blocks {
        print!("{}", match block {
            Some(id) => id.to_string(),
            None => ".".into()
        })
    }
    println!("]");
}

fn defragment_1(blocks: &mut Vec<Option<usize>>) -> usize {
    loop {
        let Some(hole_index) = blocks.iter().position(|block| block.is_none()) else {
            break;
        };
        let Some(moved) = blocks.pop() else {
            continue;
        };
        blocks[hole_index] = moved;
    }

    let mut checksum = 0usize;
    for (i, id) in blocks.iter().enumerate() {
        if let Some(id) = id {
            checksum += id * i;
        }
    }
    checksum
}

fn defragment_2(blocks: &mut Vec<Option<usize>>) -> usize {
    let mut last_hole_index = 0;
    for _ in 0..10 {
        let Some(hole_index) = blocks
            .iter()
            .skip(last_hole_index)
            .position(|block| block.is_none()) else {
            break;
        };
        let mut hole_size = 0;
        for block in blocks.iter().skip(hole_index) {
            if let Some(_block) = block {
                break;
            };
            hole_size += 1;
        }

        let Some(mut moved) = blocks.iter().enumerate().rev().find(|n| n.is_some()) else {
            break;
        };
        let mut moved_size = 0;
        let mut moved_index = 0;

        for id in available_ids.iter() {
            let index = blocks.iter().position(|block| *block == Some(*id));
            let Some(index) = index else {
                continue;
            };
            dbg!(index, hole_index);
            if index <= hole_index {
                continue;
            }

            moved_size = 0;
            for item in blocks.iter().skip(index) {
                let Some(item) = item else {
                    break;
                };
                if item != id {
                    break;
                }
                moved_size += 1;
            }

            if moved_size <= hole_size {
                moved = *id;
                moved_index = index;
                break;
            }
        }

        println!("moving #{moved} {moved_size}x to #{hole_index} {hole_size}x");
        print_blocks(&blocks);
        for i in 0..moved_size {
            blocks[hole_index + i] = Some(moved);
            blocks[moved_index + i] = None;
        }
        print_blocks(&blocks);
        println!();
        last_hole_index = hole_index;
    }

    let mut checksum = 0usize;
    for (i, id) in blocks.iter().enumerate() {
        if let Some(id) = id {
            checksum += id * i;
        }
    }
    dbg!(blocks, checksum);
    checksum
}

pub fn solve(input: Input) -> Output {
    let mut total = 0i64;
    let mut unfragmented_total = 0i64;

    for line in input.lines() {
        let line = line?;

        let mut blocks = Vec::new();

        let mut id = 0;
        for (i, c) in line.chars().enumerate() {
            let size = c.to_digit(10).unwrap_or_err()?;
            if i % 2 == 0 {
                for _ in 0..size {
                    blocks.push(Some(id));
                }
                id += 1;
            } else {
                for _ in 0..size {
                    blocks.push(None);
                }
            }
        }

        let unfragmented_checksum = defragment_2(&mut blocks.clone());
        let checksum = defragment_1(&mut blocks);

        total += checksum as i64;
        unfragmented_total += unfragmented_checksum as i64;
    }

    output!(total, unfragmented_total)
}
