use std::ops::RangeInclusive;

pub fn main(input: &str) -> (usize, usize) {
    // (start, len)

    let input = input.as_bytes();
    let line_len = input.iter().take_while(|&&a| a != b'\n').count() as isize + 1;

    let gear_or_part = input
        .iter()
        .enumerate()
        .filter(|(_, &c)| !matches!(c, b'.' | b'0'..=b'9' | b'\n'))
        .map(|(i, _)| {
            (-1..=1) // up, middle, bottom
                .map(move |a| {
                    (i as isize - (line_len * a) - 1..) // get a range starting from the far left
                        .take(3) // we want the "outline" around the special character
                        .map(|b| b as usize) // uh, idk
                        .skip_while(|&b| !matches!(input.get(b), Some(_))) // account for end/start of text
                        .take_while(|&b| input[b] != b'\n') // account for end of line
                        .filter(|&b| {
                            b != i // remove self character
                    && input[b].is_ascii_digit() // ensure we are in a number
                        })
                        .fold(Vec::new(), |mut acc: Vec<_>, x| {
                            // ensure not part of already found number
                            if let Some(_) = acc
                                .iter()
                                .rev()
                                .find(|c: &&RangeInclusive<_>| c.contains(&x))
                            {
                                return acc;
                            }

                            // find the start of this number
                            let start = x - input[..=x]
                                .iter()
                                .rev()
                                .take_while(|c| c.is_ascii_digit())
                                .enumerate()
                                .map(|(i, _)| i)
                                .last()
                                .unwrap();

                            // find it's total length
                            let len = (start..)
                                .take_while(|&b| matches!(input.get(b), Some(b'0'..=b'9')))
                                .count();

                            acc.push(start..=start + len - 1);
                            acc
                        })
                })
                .flatten()
                .map(|a| a.fold(0, |acc, x| (acc * 10) + (input[x] - b'0') as usize))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // while the above code would cause numbers to be dumplicated if they appear next to multiple "special chars", that doesn't seem to be the case here thankfully
    let part1 = gear_or_part.iter().map(|a| a.iter()).flatten().sum();
    let part2 = gear_or_part
        .iter()
        .filter_map(|a| (a.len() > 1).then_some(a.iter().fold(1, |acc, x| acc * x)))
        .sum();

    (part1, part2)
}
