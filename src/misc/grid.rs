use core::str;
use std::fmt::{Debug, Display};
use color_eyre::eyre::Result;
use error_rules::Error;
use super::option::OptionExt;


#[derive(Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    data: String
}

#[derive(Debug, Clone, Copy, Error)]
pub enum GridError {
    #[error_kind("Grid: Inserted line has different width than grid")]
    DifferentWidth,
    #[error_kind("Grid: Inserted line is wider than grid")]
    LargerWidth
}


#[allow(dead_code)]
impl Grid {
    /// Adds a single line to the grid, should not contain newlines for proper formatting.
    ///
    /// Fails if the line has different width than the grid.
    pub fn add_line(&mut self, line: impl AsRef<str>) -> Result<(), GridError> {
        let line: &str = line.as_ref();
        if line.len() != self.width {
            return Err(GridError::DifferentWidth);
        }

        self.height += 1;
        self.data.reserve(self.width + 1);
        if self.height > 1 {
            self.data.push('\n');
        }
        self.data.push_str(line);

        Ok(())
    }

    /// Creates from iterator of lines,
    /// using the width of the first line.
    ///
    /// Fails if not all lines have the same width.
    pub fn from(mut input: impl Iterator<Item = impl AsRef<str>>) -> Result<Self, GridError> {
        let first = input.next().unwrap();
        let line: &str = first.as_ref();
        let mut grid = Self {
            width: line.len(),
            height: 0,
            data: String::with_capacity(line.len() + 1)
        };

        grid.add_line(line)?;
        for line in input {
            grid.add_line(line)?;
        }

        Ok(grid)
    }

    /// Create grid using the given string as body.
    ///
    /// Each line should be the same width,
    /// ending newline is automatically removed.
    pub fn from_string(mut input: String) -> Result<Self> {
        let mut lines = input.lines();
        let width  = lines.next().unwrap_or_err()?.len();
        let mut height = 1;
        for line in lines {
            if line.len() != width {
                return Err(GridError::DifferentWidth.into());
            }
            height += 1;
        }
        if width > 0 && input.chars().last().unwrap() == '\n' {
            input.pop();
        }

        Ok(Self {
            width,
            height,
            data: input
        })
    }

    /// Adds a line to the grid, should not contain newlines for proper formatting.
    ///
    /// Fails if the line is wider than the grid, otherwise is filled to width.
    /// Spaces by default.
    pub fn add_line_with_fill(&mut self, line: impl AsRef<str>, fill: Option<char>) -> Result<(), GridError> {
        let line: &str = line.as_ref();
        if line.len() > self.width {
            return Err(GridError::LargerWidth);
        }
        let fill = fill.unwrap_or(' ');

        self.height += 1;
        self.data.reserve(self.width + 1);
        if self.height > 1 {
            self.data.push('\n');
        }
        self.data.push_str(line);
        for _ in 0..(self.width - line.len()) {
            self.data.push(fill);
        }

        Ok(())
    }

    /// Creates from iterator of lines,
    /// using the given width.
    /// Padding with fill to reach it, spaces by default.
    ///
    /// Fails if any line is longer than the given width.
    pub fn with_fill(input: impl Iterator<Item = impl AsRef<str>>, width: usize, fill: Option<char>) -> Result<Self, GridError> {
        let mut grid = Self {
            width,
            height: 0,
            data: String::with_capacity(width)
        };

        for line in input {
            grid.add_line_with_fill(line, fill)?;
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

        Self::with_fill(input, width, fill)
            .expect("Width of input changed after first iteration")
    }

    pub fn from_size(width: usize, height: usize, fill: u8) -> Self {
        let mut grid = Self {
            width,
            height: 0,
            data: String::with_capacity((width + 1) * height - 1)
        };
        let mut line = String::from(char::from(fill));
        line = line.repeat(width);

        for _ in 0..height {
            grid.add_line(&line).expect("Repeated fill char str couldn't be added to grid");
        }

        grid
    }


    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
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

struct DebugGridContents<'a>(&'a str);
impl<'a> Debug for DebugGridContents<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\"\"\"\n")?;
        f.write_str(&self.0)?;
        f.write_str("\n\"\"\"")
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f
            .debug_struct("Grid")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("data", &DebugGridContents(&self.data))
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
