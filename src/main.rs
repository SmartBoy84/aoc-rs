mod utils;

use advent::downloader::get_input;
use advent::solutions::*;

fn main() {
    bench(y2020::day1::main, &get_input(2020, 1).unwrap())
}
