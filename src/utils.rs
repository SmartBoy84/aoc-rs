use std::{marker::PhantomData, ops::Deref};

pub fn parse_int(input: &str) -> usize {
    input
        .as_bytes()
        .iter()
        .fold(0, |acc, x| (acc * 10) + (*x - b'0') as usize)
}
