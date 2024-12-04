use std::{io::BufRead, usize};

use crate::{Input, Output};


#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    data: String
}
impl Grid {
    fn from(mut input: impl Iterator<Item = impl AsRef<str>>) -> Result<Self, ()> {
        let first = input.next().unwrap();
        let line: &str = first.as_ref();
        let mut grid = Grid {
            width: line.len(),
            height: 1,
            data: String::from(line)
        };

        for line in input {
            let line = line.as_ref();
            if line.len() != grid.width {
                return Err(());
            }
            grid.height += 1;
            grid.data.push_str(line);
        }

        Ok(grid)
    }

    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> u8 {
        self.data.bytes().nth(y * self.width + x).unwrap_unchecked()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(unsafe { self.get_unchecked(x, y) })
    }

    fn index_to_xy(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;
        (x, y)
    }

    pub fn iter(&self) -> GridIterator {
        GridIterator {
            grid: self,
            index: 0
        }
    }
}

struct GridIterator<'a> {
    grid: &'a Grid,
    index: usize
}

impl Iterator for GridIterator<'_> {
    type Item = (usize, usize, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.width * self.grid.height {
            return None;
        }

        let (x, y) = self.grid.index_to_xy(self.index);
        self.index += 1;

        Some((x, y, self.grid.get(x, y).unwrap()))
    }
}

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
