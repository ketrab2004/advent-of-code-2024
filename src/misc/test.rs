use std::io::{BufReader, Cursor, Read};
use ctor::ctor;
use crate::{Input, Output};


#[ctor]
fn test_setup() {
    color_eyre::install().expect("Failed to install color_eyre");
}

pub fn str_to_input(input: &str) -> Input {
    let cursor = Cursor::new(input.to_owned());

    BufReader::new(Box::new(cursor) as Box<dyn Read>)
}

pub fn test_solver(solver: fn(Input) -> Output, input: &str, expected: Output) {
    let input = str_to_input(input);
    let result = solver(input);

    match expected {
        Err(_) => assert!(result.is_err(), "Expected error, but got {result:?}"),
        Ok(expected) => {
            let result = result.unwrap();

            assert_eq!(result.0.to_string(), expected.0.to_string(), "Part 1");
            assert_eq!(result.1.to_string(), expected.1.to_string(), "Part 2");
        }
    }
}
