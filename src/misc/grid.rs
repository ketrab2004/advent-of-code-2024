use core::str;
use std::{error::Error, fmt::{Debug, Display}};
use color_eyre::eyre::Result;
use super::option::OptionExt;


#[derive(Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    data: String
}

#[derive(Debug, Clone, Copy)]
pub enum GridError {
    InvalidWidth
}
impl Error for GridError {}
impl Display for GridError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("grid error")
    }
}


#[allow(dead_code)]
impl Grid {
    /// Creates from iterator of lines,
    /// using the width of the first line.
    ///
    /// Fails if not all lines have the same width.
    pub fn from(mut input: impl Iterator<Item = impl AsRef<str>>) -> Result<Self, ()> {
        let first = input.next().unwrap();
        let line: &str = first.as_ref();
        let mut grid = Grid {
            width: line.len(),
            height: 1,
            data: String::from(line)
        };
        grid.data.push('\n');

        for line in input {
            let line = line.as_ref();
            if line.len() != grid.width {
                return Err(());
            }
            grid.height += 1;

            grid.data.reserve(grid.width + 1);
            grid.data.push_str(line);
            grid.data.push('\n');
        }

        Ok(grid)
    }

    pub fn from_string(input: String) -> Result<Self> {
        let mut lines = input.lines();
        let width  = lines.next().unwrap_or_err()?.len();
        let mut height = 1;
        for line in lines {
            if line.len() != width {
                return Err(GridError::InvalidWidth.into());
            }
            height += 1;
        }

        Ok(Grid {
            width,
            height,
            data: input
        })
    }

    /// Creates from iterator of lines,
    /// using the given width.
    /// Padding with fill to reach it.
    ///
    /// Fails if any line is longer than the given width.
    pub fn with_width(input: impl Iterator<Item = impl AsRef<str>>, width: usize, fill: Option<char>) -> Result<Self, ()> {
        let mut grid = Grid {
            width,
            height: 0,
            data: String::with_capacity(width)
        };

        let fill = fill.unwrap_or(' ');
        for line in input {
            let line = line.as_ref();
            if line.len() > width {
                return Err(());
            }

            grid.data.reserve(width + 1);
            grid.data.push_str(line);
            for _ in 0..(width - line.len()) {
                grid.data.push(fill);
            }
            grid.data.push('\n');
        }

        Ok(grid)
    }

    /// Creates from iterator of lines.
    ///
    /// Gets the width of the widest line and pads each line with the given fill to reach it.
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

    pub fn from_size(width: usize, height: usize, fill: u8) -> Self {
        let mut line = String::from(char::from(fill));
        line = line.repeat(width);
        line.push('\n');

        let mut data = String::with_capacity((width + 1) * height);
        for _ in 0..height {
            data.push_str(&line);
        }

        Self {
            width,
            height,
            data
        }
    }


    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> u8 {
        self.data.as_bytes()[y * (self.width + 1)+ x]
    }

    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(unsafe { self.get_unchecked(x, y) })
    }

    pub fn signed_get(&self, x: isize, y: isize) -> Option<u8> {
        if x < 0 || x as usize >= self.width
            || y < 0 || y as usize >= self.height  {
                return None;
        }
        Some(unsafe { self.get_unchecked(x as usize, y as usize) })
    }

    /// Gets the value or '\0' if out of bounds.
    pub fn get_or_default(&self, x: usize, y: usize) -> u8 {
        self.get(x, y).unwrap_or(0)
    }

    /// Gets the value or '\0' if out of bounds.
    pub fn signed_get_or_default(&self, x: isize, y: isize) -> u8 {
        self.signed_get(x, y).unwrap_or(0)
    }


    pub unsafe fn set_unchecked(&mut self, x: usize, y: usize, value: u8) {
        self.data.as_bytes_mut()[y * (self.width + 1) + x] = value;
    }

    /// Returns whether the value was set.
    pub fn set(&mut self, x: usize, y: usize, value: u8) -> bool {
        if x >= self.width
            || y >= self.height  {
                return false;
        }
        unsafe { self.set_unchecked(x, y, value); }
        true
    }

    /// Returns whether the value was set.
    pub fn signed_set(&mut self, x: isize, y: isize, value: u8) -> bool {
        if x < 0 || x as usize >= self.width
            || y < 0 || y as usize >= self.height  {
                return false;
        }
        unsafe { self.set_unchecked(x as usize, y as usize, value); }
        true
    }


    fn index_to_xy(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;
        (x, y)
    }

    fn index_to_xy_signed(&self, index: usize) -> (isize, isize) {
        let (x, y) = self.index_to_xy(index);
        (x as isize, y as isize)
    }

    /// Gets a linear iterator over the grid, with coordinates included.
    pub fn iter(&self) -> GridIterator {
        GridIterator {
            grid: self,
            index: 0
        }
    }

    /// Gets a linear iterator over the grid, with (signed) coordinates included.
    pub fn iter_signed(&self) -> GridIteratorSigned {
        GridIteratorSigned {
            grid: self,
            index: 0
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.data.as_str())
    }
}

struct DebugGrid<'a>(&'a str);
impl<'a> Debug for DebugGrid<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\"\"\"\n")?;
        f.write_str(&self.0)?;
        f.write_str("\"\"\"")
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("Grid")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("data", &DebugGrid(&self.data))
            .finish()
    }
}


#[derive(Clone)]
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

#[derive(Clone)]
pub struct GridIteratorSigned<'a> {
    grid: &'a Grid,
    index: usize
}

impl Iterator for GridIteratorSigned<'_> {
    type Item = (isize, isize, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.width * self.grid.height {
            return None;
        }

        let (x, y) = self.grid.index_to_xy_signed(self.index);
        self.index += 1;

        Some((x, y, self.grid.signed_get(x, y).unwrap()))
    }
}
