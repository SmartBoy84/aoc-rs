use advent::downloader::get_input;
use advent::solutions::*;

fn main() {
    bench(y2023::day4::main, &get_input(4, 2023).unwrap())
}
