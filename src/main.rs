mod utils;

use advent::downloader::get_input;
use advent::solutions::*;

fn main() {
    bench(y2023::day12::main, &get_input(12, 2023).unwrap())
}
