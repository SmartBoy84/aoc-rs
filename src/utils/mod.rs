// following utils are from maneatingape's repo - TODO: make my own implementation
pub mod parse;
pub mod integer;

// keeping this for compatibility with old solutions (icb going back and altering them)
pub fn parse_int(input: &str) -> usize {
    input
        .as_bytes()
        .iter()
        .fold(0, |acc, x| (acc * 10) + (*x - b'0') as usize)
}
