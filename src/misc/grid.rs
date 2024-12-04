use core::str;
use std::fmt::{Debug, Display, Write};

#[derive(Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    data: String
}

#[allow(dead_code)]
impl Grid {
    pub fn from(mut input: impl Iterator<Item = impl AsRef<str>>) -> Result<Self, ()> {
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

    pub fn with_width(input: impl Iterator<Item = impl AsRef<str>>, width: usize, fill: Option<char>) -> Result<Self, ()> {
        let mut grid = Grid {
            width,
            height: 0,
            data: String::with_capacity(width)
        };

        for line in input {
            let line = line.as_ref();
            if line.len() > width {
                return Err(());
            }

            grid.data.reserve(width);
            grid.data.push_str(line);
            for _ in 0..(width - line.len()) {
                grid.data.push(fill.unwrap_or(' '));
            }
        }

        Ok(grid)
    }

    pub fn with_dynamic_width(input: impl Iterator<Item = impl AsRef<str>> + Clone, fill: Option<char>) -> Self {
        let mut width = 0;
        for line in input.clone() {
            let len = line.as_ref().len();
            if len > width {
                width = len;
            }
        }

        Self::with_width(input, width, fill)
            .expect("Width of input changed after first iteration")
    }


    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> u8 {
        self.data.as_bytes()[y * self.width + x]
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

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in 0..self.height {
            let i = line * self.width;

            f.write_str(unsafe {
                str::from_utf8_unchecked(&self.data.as_bytes()[i..i + self.width])
            })?;
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("Grid")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("data", &self.to_string())
            .finish()
    }
}


pub struct GridIterator<'a> {
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
