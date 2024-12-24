#![feature(let_chains)] // I need it pretty much everywhere - stabilising in rust 2024 anyways...

// most of the following can be manually implemented (see maneatingape's repo but icb...)
#![feature(iter_array_chunks)] // used in 2020:[1,2] - Do I really need this?

#![feature(array_windows)] // 2024:2
#![feature(array_chunks)] // 2024:2

// pub mod util {
//     pub mod ansi;
//     pub mod bitset;
//     pub mod grid;
//     pub mod hash;
//     pub mod heap;
//     pub mod integer;
//     pub mod iter;
//     pub mod math;
//     pub mod md5;
//     pub mod parse;
//     pub mod point;
//     pub mod slice;
//     pub mod thread;
// }

pub mod solutions;
pub mod downloader;
pub mod utils;

// I use maneatingape's solution when I want to work offline (get the right answer) + to compare speeds
pub mod ape;
