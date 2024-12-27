use std::io::BufRead;
use crate::{misc::{grid::Grid, option::OptionExt}, output, Input, Output};


fn direction_from_char(c: u8) -> Option<(isize, isize)> {
    match c {
        b'>' => Some((1, 0)),
        b'v' => Some((0, 1)),
        b'<' => Some((-1, 0)),
        b'^' => Some((0, -1)),
        _ => None
    }
}

fn check_move_big_box(map: &Grid, x: isize, y: isize, dx: isize, dy: isize) -> bool {
    let mut x = x;
    match map.signed_get_or_default(x, y) {
        b'[' => (),
        b']' => x -= 1,
        b'.' => return true,
        _ => return false
    }

    if dy != 0 {
        check_move_big_box(map, x, y + dy, dx, dy)
        && check_move_big_box(map, x + 1, y + dy, dx, dy)
    } else if dx > 0 {
        check_move_big_box(map, x + 1 + dx, y, dx, dy)
    } else {
        check_move_big_box(map, x + dx, y, dx, dy)
    }
}

fn move_big_box(map: &mut Grid, x: isize, y: isize, dx: isize, dy: isize) {
    let mut x = x;
    match map.signed_get_or_default(x, y) {
        b'[' => (),
        b']' => x -= 1,
        _ => return
    }

    if dy != 0 {
        move_big_box(map, x, y + dy, dx, dy);
        move_big_box(map, x + 1, y + dy, dx, dy);
    } else if dx > 0 {
        move_big_box(map, x + 1 + dx, y, dx, dy);
    } else {
        move_big_box(map, x + dx, y, dx, dy);
    }

    let (nx, ny) = (x + dx, y + dy);
    map.signed_set(x, y, b'.');
    map.signed_set(x + 1, y, b'.');
    map.signed_set(nx, ny, b'[');
    map.signed_set(nx + 1, ny, b']');
}

pub fn solve(input: Input) -> Output {
    let mut lines = input.lines();
    let mut map = String::new();
    let mut big_map = String::new();
    for line in lines.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        map.push_str(line.as_str());
        for item in line.bytes() {
            big_map.push_str(match item {
                b'#' => "##",
                b'O' => "[]",
                b'.' => "..",
                b'@' => "@.",
                _ => "??"
            });
        }
        map.push('\n');
        big_map.push('\n');
    }

    let mut map = Grid::from_string(map)?;
    let mut big_map = Grid::from_string(big_map)?;

    let (mut x, mut y, ..) = map.find_signed(b'@').unwrap_or_err()?;
    let (mut bx, mut by) = (x * 2, y);

    for line in lines {
        let line = line?;
        for action in line.as_bytes() {
            let (dx, dy) = direction_from_char(*action).unwrap_or_err()?;
            let (nx, ny) = (x + dx, y + dy);

            if map.signed_get_or_default(nx, ny) == b'.' {
                map.signed_set(nx, ny, b'@');
                map.signed_set(x, y, b'.');
                x = nx;
                y = ny;
            } else {
                let (mut last_x, mut last_y) = (nx, ny);
                while map.signed_get_or_default(last_x, last_y) == b'O' {
                    last_x += dx;
                    last_y += dy;
                }
                if map.signed_get_or_default(last_x, last_y) == b'.' {
                    map.signed_set(x, y, b'.');
                    map.signed_set(nx, ny, b'@');
                    map.signed_set(last_x, last_y, b'O');
                    x = nx;
                    y = ny;
                }
            }

            let (bnx, bny) = (bx + dx, by + dy);

            match big_map.signed_get_or_default(bnx, bny) {
                b'.' => {
                    big_map.signed_set(bnx, bny, b'@');
                    big_map.signed_set(bx, by, b'.');
                    bx = bnx;
                    by = bny;
                },
                b'[' | b']' => if check_move_big_box(&big_map, bnx, bny, dx, dy) {
                    move_big_box(&mut big_map, bnx, bny, dx, dy);
                    big_map.signed_set(bnx, bny, b'@');
                    big_map.signed_set(bx, by, b'.');
                    bx = bnx;
                    by = bny;
                },
                _ => ()
            }
        }
    }


    let mut sum = 0;
    for (x, y, value) in map.iter_signed() {
        if value == b'O' {
            sum += y * 100 + x;
        }
    }
    let mut big_sum = 0;
    for (x, y, value) in big_map.iter_signed() {
        if value == b'[' {
            big_sum += y * 100 + x;
        }
    }

    output!(sum, big_sum)
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    test_solver(solve, indoc::indoc! {"
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "}, output!(10092, 9021));
}
