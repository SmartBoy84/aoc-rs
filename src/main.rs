mod utils;

use advent::downloader::get_input;
use advent::solutions::*;

fn main() {
    bench(y2023::day8::main, &get_input(8, 2023).unwrap())
}
