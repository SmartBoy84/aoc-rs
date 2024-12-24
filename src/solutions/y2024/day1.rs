use hashbrown::HashMap;

use crate::utils::parse::ParseOps;

pub fn main(input: &str) -> (usize, usize) {
    let (mut r, mut l): (Vec<_>, Vec<_>) = input
        .iter_unsigned::<usize>()
        .array_chunks::<2>()
        .map(|[a, b]| (a, b))
        .unzip();

    // man this is infuriating - this section takes up like 155us

    r.sort_unstable();
    l.sort_unstable();

    let part1: usize = r.iter().zip(l.iter()).map(|(a, b)| a.abs_diff(*b)).sum();

    // solution 1
    // // let mut r = r.into_iter();
    // // let mut l = l.into_iter();

    // // let mut part2 = 0;

    // // while let Some(r_i) = r.next() {
    // //     let r_n = r.take_while_ref(|&b| r_i == b).count();
    // //     let l_n = l.take_while_ref(|&b| r_i == b).count();
    // //     part2 += (r_n + 1) * l_n * r_i;
    // // }

    // solution 2
    // // my first solution is above - it's much, much slower than hashmaps?!!
    let mut freq = HashMap::with_capacity(1000);
    for r_n in r {
        *freq.entry(r_n).or_insert(0) += 1;
    }
    let part2 = l.into_iter().map(|l| freq.get(&l).unwrap_or(&0) * l).sum();

    // solution 3 - maneatingape's correction of my first approach
    // turns out it was slower than the hashmap method because of unintended issues with `take_while_ref`
    // let mut part2 = 0;
    // let mut count1 = 1;
    // let mut count2 = 0;
    // let mut i = 0;
    // let mut j = 0;

    // while i < l.len() {
    //     let number = l[i];
    //     i += 1;

    //     while i < l.len() && number == l[i] {
    //         count1 += 1;
    //         i += 1;
    //     }

    //     while j < r.len() && number >= r[j] {
    //         count2 += (number == r[j]) as u32;
    //         j += 1;
    //     }

    //     part2 += number * count1 * count2 as usize;
    //     count1 = 1;
    //     count2 = 0;
    // }

    (part1, part2)

    // let i = parse(input);
    // (part1(&i) as usize, part2(&i) as usize)
}
