use hashbrown::HashMap;

// Euclidean algorthim to find LCM; I don't understand it
fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

pub fn main(input: &str) -> (u64, u64) {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    let lines = nodes.lines().count();
    let mut starting_points = vec![];
    let nodes = nodes
        .split("\n")
        .fold(HashMap::with_capacity(lines), |mut acc, l| {
            let start = &l[0..3];
            if start.ends_with('A') {
                starting_points.push(start);
            }
            acc.insert(start, [&l[7..10], &l[12..15]]);
            acc
        });

    let distances = (0..starting_points.len())
        .zip(starting_points.iter())
        .map(|(_, &a)| {
            let mut instructions = instructions.chars().cycle().zip(0..);
            let mut current = a;
            for (inst, _) in instructions.by_ref() {
                current = nodes.get(current).unwrap()[(inst == 'R') as usize];
                if current.ends_with('Z') {
                    break;
                }
            }
            instructions.next().unwrap().1
        })
        .collect::<Vec<_>>();

    let part1 = distances[starting_points.iter().position(|&x| x == "AAA").unwrap()];
    let part2 = distances.iter().fold(1, |acc, &x| (acc * x) / gcd(acc, x));

    (part1, part2)
}
