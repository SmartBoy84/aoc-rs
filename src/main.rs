mod utils;

use advent::downloader::get_input;
use advent::solutions::*;

fn main() {
    bench(y2023::day11::main, &get_input(11, 2023).unwrap())
}
