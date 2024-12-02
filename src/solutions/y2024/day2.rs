use crate::utils;

pub fn main(input: &str) -> (usize, usize) {
    let input = input
        .split('\n')
        .map(|r| r.split(' ').map(utils::parse_int).collect::<Vec<_>>())
        .collect::<Vec<_>>();

let part1 = input.iter().filter(|l| l.array_windows::<2>())

    todo!()
}
