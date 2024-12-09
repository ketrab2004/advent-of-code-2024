use std::io::BufRead;
use crate::{misc::option::OptionExt, output, Input, Output};


fn shrink_1(blocks: &mut Vec<Option<u32>>) -> usize {
    loop {
        let Some(hole_index) = blocks.iter().position(|block| block.is_none()) else {
            break;
        };
        let Some(moved) = blocks.pop() else {
            continue;
        };
        blocks[hole_index] = moved;
    }

    let mut checksum = 0;
    for (i, id) in blocks.iter().enumerate() {
        if let Some(id) = id {
            checksum += i * (*id as usize);
        }
    }
    checksum
}


#[derive(Debug, Clone, Copy)]
struct Block {
    id: Option<u32>,
    size: u32
}

fn shrink_2(blocks: &mut Vec<Block>) -> usize {
    let mut last_moved_offset = 0;
    loop {
        let mut i = 0;
        while i < blocks.len() {
            let block = blocks[i];
            while blocks.get(i + 1).is_some() && blocks[i + 1].id == block.id {
                blocks[i].size += blocks[i + 1].size;
                blocks.remove(i + 1);
            }
            i += 1;
        }

        let Some((to_move_index, to_move)) = blocks
            .iter()
            .enumerate()
            .rev()
            .skip(last_moved_offset)
            .find(|(_, block)| block.id.is_some()) else {
            break;
        };
        let to_move = *to_move;

        let Some(hole_index) = blocks
            .iter()
            .position(|block| block.id.is_none() && block.size >= to_move.size) else {
            last_moved_offset = blocks.len() - to_move_index;
            continue;
        };
        if hole_index > to_move_index {
            last_moved_offset = blocks.len() - to_move_index;
            continue;
        }
        let hole = blocks[hole_index];

        blocks[to_move_index].id = None;
        blocks[hole_index].size -= to_move.size;
        if hole.size == 0 {
            blocks[hole_index] = to_move;
        } else {
            blocks.insert(hole_index, to_move);
        }
    }

    let mut checksum = 0;
    let mut i = 0;
    for block in blocks.iter() {
        if let Some(id) = block.id {
            for j in 0..block.size as usize {
                checksum += (i + j) * (id as usize);
            }
        }
        i += block.size as usize;
    }
    checksum
}


pub fn solve(input: Input) -> Output {
    let mut total = 0;
    let mut unfragmented_total = 0;

    for line in input.lines() {
        let line = line?;

        let mut fragmented_blocks = Vec::new();
        let mut blocks = Vec::new();

        let mut id = 0;
        for (i, c) in line.chars().enumerate() {
            let size = c.to_digit(10).unwrap_or_err()?;
            if i % 2 == 0 {
                for _ in 0..size {
                    fragmented_blocks.push(Some(id));
                }
                blocks.push(Block {
                    id: Some(id),
                    size
                });
                id += 1;

            } else {
                for _ in 0..size {
                    fragmented_blocks.push(None);
                }
                blocks.push(Block {
                    id: None,
                    size
                });
            }
        }

        let checksum = shrink_1(&mut fragmented_blocks);
        let unfragmented_checksum = shrink_2(&mut blocks);

        total += checksum as usize;
        unfragmented_total += unfragmented_checksum as usize;
    }

    output!(total, unfragmented_total)
}
