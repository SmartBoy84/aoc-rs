mod utils;

use advent::downloader::get_input;
use advent::solutions::*;

fn main() {
    bench(y2023::day5::main, &get_input(5, 2023).unwrap())
}
