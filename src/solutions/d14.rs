use std::io::BufRead;
use color_eyre::eyre::Result;
use crate::{misc::option::OptionExt, output, Input, Output};


fn parse_pos(line: &str) -> Result<(isize, isize)> {
    let (x, y) = line.split_once(',').unwrap_or_err()?;
    let x = x.parse()?;
    let y = y.parse()?;
    Ok((x, y))
}

pub fn solve(input: Input) -> Output {
    let mut robot_per_quadrant = [[0; 2]; 2];

    let (width, height) = (101, 103);
    let (middle_x, middle_y) = (width / 2, height / 2);
    let time = 100;
    for line in input.lines() {
        let line = line?;

        let (pos, velocity) = line.split_once(' ').unwrap_or_err()?;
        let (mut x, mut y) = parse_pos(&pos[2..])?;
        let (dx, dy) = parse_pos(&velocity[2..])?;

        x = (x + dx * time).rem_euclid(width);
        y = (y + dy * time).rem_euclid(height);

        let quadrant_x = if x < middle_x { 0 }
            else if x > middle_x { 1 }
            else { continue; };
        let quadrant_y = if y < middle_y { 0 }
            else if y > middle_y { 1 }
            else { continue; };

        robot_per_quadrant[quadrant_x][quadrant_y] += 1;
    }

    println!("{:?}\n{:?}", robot_per_quadrant[0], robot_per_quadrant[1]);
    let result = robot_per_quadrant[0][0] * robot_per_quadrant[0][1] * robot_per_quadrant[1][0] * robot_per_quadrant[1][1];
    output!(result)
}
