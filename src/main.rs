#![feature(array_chunks)]

mod utils;

use advent::downloader::get_input;
use advent::solutions::*;

fn main() {
    bench(y2020::day2::main, &get_input(2020, 2).unwrap())
}
