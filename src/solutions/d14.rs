use std::{cmp::Ordering, io::BufRead};
use color_eyre::eyre::Result;
use crate::{misc::{grid::Grid, option::OptionExt, progress::pretty_progress_bar}, output, Input, Output};


fn parse_pos(line: &str) -> Result<(isize, isize)> {
    let (x, y) = line.split_once(',').unwrap_or_err()?;
    let x = x.parse()?;
    let y = y.parse()?;
    Ok((x, y))
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: (isize, isize),
    velocity: (isize, isize)
}

fn check_christmas_tree(robots: &[Robot], width: isize, height: isize) -> bool {
    let mut map = Grid::from_size(width as usize, height as usize, b'0');
    for robot in robots.iter() {
        let (x, y) = robot.pos;
        let current = map.signed_get_or_default(x, y);
        map.signed_set(x, y, (current - b'0' + 1).to_string().as_bytes()[0]);
    }

    let min_height = 3;
    for robot in robots {
        let (x, y) = robot.pos;

        let mut height = 1;
        'search: loop {
            for dx in -height..=height {
                let found = map.signed_get_or_default(x + dx, y + height);
                if found == b'0' || found == b'\0' {
                    break 'search;
                }
            }
            height += 1;
        }

        if height >= min_height {
            println!("{}", map);
            return true;
        }
    }
    false
}


pub fn solve(input: Input) -> Output {
    let mut robots_per_quadrant = [[0; 2]; 2];

    let (width, height) = (101, 103);
    let (middle_x, middle_y) = (width / 2, height / 2);

    let mut robots = Vec::<Robot>::new();

    let time = 100;
    for line in input.lines() {
        let line = line?;

        let (pos, velocity) = line.split_once(' ').unwrap_or_err()?;
        let (mut x, mut y) = parse_pos(&pos[2..])?;
        let (dx, dy) = parse_pos(&velocity[2..])?;

        robots.push(Robot {
            pos: (x, y),
            velocity: (dx, dy)
        });

        x = (x + dx * time).rem_euclid(width);
        y = (y + dy * time).rem_euclid(height);

        let quadrant_x = match x.cmp(&middle_x) {
            Ordering::Less => 0,
            Ordering::Greater => 1,
            Ordering::Equal => continue
        };
        let quadrant_y = match y.cmp(&middle_y) {
            Ordering::Less => 0,
            Ordering::Greater => 1,
            Ordering::Equal => continue
        };

        robots_per_quadrant[quadrant_x][quadrant_y] += 1;
    }

    let mut i = 0;
    let max_depth = 10000;
    let progress = pretty_progress_bar(max_depth as u64);
    loop {
        if check_christmas_tree(robots.as_slice(), width, height) {
            break;
        }
        for robot in robots.iter_mut() {
            let (x, y) = robot.pos;
            let (dx, dy) = robot.velocity;
            robot.pos = (
                (x + dx).rem_euclid(width),
                (y + dy).rem_euclid(height)
            );
        }

        if i >= max_depth {
            progress.finish_and_clear();
            break;
        }
        progress.inc(1);
        i += 1;
    }

    println!("{:?}\n{:?}", robots_per_quadrant[0], robots_per_quadrant[1]);
    let result = robots_per_quadrant[0][0] * robots_per_quadrant[0][1] * robots_per_quadrant[1][0] * robots_per_quadrant[1][1];
    output!(result, i)
}
