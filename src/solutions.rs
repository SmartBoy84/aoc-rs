pub mod y2022;
pub mod y2023;

use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    time,
};

pub fn bench<T: Debug, U: Debug>(runner: fn(&str) -> (T, U), input: &str) {
    let start = time::Instant::now();
    let (x, y) = runner(input);
    let time_taken = start.elapsed();
    println!(
        "Part 1: {:?}\nPart 2: {:?}\nTime taken: {:?}",
        x, y, time_taken
    );
}
