use crate::parse_utils;

pub fn main(input: &str) -> (usize, usize) {
    // manual parsing for those precious microseconds!
    let input = input
        .split("\n")
        .map(|a| {
            let mut x = [0i8; 4];
            let mut i = 0;
            let mut a = a.as_bytes().iter();
            loop {
                if let Some(c) = a.next() {
                    if !c.is_ascii_digit() {
                        continue;
                    }
                    let mut num = (c - b'0') as i8;
                    for c in a.by_ref() {
                        if !c.is_ascii_digit() {
                            break;
                        }
                        num = (num * 10) + (c - b'0') as i8;
                    }
                    x[i] = num;
                    i += 1;
                } else {
                    break;
                }
            }
            x
        })
        .collect::<Vec<_>>();
    // don't question the compiler magic - matches is faster than explicit (commented) statements?!
    let part1 = input.iter().filter(|a| matches!((a[0] - a[2]).signum() + (a[1] - a[3]).signum(), -1..=1)).count();
    // let part1 = input.iter().filter(|a| a[0] >= a[2] && a[1] <= a[3] || a[0] <= a[2] && a[1] >= a[3]).count();

    let part2 = input.iter().filter(|a| {matches!((a[0] - a[2]).signum() + (a[0] - a[3]).signum(), -1..=1)|| matches!((a[2] - a[0]).signum() + (a[2] - a[1]).signum(), -1..=1)}).count();
    // let part2 = input.iter().filter(|a| (a[0] >= a[2] && a[0] <= a[3]) || (a[1] >= a[2] && a[1] <= a[3])).count();

    (part1, part2)
}
