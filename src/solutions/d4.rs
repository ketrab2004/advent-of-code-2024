use std::{io::BufRead, usize};
use crate::{Input, Output, misc::grid::Grid};


const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1)
];

pub fn solve(input: Input) -> Output {
    let grid = Grid::from(input
        .lines()
        .map(|line| line.unwrap())
    ).unwrap();

    let search = "XMAS".as_bytes();
    let mut count = 0;
    let mut mas_count = 0;

    for (x, y, value) in grid.iter() {
        if value == search[0] {
            for (dx, dy) in DIRECTIONS {
                let mut broke = false;
                let (mut cur_x, mut cur_y) = (x, y);
                for cur in search.iter().skip(1) {
                    cur_x = cur_x.wrapping_add(dx as usize);
                    cur_y = cur_y.wrapping_add(dy as usize);
                    let grid_cur = grid.get(cur_x, cur_y).unwrap_or(0);
                    if *cur != grid_cur {
                        broke = true;
                        break;
                    }
                }
                if !broke {
                    count += 1;
                }
            }
        }

        if value == b'A' {
            let top_left = grid.get(x.wrapping_add(usize::MAX), y.wrapping_add(usize::MAX)).unwrap_or(0);
            let bottom_right = grid.get(x + 1, y + 1).unwrap_or(0);
            let top_right = grid.get(x + 1, y.wrapping_add(usize::MAX)).unwrap_or(0);
            let bottom_left = grid.get(x.wrapping_add(usize::MAX), y + 1).unwrap_or(0);

            if (top_left == b'M' && bottom_right == b'S'
                || top_left == b'S' && bottom_right == b'M')
                && (top_right == b'M' && bottom_left == b'S'
                || top_right == b'S' && bottom_left == b'M'
            ) {
                mas_count += 1;
            }
        }
    }

    (count, mas_count)
}
