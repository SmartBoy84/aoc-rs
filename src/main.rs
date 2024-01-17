pub mod downloader;
use downloader::get_input;

pub mod solutions;
use solutions::*;

pub mod parse_utils;
use parse_utils::*;

fn main() {
    bench(y2022::day5::main, &get_input(5, 2022).unwrap())
}
