//// Aight, solemn promise - clean day 11 (no sol check)

// pub fn main(input: &str) -> (usize, usize);

pub mod scratch; // to test external solutions

pub mod day1;
pub mod day2;

// keeping this for compatibility with old solutions (icb going back and altering them)
pub fn parse_int(input: &str) -> usize {
    input
        .as_bytes()
        .iter()
        .fold(0, |acc, x| (acc * 10) + (*x - b'0') as usize)
}
