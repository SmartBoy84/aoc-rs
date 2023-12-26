use super::*;

const NUM_MAP: [&str; 10] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

fn get_sum(input: &str, part2: bool) -> usize {
    input
        .split("\n")
        .map(|line| {
            let mut digits = line
                .bytes()
                .enumerate()
                .filter_map(|(index, char)| match char {
                    b'0'..=b'9' => Some((char - b'0') as usize),
                    _ if part2 => NUM_MAP.iter().enumerate().find_map(|(value, name)| {
                        line[index..].starts_with(name).then_some(value + 1)
                    }),
                    _ => None,
                });
            let a = digits.next().expect(&format!("Ay yo, no digits?! -> {}", line));
            let b = digits.last().unwrap_or(a); // yuck, actual loop stuff
            a * 10 + b
        })
        .sum()
}

pub fn main() -> Result<()> {
    let input = get_input(1, 2023)?;
    let input = input.trim();

    println!("Part 1: {}", get_sum(input, false));
    println!("Part 2: {}", get_sum(input, true));

    Ok(())
}
