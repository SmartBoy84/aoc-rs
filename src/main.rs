pub mod downloader;

pub mod solutions;
use downloader::get_input;
use solutions::*;

use std::{fmt::Debug, time};
// use std::fmt::Debug

fn bench<T: Debug, U: Debug>(runner: fn(&str) -> (T, U), input: &str) {
    let start = time::Instant::now();
    let (x, y) = runner(input);
    let time_taken = start.elapsed();
    println!(
        "Part 1: {:?}\nPart 2: {:?}\nTime taken: {:?}",
        x, y, time_taken
    );
}

fn main() {
    bench(y2022::day1::main, &get_input(2, 2023).unwrap());
}
