mod utils;

use advent::downloader::get_input;
use advent::solutions::*;

fn main() {
    bench(y2023::day15::main, &get_input(15, 2023).unwrap())
}
