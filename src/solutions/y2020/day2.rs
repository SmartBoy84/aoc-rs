use crate::utils;

pub fn main(input: &str) -> (usize, usize) {
    let input = input
        .split(['-', ':', ' ', '\n'])
        .filter(|s| !s.is_empty())
        .array_chunks::<4>()
        .map(|[start, end, letter, pass]| {
            (
                utils::parse_int(&start),
                utils::parse_int(&end),
                letter.as_bytes()[0],
                pass,
            )
        })
        .collect::<Vec<_>>();

    let part1 = input
        .iter()
        .filter(|(start, end, letter, pass)| {
            (start..=end).contains(&&pass.as_bytes().iter().filter(|c| c == &letter).count())
        })
        .count();

    let part2 = input
        .iter()
        .filter(|(start, end, letter, pass)| {
            let b = pass.as_bytes();
            (&b[*start-1] == letter) ^ (&b[*end-1] == letter)
        })
        .count();
    (part1, part2)
}
