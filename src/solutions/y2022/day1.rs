pub fn main(input: &str) -> (u32, u32) {
    let mut elves: Vec<u32> = input
        .split("\n\n")
        .map(|a| {
            a.split("\n")
                .map(|b| str::parse::<u32>(b).unwrap())
                .sum::<u32>()
        })
        .collect();

    elves.sort_unstable(); // order doesn't matter for x1 == x2 (numbers)

    let part1 = *elves.iter().max().unwrap();
    let part2 = elves.iter().rev().sum();

    (part1, part2)
}
