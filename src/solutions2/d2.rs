use std::io::BufRead;
use crate::{Input, Output, output};

fn has_repeats(input: &[u8]) -> bool {
    for i in 1..(input.len() / 2 + 1) {
        if input.len() % i != 0 {
            continue;
        }

        let mut all = true;
        for j in 0..(input.len() / i - 1) {
            if input[j * i..(j + 1) * i] != input[(j + 1) * i..(j + 2) * i] {
                all = false;
                break;
            }
        }
        if all {
            return true;
        }
    }
    false
}

pub fn solve(input: Input) -> Output {
    let line: String = input.lines().next().unwrap()?;

    let mut invalid_sum = 0i64;
    let mut invalid_sum2 = 0i64;

    for item in line.split(',') {
        let (start, end) = item.split_once('-').unwrap();

        let range = start.parse::<i64>()?..=end.parse::<i64>()?;

        if start.starts_with("0") {
            invalid_sum += range.start();
            continue;
        } else if end.starts_with("0") {
            invalid_sum += range.end();
            continue;
        }

        for id in range {
            let text = id.to_string();
            if text.len() % 2 == 0 {
                if text[..text.len()/2] == text[text.len()/2..] {
                    invalid_sum += id;
                }
            }

            if has_repeats(text.as_bytes()) {
                invalid_sum2 += id;
            }
        }
    }

    output!(invalid_sum, invalid_sum2)
}


#[test]
fn test() {
    use crate::misc::test::test_solver;

    assert!(has_repeats(b"11"));
    assert!(has_repeats(b"101010"));
    assert!(has_repeats(b"824824824"));
    assert!(!has_repeats(b"1000"));

    test_solver(solve, indoc::indoc! {"
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
    "}, output!(1227775554, 4174379265i64));
}
