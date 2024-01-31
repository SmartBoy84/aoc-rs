mod utils;

use advent::downloader::get_input;
use advent::solutions::*;

fn main() {
    bench(y2023::day10::main, &get_input(10, 2023).unwrap())
}
