pub mod downloader;

pub mod solutions;
use solutions::*;

use anyhow::Result;

fn main() -> Result<()> {
    Ok(y2023::day3::main()?)
}
