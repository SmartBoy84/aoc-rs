mod utils;

use advent::downloader::get_input;
use advent::solutions::*;

fn main() {
    bench(y2023::day13::main, &get_input(13, 2023).unwrap())
}
