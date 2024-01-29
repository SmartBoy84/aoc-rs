mod utils;

use advent::downloader::get_input;
use advent::solutions::*;

fn main() {
    bench(y2023::day9::main, &get_input(9, 2023).unwrap())
}
