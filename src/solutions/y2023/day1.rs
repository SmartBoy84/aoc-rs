const NUM_MAP: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_sum(input: &[&str], part2: bool) -> usize {
    input
        .iter()
        .map(|line| {
            let mut digits = line
                .as_bytes()
                .iter()
                .enumerate()
                .filter_map(|(index, char)| match char {
                    b'0'..=b'9' => Some((char - b'0') as usize),
                    _ if part2 => NUM_MAP.iter().enumerate().find_map(|(value, name)| {
                        line[index..].starts_with(name).then_some(value + 1)
                    }),
                    _ => None,
                });
                let a = digits.next().unwrap();
                let b = digits.nth_back(0).unwrap_or(a);
                a * 10 + b
        })
        .sum()
}

pub fn main(input: &str) -> (usize, usize) {
    let input: Vec<_> = input.lines().collect();
    (get_sum(&input, false), get_sum(&input, true))
}
