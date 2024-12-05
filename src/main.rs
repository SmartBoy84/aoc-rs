mod utils;

use advent::downloader::get_input;
use advent::solutions::*;

fn main() {
    bench(y2024::day3::main, &get_input(2024, 3).unwrap())
}
