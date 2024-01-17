use advent::downloader::get_input;
use advent::solutions::*;

fn main() {
    bench(y2022::day5::main, &get_input(5, 2022).unwrap())
}
