pub fn main(input: &str) -> (u32, usize) {
    let mut arr = vec![1usize; input.split("\n").count()];
    let mut part1 = 0;

    input
        .split("\n")
        .map(|a| {
            let mut x = a.split("|").map(|b| b.split_whitespace());
            let check = x.next().unwrap().collect::<Vec<_>>();
            let cards = x.next().unwrap();
            cards.filter(move |b| check.contains(b)).count()
        })
        .enumerate()
        .for_each(|(x, won)| {
            part1 += if won != 0 { 1 << (won - 1) } else { 0 }; // no need to do inefficient pow, binary is already base 2!
            for y in (x + 1..).take(won) {
                arr[y] += arr[x];
            }
        });

    (part1, arr.iter().sum())
}
