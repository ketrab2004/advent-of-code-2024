use std::io::BufRead;
use crate::{Input, Output, misc::grid::Grid};


const DIRECTIONS: [(isize, isize); 8] = [
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
        let x = x as isize;
        let y = y as isize;

        if value == search[0] {
            for (dx, dy) in DIRECTIONS {

                let mut broke = false;
                let (mut cur_x, mut cur_y) = (x, y);
                for cur in search.iter().skip(1) {
                    cur_x += dx;
                    cur_y += dy;
                    let grid_cur = grid.signed_get_or_default(cur_x, cur_y);

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
            let top_left = grid.signed_get_or_default(x - 1, y - 1);
            let bottom_right = grid.signed_get_or_default(x + 1, y + 1);
            let top_right = grid.signed_get_or_default(x + 1, y - 1);
            let bottom_left = grid.signed_get_or_default(x - 1, y + 1);

            if (top_left == b'M' && bottom_right == b'S'
                || top_left == b'S' && bottom_right == b'M')
                && (top_right == b'M' && bottom_left == b'S'
                || top_right == b'S' && bottom_left == b'M'
            ) {
                mas_count += 1;
            }
        }
    }

    Ok((count, mas_count))
}
