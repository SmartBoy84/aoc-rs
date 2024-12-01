use crate::utils;

const TARGET: usize = 2020;

fn find_pair(target: usize, vals: &[usize]) -> Option<usize> {
    let mut markers = [false; TARGET];

    for &i in vals {
        // see, everything done in one iteration!
        let Some(c) = target.checked_sub(i) else {
            continue;
        };
        markers[i] = true;
        if markers[c] == true {
            return Some(i * (c));
        }
    }
    None
}

pub fn main(input: &str) -> (usize, usize) {
    let input = input.split("\n").map(utils::parse_int).collect::<Vec<_>>();

    /*
    Obvious solution is to take each value in array and iterate over every other to determine if pairing found - this is O(n^2)
    We can do better by using the fact that for each value from 0..2020, there is only one other value with which it adds to 2020

    inspired by maneatingape
    */

    let part1 = find_pair(TARGET, &input).unwrap();

    let part2 = input
        .iter()
        .enumerate()
        .find_map(|(i, a)| find_pair(TARGET - a, &input[i..]).map(|p| p * a))
        .unwrap();
    // for (i, a) in input.iter().enumerate() {
    //     if let Some(p) = find_pair(TARGET - a, &input[i..]) {
    //         break;
    //     }
    // }

    // part 2 is identical, except we'll now have to do it in O(2^n)

    // todo!()
    (part1, part2)
}
