use advent::downloader::get_input;
use advent::solutions::*;

fn main() {
    bench(y2023::day3::main, &get_input(3, 2023).unwrap())
}
