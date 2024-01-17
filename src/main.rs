use advent::downloader::get_input;
use advent::solutions::*;

fn main() {
    bench(y2022::day7::main, &get_input(7, 2022).unwrap())
}
