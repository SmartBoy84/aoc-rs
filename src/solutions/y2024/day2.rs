use std::cmp::Ordering;

use crate::utils;

pub fn main(input: &str) -> (usize, usize) {
    let input = input
        .split('\n')
        .map(|r| r.split(' ').map(utils::parse_int).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let part1 = input.iter().filter(|l| {
        let mut ord = None;
        for &[a, b] in l.array_windows::<2>() {
            let ord_n = a.cmp(&b);
            if ord_n != Ordering::Equal {
                if let Some(o) = ord {
                    if ord_n != o {
                        return false;
                    }
                } else {
                    ord = Some(ord_n)
                }
            }
            if !(1..=3).contains(&a.abs_diff(b)) {
                return false;
            }
        }
        return true;
    }).count();

    (part1, 1)
}
