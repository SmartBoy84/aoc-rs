#![feature(iter_array_chunks)] // used in 2020:2
#![feature(array_windows)] // 2024:2
#![feature(let_chains)] // 2024:2
#![feature(array_chunks)] // 2024:2

pub mod util {
    pub mod ansi;
    pub mod bitset;
    pub mod grid;
    pub mod hash;
    pub mod heap;
    pub mod integer;
    pub mod iter;
    pub mod math;
    pub mod md5;
    pub mod parse;
    pub mod point;
    pub mod slice;
    pub mod thread;
}

pub mod solutions;
pub mod downloader;
pub mod utils;