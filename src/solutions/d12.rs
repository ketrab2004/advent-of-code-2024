use std::{collections::{HashSet, VecDeque}, io::BufRead};
use crate::{misc::grid::Grid, output, Input, Output};


pub const DIRECTIONS: [(isize, isize); 4] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1)
];

#[derive(Debug, Clone)]
struct Field {
    pub crop: u8,
    pub blocks: Vec<(isize, isize)>,
    pub perimeter: usize
}

fn has_edge(grid: &Grid, x: isize, y: isize, crop: u8, dir: usize) -> bool {
    let (dx, dy) = DIRECTIONS[dir];
    let (nx, ny) = (x + dx, y + dy);

    grid.signed_get_or_default(nx, ny) != crop
}

pub fn solve(input: Input) -> Output {
    let grid = Grid::from(input
        .lines()
        .map(|line| line.unwrap())
    )?;


    let mut used = HashSet::new();
    let mut fields = Vec::<Field>::new();
    let mut cost = 0;

    for (x, y, crop) in grid.iter_signed() {
        if used.contains(&(x, y)) {
            continue;
        }
        used.insert((x, y));

        let mut field = Field {
            crop,
            blocks: vec![(x, y)],
            perimeter: 0
        };

        let mut queue = VecDeque::new();
        queue.push_back((x, y));

        while let Some((x, y)) = queue.pop_front() {
            for (dir, (dx, dy)) in DIRECTIONS.iter().enumerate() {
                let (nx, ny) = (x + dx, y + dy);

                if has_edge(&grid, x, y, crop, dir) {
                    field.perimeter += 1;
                    continue;
                }

                if !used.contains(&(nx, ny))  {
                    queue.push_back((nx, ny));
                    used.insert((nx, ny));

                    field.blocks.push((nx, ny));
                }
            }
        }

        cost += field.perimeter * field.blocks.len();
        fields.push(field);
    }

    let mut bulk_cost = 0;
    for field in fields.iter() {
        let mut edge_count = 0;
        let mut used_edges = HashSet::new();

        for (x, y) in field.blocks.iter() {
            for (dir, (dx, dy)) in DIRECTIONS.iter().enumerate() {
                if used_edges.contains(&(*x, *y, dir)) {
                    continue;
                }
                let (nx, ny) = (x + dx, y + dy);
                if !has_edge(&grid, *x, *y, field.crop, dir) {
                    continue;
                }
                used_edges.insert((nx, ny, dir));

                let mut i = 0;
                let search_dir = (dir + 1) % DIRECTIONS.len();
                let (dx, dy) = DIRECTIONS[search_dir];
                loop {
                    let (nx, ny) = (x + i * dx, y + i * dy);
                    if !field.blocks.contains(&(nx, ny)) || !has_edge(&grid, nx, ny, field.crop, dir) {
                        break;
                    }
                    used_edges.insert((nx, ny, dir));
                    i += 1;
                }

                i = 0;
                let search_dir = (dir + DIRECTIONS.len() - 1) % DIRECTIONS.len();
                let (dx, dy) = DIRECTIONS[search_dir];
                loop {
                    let (nx, ny) = (x + i * dx, y + i * dy);
                    if !field.blocks.contains(&(nx, ny)) || !has_edge(&grid, nx, ny, field.crop, dir) {
                        break;
                    }
                    used_edges.insert((nx, ny, dir));
                    i += 1;
                }

                edge_count += 1;
            }
        }
        bulk_cost += edge_count * field.blocks.len();
    }


    output!(cost, bulk_cost)
}
