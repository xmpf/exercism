// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

// Note: To run tests, the nightly toolchain is required
// which can be installed with `rustup toolchain install nightly`.
// The tests can be run with `cargo +nightly test`.

pub fn expected_minutes_in_oven() -> i32 {
    // adding a semicolon ";" at the end of a line
    // transforms that line to a statement
    // omitting the semicolon ";" at the end we say to compiler
    // that this is an expression, so return the value.
    40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    let expected_time = expected_minutes_in_oven();
    if actual_minutes_in_oven > expected_time || actual_minutes_in_oven < 0 {
        panic!("Invalid argument!");
    }
    expected_time - actual_minutes_in_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    2 * number_of_layers
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
