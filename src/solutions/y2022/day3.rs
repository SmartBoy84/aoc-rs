fn item_value(a: u8) -> u32 {
    (match a {
        b'a'..=b'z' => a - b'a',
        b'A'..=b'Z' => a - b'A' + 26,
        _ => unreachable!(),
    } as u32)
        + 1
}

fn common_item(groups: &[&[u8]]) -> u8 {
    *groups[0]
        .iter()
        .find(|&c| groups[1..].iter().all(|group| group.contains(c)))
        .unwrap()
}

pub fn main(input: &str) -> (u32, u32) {
    let part1 = input
        .split("\n")
        .map(|a| a.as_bytes().chunks_exact(a.len() / 2).collect())
        .map(|a: Vec<_>| common_item(&a))
        .map(item_value)
        .sum();

    let part2 = input
        .split("\n")
        .map(|a| a.as_bytes())
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .into_iter()
        .map(common_item)
        .map(item_value)
        .sum();

    (part1, part2)
}
