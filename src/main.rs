mod utils;

use advent::downloader::get_input;
use advent::solutions::*;

fn main() {
    bench(y2023::day7::main, &get_input(7, 2023).unwrap())
}
